use axum::{
    body::Body,
    extract::{State, Request},
    http::{StatusCode, Response},
    Router,
};
use cloudemu_spi::{CloudProviderTrait, Request as CloudRequest};
use std::sync::Arc;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, error};

/// Start a server for a specific provider
pub async fn start_provider_server(
    provider: Arc<dyn CloudProviderTrait>,
    port: u16,
    name: &str,
) -> anyhow::Result<()> {
    // For AWS, we might handle it differently in main.rs, 
    // but this generic server works for any trait implementation.
    
    let app = Router::new()
        .fallback(handle_request)
        .with_state(provider);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting {} provider on http://{}", name, addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn handle_request(
    State(provider): State<Arc<dyn CloudProviderTrait>>,
    req: Request,
) -> Response<Body> {
    // 1. Convert Axum Request to CloudRequest
    let (parts, body) = req.into_parts();
    let method = parts.method.to_string();
    
    let path = parts.uri.path_and_query()
        .map(|pq| pq.as_str())
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

    let body_bytes = match http_body_util::BodyExt::collect(body).await {
        Ok(collected) => collected.to_bytes().to_vec(),
        Err(e) => {
            error!("Failed to read body: {}", e);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to read body"))
                .unwrap();
        }
    };

    let cloud_req = CloudRequest {
        method,
        path,
        headers,
        body: body_bytes,
    };

    // 2. Call Provider
    match provider.handle_request(cloud_req).await {
        Ok(cloud_res) => {
            // 3. Convert CloudResponse to Axum Response
            let mut builder = Response::builder().status(cloud_res.status);
            for (k, v) in cloud_res.headers {
                builder = builder.header(k, v);
            }
            builder.body(Body::from(cloud_res.body)).unwrap()
        }
        Err(e) => {
            error!("Provider error: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Provider Error: {}", e)))
                .unwrap()
        }
    }
}
