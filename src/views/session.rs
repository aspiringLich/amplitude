use axum::extract::FromRequestParts;
use axum_extra::extract::{cookie::Cookie, CookieJar};
use chrono::{NaiveDateTime, Utc};
use entity::{session, user};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, IntoActiveModel, Set};
use uuid::{NoContext, Timestamp, Uuid};

use crate::app::AppState;

use super::internal;

pub struct Session {
    session_id: Option<Uuid>,
    jar: CookieJar,
    session: Option<session::Model>,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Session {
    type Rejection = super::Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, &())
            .await
            .map_err(internal)?;
        let session_id = jar
            .get("session")
            .and_then(|s| s.value().parse::<Uuid>().ok());
        Ok(Self {
            session_id,
            jar,
            session: None,
        })
    }
}

fn expiration(state: &AppState) -> NaiveDateTime {
    Utc::now().naive_utc() + state.config.session.expiration
}

impl Session {
    async fn session(&mut self, db: &DatabaseConnection) -> Result<Option<session::Model>, DbErr> {
        match self.session.as_ref() {
            Some(s) => Ok(Some(s.to_owned())),
            None => {
                let Some(session_id) = self.session_id else {
                    return Ok(None);
                };
                let s = session::Model::get(db, session_id).await?;
                self.session = s.clone();
                Ok(s)
            }
        }
    }

    pub async fn get(&mut self, db: &DatabaseConnection) -> Result<Option<user::Model>, DbErr> {
        match self.session(db).await? {
            Some(session) => Ok(Some(session.find_related_user(db).await?)),
            None => Ok(None),
        }
    }

    #[must_use]
    pub async fn add(self, state: &AppState, user_id: Uuid) -> Result<CookieJar, super::Error> {
        let now = Utc::now();
        let secs = now.timestamp() as u64;
        let nanos = now.timestamp_subsec_nanos();
        let session_id = Uuid::new_v7(Timestamp::from_unix(NoContext, secs, nanos));

        let expiration = expiration(state);
        tracing::trace!(id = ?session_id, expiration = ?expiration, "Adding session");
        session::Model {
            session_id,
            user_id,
            expiration,
        }
        .insert(&state.db)
        .await?;

        let mut cookie = Cookie::new("session", session_id.as_simple().to_string());
        cookie.make_permanent();
        cookie.set_path("/");

        Ok(self.jar.add(cookie))
    }

    #[must_use]
    pub async fn get_or_add(
        mut self,
        state: &AppState,
        user_id: Uuid,
    ) -> Result<CookieJar, super::Error> {
        match self.get(&state.db).await? {
            Some(_) => Ok(self.jar),
            None => self.add(state, user_id).await,
        }
    }

    pub async fn update_expiration(&mut self, state: &AppState) -> Result<(), DbErr> {
        let Some(session) = self.session(&state.db).await? else {
            return Ok(());
        };
        let mut active = session.into_active_model();
        let new_expiration = expiration(state);
        tracing::trace!("Updated session expiration: {new_expiration}");

        active.expiration = Set(new_expiration);
        active.update(&state.db).await?;
        Ok(())
    }
}
