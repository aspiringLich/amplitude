use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use eyre::Context;

use crate::views::{
    auth::{login, UserAvatar},
    bad_request, forbidden, internal, not_found,
    session::Session,
    unauthorized, Error,
};

pub mod auth;
pub mod exec;

pub type AppState = std::sync::Arc<crate::AppState>;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/exec", exec::routes())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::cors::CorsLayer::very_permissive())
}
