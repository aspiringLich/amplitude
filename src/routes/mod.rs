use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use std::time::Duration;
use tower_http::cors;

type AppState = std::sync::Arc<super::app::AppState>;

mod auth;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::routes())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(cors::CorsLayer::very_permissive())
}
