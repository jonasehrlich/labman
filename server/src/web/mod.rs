use axum::routing;

mod api;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", routing::get(handler1))
        .nest("/api", api::router())
}

async fn handler1() -> &'static str {
    "Hello from router 1"
}
