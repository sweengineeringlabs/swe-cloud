use clap::Parser;
use cloudemu_server::config::AppConfig;
use cloudemu_server::server::start_provider_server;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::info;

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
        
        info!("Initializing AWS Provider (Port: {})...", port);

        // Configure legacy AWS control-plane
        let dp_config = data_plane::Config {
            port,
            host: "0.0.0.0".to_string(),
            data_dir,
            ..Default::default()
        };

        // Spawn AWS server
        tasks.spawn(async move {
            if let Err(e) = cloudemu_aws::gateway::ingress::start(dp_config).await {
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
            let provider = Arc::new(cloudemu_azure::AzureProvider::new());
            if let Err(e) = start_provider_server(provider, port, "Azure").await {
                 tracing::error!("Azure Server failed: {}", e);
                 return Err(e);
            }
             Ok(())
        });
    }

    // 3. GCP Provider
    if config.enable_gcp {
        let port = config.gcp_port;
        info!("Initializing GCP Provider (Port: {})...", port);
        
        tasks.spawn(async move {
            let provider = Arc::new(cloudemu_gcp::GcpProvider::new());
            if let Err(e) = start_provider_server(provider, port, "GCP").await {
                 tracing::error!("GCP Server failed: {}", e);
                 return Err(e);
            }
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
