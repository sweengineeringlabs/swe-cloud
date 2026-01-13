//! HTTP Router

use crate::Emulator;
use axum::{
    Router,
    routing::{any, get},
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

/// Create the main router
pub fn create_router(emulator: Arc<Emulator>) -> Router {
    let mut router = Router::new()
        // Health check endpoints
        .route("/health", get(health_check))
        .route("/_localstack/health", get(health_check)) // LocalStack compat
        .route("/", axum::routing::post(super::dispatcher::dispatch));

    // S3 routes
    #[cfg(feature = "s3")]
    {
        router = router
            // List buckets (root path, GET only)
            .route("/", get(crate::services::s3::handlers::list_buckets))
            // Bucket operations
            .route("/:bucket", any(crate::services::s3::handlers::bucket_handler))
            // Object operations  
            .route("/:bucket/*key", any(crate::services::s3::handlers::object_handler));
    }

    // Lambda routes
    #[cfg(feature = "lambda")]
    {
        router = router
            .route("/2015-03-31/functions/:function_name/invocations", any(crate::services::lambda::handlers::handle_request))
            .route("/2015-03-31/functions/:function_name", any(crate::services::lambda::handlers::handle_request))
            .route("/2015-03-31/functions", any(crate::services::lambda::handlers::handle_request));
    }

    router
        .with_state(emulator)
        .layer(TraceLayer::new_for_http())
}

/// Health check endpoint
async fn health_check() -> &'static str {
    r#"{"status":"running","version":"0.1.0","services":{"s3":"available"}}"#
}
