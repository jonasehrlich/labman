use crate::core;
use axum::{http, routing};
use std::sync::Arc;

mod api;

/// Utility function for mapping boxed errors into a `500 Internal Server Error`
/// response.
fn internal_error(err: Box<dyn std::error::Error>) -> http::StatusCode {
    eprintln!("Internal server error: {}", err);
    http::StatusCode::INTERNAL_SERVER_ERROR
}

pub fn router() -> routing::Router<Arc<core::Labman>> {
    routing::Router::new()
        .route("/", routing::get(handler1))
        .nest("/api", api::router())
}

async fn handler1() -> &'static str {
    "Hello from router 1"
}
