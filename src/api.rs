use std::sync::Arc;

use axum::{
    // routing::{get, post},
    Router,
};

use crate::app::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
}
