use crate::core;
use axum::routing;
use axum::{Json, http, response};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

pub fn router() -> routing::Router<Arc<core::Labman>> {
    routing::Router::new().nest("/v1", v1::router())
}

/// Represents a JSON response for HTTP status codes containing the canonical reason for the status.
#[derive(Serialize, ToSchema)]
struct StatusJsonResponse {
    status: String,
}

/// Represents an HTTP status code that can be converted into a response.
struct HttpStatus(http::StatusCode);

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

pub mod v1 {
    use crate::{core, web::internal_error, web::not_found};
    use axum::extract::{Path, State};
    use axum::{Json, http, routing};
    use std::sync::Arc;

    #[derive(utoipa::OpenApi)]
    #[openapi(paths(list_users, create_user, get_user, delete_user))]
    pub struct OpenApiDoc;

    pub fn router() -> routing::Router<Arc<core::Labman>> {
        routing::Router::new()
            .route("/users", routing::get(list_users).post(create_user))
            .route("/users/{id}", routing::get(get_user).delete(delete_user))
    }

    #[utoipa::path(
    get,
    path = "/users",
    description = "List all users",
    responses(
        (status = OK, description = "List users", body = [core::models::User]),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = super::StatusJsonResponse),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized", body = super::StatusJsonResponse)
    )
)]
    async fn list_users(
        State(labman): State<Arc<core::Labman>>,
    ) -> Result<Json<Vec<core::models::User>>, super::HttpStatus> {
        // TODO: Check if requesting user is authorized to list users

        let users = labman
            .user()
            .iter(&core::models::UserRole::min())
            .await
            .map_err(crate::web::internal_error)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(crate::web::internal_error)?;
        Ok(Json(users))
    }

    #[utoipa::path(
    get,
    path = "/users/{id}",
    description = "Get a single user",
    responses(
        (status = OK, description = "Success", body = core::models::User),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = super::StatusJsonResponse),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized", body = super::StatusJsonResponse),
        (status = http::StatusCode::NOT_FOUND, description = "User not found", body = super::StatusJsonResponse)
    )
)]
    async fn get_user(
        State(labman): State<Arc<core::Labman>>,
        Path(id): Path<u32>,
    ) -> Result<Json<core::models::User>, super::HttpStatus> {
        // TODO: Check if requesting user is authorized to get users
        let user = labman.user().get_by_id(id).await.map_err(not_found)?;
        Ok(Json(user))
    }

    #[utoipa::path(
    post,
    path = "/users",

    description = "Create a user",
    responses(
        (status = OK, description = "Success", body = core::models::User),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = super::StatusJsonResponse),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized", body = super::StatusJsonResponse),
    )
)]
    async fn create_user(
        State(labman): State<Arc<core::Labman>>,
        Json(new_user): Json<core::models::NewUser>,
    ) -> Result<(http::StatusCode, Json<core::models::User>), super::HttpStatus> {
        // TODO: Check if requesting user is authorized to create users
        let user = labman
            .user()
            .create(&new_user.name, &new_user.role)
            .await
            .map_err(internal_error)?;

        Ok((http::StatusCode::CREATED, Json(user)))
    }

    #[utoipa::path(
    delete,
    path = "/users/{id}",
    description = "Delete a user",
    responses(
        (status = OK, description = "Success", body = super::StatusJsonResponse),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = super::StatusJsonResponse),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized", body = super::StatusJsonResponse)
    )
)]
    async fn delete_user(
        State(_labman): State<Arc<core::Labman>>,
    ) -> Result<http::StatusCode, super::HttpStatus> {
        // TODO: Check if requesting user is authorized to delete users

        // Placeholder for actual user deletion logic
        println!("Deleting user");
        Ok(http::StatusCode::NO_CONTENT)
    }
}
