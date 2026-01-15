mod config;
mod error;

use axum::{
    routing::get,
    Router,
    Json,
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use gcp_data_core::StorageEngine;
use gcp_data_core::config::Config as CoreConfig;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env();
    
    // Initialize Core Storage Engine
    let core_config = CoreConfig::default()
        .data_dir(config.data_dir.clone());
    
    // In a real app we'd handle this error properly
    let _storage = StorageEngine::new(&core_config).expect("Failed to initialize storage engine");

    let app = Router::new()
        .route("/health", get(health_check));

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse().unwrap();
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "gcp-data-api"
    }))
}
