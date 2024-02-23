use std::default::Default;

use super::*;
use entity::google_user::Entity as GoogleUser;
use entity::user::Entity as User;
use google_oauth::AsyncClient;
use sea_orm::EntityTrait;

#[derive(serde::Deserialize)]
struct GoogleCredentials {
    credentials: String,
}

async fn google_login(
    State(state): State<AppState>,
    Json(creds): Json<GoogleCredentials>,
) -> impl IntoResponse {
    let auth = &state.secrets.google_auth;
    let client = AsyncClient::new(&auth.client_id).timeout(Duration::from_secs(5));
    let payload = match client.validate_id_token(&creds.credentials).await {
        Ok(payload) => payload,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };
    dbg!(&payload);

    match GoogleUser::find_by_id(&payload.aud).one(&state.db).await {
        Ok(Some(user)) => {
            dbg!(&user);
            return StatusCode::OK.into_response();
        }
        Ok(None) => {
            return StatusCode::CREATED.into_response();
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/google", post(google_login))
}
