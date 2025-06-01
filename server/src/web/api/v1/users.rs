use crate::{core, web::api, web::internal_error, web::not_found};
use axum::extract::{Path, State};
use axum::{Json, http, routing};
use serde::Deserialize;
use std::sync::Arc;

pub fn router() -> routing::Router<Arc<core::Labman>> {
    routing::Router::new()
        .route("/users", routing::get(list_users).post(create_user))
        .route("/users/{id}", routing::get(get_user).delete(delete_user))
}

#[utoipa::path(
    get,
    path = "/users",
    description = "List users",
    responses(
        (status = OK, description = "List users", body = [core::entity::user::Model]),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = api::StatusJsonResponse),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized", body = api::StatusJsonResponse)
    )
)]
async fn list_users(
    State(labman): State<Arc<core::Labman>>,
) -> Result<Json<Vec<core::entity::user::Model>>, api::HttpStatus> {
    // TODO: Check if requesting user is authorized

    let users = labman
        .user()
        .list(&core::entity::user::UserRole::min())
        .await
        .map_err(internal_error)?;
    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    description = "Get a single user",
    responses(
        (status = OK, description = "Success", body = core::entity::user::Model),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = api::StatusJsonResponse),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized", body = api::StatusJsonResponse),
        (status = http::StatusCode::NOT_FOUND, description = "User not found", body = api::StatusJsonResponse)
    )
)]
async fn get_user(
    State(labman): State<Arc<core::Labman>>,
    Path(id): Path<i32>,
) -> Result<Json<core::entity::user::Model>, api::HttpStatus> {
    // TODO: Check if requesting user is authorized
    match labman.user().get_by_id(id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(api::HttpStatus(not_found(anyhow::Error::msg(
            "User not found",
        )))),
        Err(e) => Err(api::HttpStatus(internal_error(e))),
    }
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateUser {
    pub name: String,
    pub role: core::entity::user::UserRole,
}

#[utoipa::path(
    post,
    path = "/users",
    description = "Create a user",
    responses(
        (status = OK, description = "Success", body = core::entity::user::Model),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = api::StatusJsonResponse),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized", body = api::StatusJsonResponse),
    )
)]
async fn create_user(
    State(labman): State<Arc<core::Labman>>,
    Json(new_user): Json<CreateUser>,
) -> Result<(http::StatusCode, Json<core::entity::user::Model>), api::HttpStatus> {
    // TODO: Check if requesting user is Administrator
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
        (status = OK, description = "Success", body = api::StatusJsonResponse),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = api::StatusJsonResponse),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized", body = api::StatusJsonResponse)
    )
)]
async fn delete_user(
    State(labman): State<Arc<core::Labman>>,
    Path(id): Path<u32>,
) -> Result<api::HttpStatus, api::HttpStatus> {
    // TODO: Check if requesting user is Administrator to delete users
    labman
        .user()
        .delete(id as i32)
        .await
        .map_err(internal_error)?;
    Ok(api::HttpStatus(http::StatusCode::OK))
}
