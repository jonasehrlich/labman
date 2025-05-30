use crate::core;
use axum::routing;
use std::sync::Arc;

pub fn router() -> routing::Router<Arc<core::Labman>> {
    routing::Router::new().nest("/v1", v1::router())
}

pub mod v1 {
    use crate::core;
    use axum::{Json, extract::State, http, routing};
    use std::sync::Arc;

    pub fn router() -> routing::Router<Arc<core::Labman>> {
        routing::Router::new()
            .route("/users", routing::get(list_users).post(create_user))
            .route("/users/{id}", routing::get(get_user).delete(delete_user))
    }

    async fn list_users(
        State(labman): State<Arc<core::Labman>>,
    ) -> Result<Json<Vec<core::models::User>>, (http::StatusCode, String)> {
        let users = labman
            .user()
            .iter(&core::models::UserRole::min())
            .await
            .map_err(crate::web::internal_error)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(crate::web::internal_error)?;
        Ok(Json(users))
    }

    async fn get_user(
        State(_labman): State<Arc<core::Labman>>,
    ) -> Result<Json<String>, http::StatusCode> {
        // Placeholder for actual user retrieval logic
        Ok(Json("alice".to_string()))
    }

    async fn create_user(
        State(_labman): State<Arc<core::Labman>>,
        Json(user): Json<String>,
    ) -> Result<http::StatusCode, http::StatusCode> {
        // Placeholder for actual user creation logic
        println!("Creating user: {}", user);
        Ok(http::StatusCode::CREATED)
    }

    async fn delete_user(
        State(_labman): State<Arc<core::Labman>>,
    ) -> Result<http::StatusCode, http::StatusCode> {
        // Placeholder for actual user deletion logic
        println!("Deleting user");
        Ok(http::StatusCode::NO_CONTENT)
    }
}
