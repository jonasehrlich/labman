use crate::core;
use axum::routing;
use axum::{Json, http, response};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

/// Get the router for the API.
pub(super) fn router() -> routing::Router<Arc<core::Labman>> {
    routing::Router::new().nest("/v1", v1::router())
}

/// JSON response for HTTP containing the canonical reason for the status.
#[derive(Serialize, ToSchema)]
struct StatusJsonResponse {
    status: String,
}

/// Represents an HTTP status code that can be converted into a response.
struct HttpStatus(http::StatusCode);

/// Implementing the `From` trait to convert `http::StatusCode` into `HttpStatus`.
impl From<http::StatusCode> for HttpStatus {
    fn from(status: http::StatusCode) -> Self {
        HttpStatus(status)
    }
}

impl response::IntoResponse for HttpStatus {
    fn into_response(self) -> response::Response {
        let error_response = StatusJsonResponse {
            status: self.0.canonical_reason().unwrap_or("Unknown").to_string(),
        };

        (self.0, Json(error_response)).into_response()
    }
}

#[derive(utoipa::OpenApi)]
#[openapi(
        nest(
            (path = "/v1", api = v1::ApiDoc)
        )
    )]
pub struct ApiDoc;

pub mod v1 {
    use crate::core;
    use axum::routing;
    use std::sync::Arc;

    mod users;

    /// API documentation for the v1 endpoints.
    #[derive(utoipa::OpenApi)]
    #[openapi(paths(
        users::list_users,
        users::create_user,
        users::get_user,
        users::delete_user
    ))]
    pub(super) struct ApiDoc;

    /// Get the router for the v1 API.
    pub(super) fn router() -> routing::Router<Arc<core::Labman>> {
        routing::Router::new().merge(users::router())
    }
}
