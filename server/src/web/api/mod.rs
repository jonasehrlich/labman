use axum::Router;

pub fn router() -> Router {
    Router::new().nest("/v1", v1::router())
}

pub mod v1 {
    use axum::{Json, Router, http::StatusCode, routing};

    pub fn router() -> Router {
        Router::new()
            .route("/users", routing::get(get_users).post(create_user))
            .route("/users/{id}", routing::get(get_user).delete(delete_user))
    }

    async fn get_users() -> Json<Vec<String>> {
        // Placeholder for actual user retrieval logic
        Json(vec!["alice".to_string(), "bob".to_string()])
    }
    async fn get_user() -> Result<Json<String>, StatusCode> {
        // Placeholder for actual user retrieval logic
        Ok(Json("alice".to_string()))
    }
    async fn create_user(Json(user): Json<String>) -> Result<StatusCode, StatusCode> {
        // Placeholder for actual user creation logic
        println!("Creating user: {}", user);
        Ok(StatusCode::CREATED)
    }
    async fn delete_user() -> Result<StatusCode, StatusCode> {
        // Placeholder for actual user deletion logic
        println!("Deleting user");
        Ok(StatusCode::NO_CONTENT)
    }
}
