use axum::extract::FromRequestParts;
use axum_extra::extract::{cookie::Cookie, CookieJar};
use chrono::Utc;
use entity::{session, user};
use sea_orm::{DatabaseConnection, DbErr};
use std::time::Duration;
use uuid::{NoContext, Timestamp, Uuid};

use super::internal;

pub struct Session {
    session_id: Option<Uuid>,
    jar: CookieJar,
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
        Ok(Self { session_id, jar })
    }
}

impl Session {
    pub async fn get(&self, db: &DatabaseConnection) -> Result<Option<user::Model>, DbErr> {
        let Some(session_id) = self.session_id else {
            return Ok(None);
        };
        let session = session::Model::get(db, session_id).await?;
        match session {
            Some(session) => Ok(Some(session.find_related_user(db).await?)),
            None => Ok(None),
        }
    }

    #[must_use]
    pub async fn add(
        self,
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<CookieJar, super::Error> {
        let now = Utc::now();
        let secs = now.timestamp() as u64;
        let nanos = now.timestamp_subsec_nanos();
        let session_id = Uuid::new_v7(Timestamp::from_unix(NoContext, secs, nanos));

        tracing::trace!("Adding session: {}", session_id);
        session::Model {
            session_id,
            user_id,
            expiration: now.naive_local() + Duration::from_secs(60 * 60 * 24 * 7),
        }
        .insert(db)
        .await?;

        let mut cookie = Cookie::new("session", session_id.as_simple().to_string());
        cookie.make_permanent();

        Ok(self.jar.add(cookie))
    }

    #[must_use]
    pub async fn get_or_add(
        self,
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<CookieJar, super::Error> {
        match self.get(db).await? {
            Some(_) => Ok(self.jar),
            None => self.add(db, user_id).await,
        }
    }
}
