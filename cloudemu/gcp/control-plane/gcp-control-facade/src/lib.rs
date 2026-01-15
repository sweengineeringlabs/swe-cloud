//! GCP Control-Plane Facade
#![warn(missing_docs)]

use std::sync::Arc;
use axum::{
    Router,
    extract::{State, Request},
    response::Response,
    body::Body,
    http::StatusCode,
};

use gcp_control_spi::{CloudProviderTrait, Request as GcpRequest};

// Re-exports
pub use gcp_control_core;
pub use gcp_control_spi;
pub use gcp_control_api;
pub use gcp_control_core::GcpProvider;

/// Create an Axum router for the GCP provider.
pub fn router(provider: Arc<GcpProvider>) -> Router {
    Router::new()
        .fallback(handle_request)
        .with_state(provider)
}

async fn handle_request(
    State(provider): State<Arc<GcpProvider>>,
    req: Request,
) -> Response<Body> {
    // Convert to GCP Request
    let (parts, body) = req.into_parts();
    
    // Simplified body collection
    let body_bytes = match http_body_util::BodyExt::collect(body).await {
        Ok(collected) => collected.to_bytes().to_vec(),
        Err(e) => return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(format!("Failed to read body: {}", e)))
            .unwrap(),
    };
    
    let path = parts.uri.path_and_query()
        .map(|p| p.as_str())
        .unwrap_or(parts.uri.path())
        .to_string();
        
    let mut headers = std::collections::HashMap::new();
    for (name, value) in parts.headers {
        if let Some(n) = name {
            if let Ok(v) = value.to_str() {
                headers.insert(n.to_string(), v.to_string());
            }
        }
    }

    let gcp_req = GcpRequest {
        method: parts.method.to_string(),
        path,
        headers,
        body: body_bytes,
    };

    match provider.handle_request(gcp_req).await {
        Ok(res) => {
            let mut builder = Response::builder().status(res.status);
            for (k, v) in res.headers {
                builder = builder.header(k, v);
            }
            builder.body(Body::from(res.body)).unwrap()
        }
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(format!("Error: {}", e)))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    use gcp_control_core::GcpProvider;

    #[tokio::test]
    async fn test_router_routing() {
        let provider = Arc::new(GcpProvider::in_memory());
        let app = router(provider);

        let response = app
            .oneshot(Request::builder().method("PUT").uri("/test-bucket").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }
}
