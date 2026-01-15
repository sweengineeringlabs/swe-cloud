use clap::Parser;
use cloudemu_server::config::AppConfig;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::info;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::{Router, extract::{State, Request}, response::Response, body::Body, http::StatusCode};
use std::path::PathBuf;

// AWS Imports
use aws_control_facade::gateway;

// Azure Imports
use azure_control_facade::{AzureProvider, azure_control_spi::{CloudProviderTrait as AzureTrait, Request as AzureRequest}};

// GCP Imports
use gcp_control_facade::{GcpProvider, gcp_control_spi::{CloudProviderTrait as GcpTrait, Request as GcpRequest}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let config = AppConfig::parse();
    let mut tasks = JoinSet::new();

    info!("Initializing CloudEmu Multi-Cloud Server...");
    info!("----------------------------------------");

    // 1. AWS Provider
    if config.enable_aws {
        let port = config.aws_port;
        let data_dir = config.data_dir.join("aws");
        let host = "0.0.0.0";
        
        info!("Initializing AWS Provider (Port: {})...", port);

        tasks.spawn(async move {
            if let Err(e) = gateway::ingress::start(host, port, data_dir).await {
                 tracing::error!("AWS Server failed: {}", e);
                 return Err(anyhow::anyhow!("AWS failure: {}", e));
            }
            Ok(())
        });
    }

    // 2. Azure Provider
    if config.enable_azure {
        let port = config.azure_port;
        info!("Initializing Azure Provider (Port: {})...", port);
        
        tasks.spawn(async move {
            let provider = Arc::new(AzureProvider::new());
            let app = Router::new()
                .fallback(handle_azure_request)
                .with_state(provider);

            let addr = SocketAddr::from(([0, 0, 0, 0], port));
            let listener = TcpListener::bind(addr).await?;
            axum::serve(listener, app).await?;
            Ok(())
        });
    }

    // 3. GCP Provider
    if config.enable_gcp {
        let port = config.gcp_port;
        info!("Initializing GCP Provider (Port: {})...", port);
        
        tasks.spawn(async move {
            let provider = Arc::new(GcpProvider::new());
            let app = Router::new()
                .fallback(handle_gcp_request)
                .with_state(provider);

            let addr = SocketAddr::from(([0, 0, 0, 0], port));
            let listener = TcpListener::bind(addr).await?;
            axum::serve(listener, app).await?;
            Ok(())
        });
    }
    
    // Wait for tasks
    if tasks.is_empty() {
        tracing::warn!("No providers enabled. Exiting.");
        return Ok(());
    }

    while let Some(res) = tasks.join_next().await {
        match res {
            Ok(ret) => {
                if let Err(e) = ret {
                    tracing::error!("Provider task failed: {}", e);
                }
            },
            Err(e) => tracing::error!("Join error: {}", e),
        }
    }

    Ok(())
}

async fn handle_azure_request(
    State(provider): State<Arc<AzureProvider>>,
    req: Request,
) -> Response<Body> {
    // Convert to Azure Request
    let (parts, body) = req.into_parts();
    let body_bytes = http_body_util::BodyExt::collect(body).await.unwrap().to_bytes().to_vec(); // Simplified error handling
    
    let path = parts.uri.path_and_query().map(|p| p.as_str()).unwrap_or(parts.uri.path()).to_string();
    let mut headers = std::collections::HashMap::new();
    for (name, value) in parts.headers {
        if let Some(n) = name {
            if let Ok(v) = value.to_str() {
                headers.insert(n.to_string(), v.to_string());
            }
        }
    }

    let azure_req = AzureRequest {
        method: parts.method.to_string(),
        path,
        headers,
        body: body_bytes,
    };

    match provider.handle_request(azure_req).await {
        Ok(res) => {
            let mut builder = Response::builder().status(res.status);
            for (k, v) in res.headers {
                builder = builder.header(k, v);
            }
            builder.body(Body::from(res.body)).unwrap()
        }
        Err(e) => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::from(format!("Error: {}", e))).unwrap()
    }
}

async fn handle_gcp_request(
    State(provider): State<Arc<GcpProvider>>,
    req: Request,
) -> Response<Body> {
    // Duplicate logic for GCP (Decoupled!)
    let (parts, body) = req.into_parts();
    let body_bytes = http_body_util::BodyExt::collect(body).await.unwrap().to_bytes().to_vec();
    
    let path = parts.uri.path_and_query().map(|p| p.as_str()).unwrap_or(parts.uri.path()).to_string();
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
        Err(e) => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::from(format!("Error: {}", e))).unwrap()
    }
}
