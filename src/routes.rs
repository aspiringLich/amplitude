use axum::{
    body::Body, extract::State, http::{Method, Request, Result, StatusCode, Uri}, response::{IntoResponse, Response}, routing::{delete, get, post}, Json, Router
};
use std::time::Duration;
use tower_http::cors;

type AppState = std::sync::Arc<super::app::AppState>;

mod auth;

struct RequestUri(Uri);

fn on_request(request: &Request<Body>, _span: &tracing::Span) {
    tracing::debug!("{:?}", request.uri());
}

fn on_response(response: &Response<Body>, _latency: Duration, _span: &tracing::Span) {
    tracing::debug!("{:?}", response.status());
}

fn on_failure(
    error: tower_http::classify::ServerErrorsFailureClass,
    _latency: Duration,
    _span: &tracing::Span,
) {
    tracing::error!("{:?}", error);
}

pub fn routes() -> Router<AppState> {
    let cors = cors::CorsLayer::very_permissive();

    Router::new()
        .nest("/auth", auth::routes())
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .on_request(on_request)
                .on_response(on_response)
                .on_failure(on_failure),
        )
        .layer(cors)
}
