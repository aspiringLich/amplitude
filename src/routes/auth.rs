use super::*;

use chrono::Utc;
use entity::{google_user, sea_orm_active_enums::Account, user};
use google_oauth::AsyncClient;
use uuid::{NoContext, Timestamp, Uuid};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/google", post(google_login))
        .route("/session", get(session))
}

#[derive(Deserialize)]
struct GoogleCredentials {
    credentials: String,
}

async fn google_login(
    session: Session,
    State(state): State<AppState>,
    Json(creds): Json<GoogleCredentials>,
) -> Result<Response<Body>, Error> {
    let auth = &state.secrets.google_auth;
    let client = AsyncClient::new(&auth.client_id).timeout(Duration::from_secs(5));
    let payload = client
        .validate_id_token(&creds.credentials)
        .await
        .context("Failed to validate Google ID token")
        .map_err(unauthorized)?;

    match google_user::Model::get(&state.db, &payload.sub).await? {
        // google user found; find related user
        Some(guser) => {
            let user = guser.find_related_user(&state.db).await?;
            login(&state, session, &user, StatusCode::OK).await
        }
        // no user found; create one
        None => {
            let now = Utc::now();
            let secs = now.timestamp() as u64;
            let nanos = now.timestamp_subsec_nanos();
            let uuid = Uuid::new_v7(Timestamp::from_unix(NoContext, secs, nanos));

            let first = payload.given_name.unwrap_or("FIRSTNAME".to_string());
            let last = payload.family_name.unwrap_or("LASTNAME".to_string());
            let name = format!("{} {}", first, last);

            google_user::Model {
                user_id: uuid,
                google_id: payload.sub,
            }
            .insert(&state.db)
            .await?;

            let user = user::Model {
                user_id: uuid,
                account: Account::User,
                name: name.clone(),
                avatar_url: payload.picture.clone(),
                created: now.naive_local(),
            };
            let res = login(&state, session, &user, StatusCode::CREATED).await?;
            user.insert(&state.db).await?;

            Ok(res)
        }
    }
}

async fn session(
    mut session: Session,
    State(state): State<AppState>,
) -> Result<Json<UserAvatar>, Error> {
    match session.get(&state.db).await? {
        Some(s) => {
            session.update_expiration(&state).await?;
            Ok(Json(UserAvatar::new(&s)))
        }
        None => Err(not_found("Session not found")),
    }
}
