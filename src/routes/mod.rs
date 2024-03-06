use axum::{
    extract::{FromRequestParts, State},
    routing::post,
    Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use chrono::Utc;
use entity::{session, user};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use std::{cell::Cell, time::Duration};
use tower_http::cors;
use uuid::{NoContext, Timestamp, Uuid};

use crate::views::{self, internal};

type AppState = std::sync::Arc<super::app::AppState>;

mod auth;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::routes())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(cors::CorsLayer::very_permissive())
}

pub struct Session {
    session_id: Option<Uuid>,
    jar: CookieJar,
}

#[axum::async_trait]
impl FromRequestParts<AppState> for Session {
    type Rejection = views::Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
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
    pub fn get(&self, db: &DatabaseConnection) -> Option<user::Model> {
        todo!()
    }

    #[must_use]
    pub async fn add(
        self,
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<CookieJar, views::Error> {
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
}
