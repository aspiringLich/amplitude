use super::*;
use entity::google_user;
use google_oauth::AsyncClient;
use crate::error::{IntoStatusResult, StatusContext, StatusResult};

#[derive(serde::Deserialize)]
struct GoogleCredentials {
    credentials: String,
}

async fn google_login(
    State(state): State<AppState>,
    Json(creds): Json<GoogleCredentials>,
) -> StatusResult<Response> {
    let auth = &state.secrets.google_auth;
    let client = AsyncClient::new(&auth.client_id).timeout(Duration::from_secs(5));
    let payload = client
        .validate_id_token(&creds.credentials)
        .await
        .err_context(StatusCode::UNAUTHORIZED, "Invalid Google ID Token")?;
    dbg!(&payload);

    match google_user::Entity::find_by_id(&payload.aud).one(&state.db).await {
        Ok(Some(user)) => {
            dbg!(&user);
            return StatusCode::OK.status_result();
        }
        Ok(None) => {
            return StatusCode::CREATED.status_result();
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).status_result();
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/google", post(google_login))
}
