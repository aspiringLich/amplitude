use super::*;
use crate::views::{
    auth::{user_avatar, UserAvatar},
    internal, unauthorized, Error,
};

use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, HeaderName, Response, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use chrono::Utc;
use entity::{google_user, sea_orm_active_enums::Account, user};
use eyre::Context;
use google_oauth::AsyncClient;
use uuid::{NoContext, Timestamp, Uuid};

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
        Some(user) => Ok(user_avatar(user.find_related_user(&state.db).await?).into_response()),
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

            user::Model {
                user_id: uuid,
                account: Account::User,
                name: name.clone(),
                avatar_url: payload.picture.clone(),
                created: now.naive_local(),
            }
            .insert(&state.db)
            .await?;

            let user_avatar = UserAvatar {
                name,
                avatar_url: payload.picture,
            };
            let res = (
                StatusCode::CREATED,
                session.add(&state.db, uuid).await?,
                AppendHeaders([(CONTENT_TYPE, "application/json")]),
                serde_json::to_string(&user_avatar).map_err(internal)?,
            )
                .into_response();
            Ok(res)
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/google", post(google_login))
}
