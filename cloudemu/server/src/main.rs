use clap::Parser;
use cloudemu_server::config::AppConfig;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::info;
use std::net::SocketAddr;
use tokio::net::TcpListener;

// AWS Imports
use aws_control_facade::gateway;

// Azure Imports
use azure_control_facade::AzureProvider;

// GCP Imports
use gcp_control_facade::GcpProvider;

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
            // Use the facade's router logic
            let app = azure_control_facade::router(provider);

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
            // Use the facade's router logic
            let app = gcp_control_facade::router(provider);

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
