use crate::core;
use axum::{http, routing};
use std::sync::Arc;

pub use app::App;

mod api;
mod app;

/// Utility function for mapping errors into a `500 Internal Server Error`
/// response.
fn internal_error(err: anyhow::Error) -> http::StatusCode {
    eprintln!("Internal server error: {}", err);
    http::StatusCode::INTERNAL_SERVER_ERROR
}

fn not_found(err: anyhow::Error) -> http::StatusCode {
    eprintln!("Not found: {}", err);
    http::StatusCode::NOT_FOUND
}

fn router() -> routing::Router<Arc<core::Labman>> {
    routing::Router::new()
        .route("/", routing::get(handler1))
        .nest("/api", api::router())
}

async fn handler1() -> &'static str {
    "Hello from router 1"
}

#[derive(utoipa::OpenApi)]
#[openapi(
        nest(
            (path = "/api", api = api::ApiDoc)
        )
    )]
struct ApiDoc;
