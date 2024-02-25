use super::*;
use crate::error::{IntoStatusResult, StatusContext, StatusResult};
use chrono::Utc;
use entity::google_user;
use entity::sea_orm_active_enums::Account;
use entity::user;
use google_oauth::AsyncClient;
use sea_orm::ModelTrait;
use uuid::{NoContext, Timestamp, Uuid};

#[derive(Deserialize)]
struct GoogleCredentials {
    credentials: String,
}

#[derive(Serialize)]
struct UserAvatar {
    name: String,
    avatar_url: Option<String>,
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
        .err_response(StatusCode::UNAUTHORIZED, "Invalid Google ID Token")?;

    match google_user::Entity::find_by_id(&payload.sub)
        .one(&state.db)
        .await
    {
        Ok(Some(user)) => match user.find_related(user::Entity).one(&state.db).await {
            Ok(Some(user)) => {
                return Json(UserAvatar {
                    name: user.name,
                    avatar_url: user.avatar_url,
                })
                .status_result();
            }
            Ok(None) => unreachable!(),
            Err(e) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).status_result();
            }
        },
        Ok(None) => {
            let now = Utc::now();
            let secs = now.timestamp() as u64;
            let nanos = now.timestamp_subsec_nanos();
            let uuid = Uuid::new_v7(Timestamp::from_unix(NoContext, secs, nanos));

            let first = payload.given_name.unwrap_or("FIRSTNAME".to_string());
            let last = payload.family_name.unwrap_or("LASTNAME".to_string());
            let name = format!("{} {}", first, last);

            google_user::Entity::insert(google_user::ActiveModel {
                user_id: Set(uuid),
                google_id: Set(payload.sub),
            })
            .do_nothing()
            .exec(&state.db)
            .await
            .err_context("Failed to create Google user")?;
            user::Entity::insert(user::ActiveModel {
                user_id: Set(uuid),
                account: Set(Account::User),
                name: Set(name.clone()),
                avatar_url: Set(payload.picture.clone()),
                created: Set(now.naive_local()),
            })
            .do_nothing()
            .exec(&state.db)
            .await
            .err_context("Failed to create user")?;

            return (
                StatusCode::CREATED,
                Json(UserAvatar {
                    name,
                    avatar_url: payload.picture,
                }),
            )
                .status_result();
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).status_result();
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/google", post(google_login))
}
