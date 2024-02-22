use std::sync::Arc;

use axum::{
    response::Response, routing::{get, post}, Router
};

use crate::app::AppState;

async fn string_handler() -> String {
    "Hello, World!".to_string()
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/",  get(string_handler))
}
