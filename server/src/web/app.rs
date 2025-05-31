use crate::{core, web};
use std::sync::Arc;
use utoipa::OpenApi;

/// Main application structure that holds the Labman instance.
pub struct App {
    labman: Arc<core::Labman>,
}

impl App {
    pub async fn new(labman: Arc<core::Labman>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { labman })
    }

    pub async fn serve(self, host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let app = web::router().with_state(self.labman).merge(
            utoipa_rapidoc::RapiDoc::with_openapi("/api-docs/openapi.json", web::ApiDoc::openapi())
                .path("/rapidoc"),
        );

        // TODO: Check if this would also work with IPv6 addresses
        let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
            .await
            .unwrap();
        println!("Server running on http://{host}:{port}");
        axum::serve(listener, app)
            // .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
            .await?;

        // deletion_task.await??;
        Ok(())
    }
}
