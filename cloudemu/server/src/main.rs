use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, error};
use tokio::task;

// Import router creators
// AWS uses aws-control-facade::gateway (which re-exports aws-control-core::gateway)
// But aws-control-facade::gateway::ingress::start binds port directly.
// We might want to use aws_control_facade::gateway::create_router directly if exposed, 
// or just spawn the ingress::start in a task.
// From previous view_file of aws/ingress.rs: start(host, port, data_dir) -> Result<()>

use azure_data_api::router as azure_router;
use azure_data_core::{StorageEngine as AzureStorage, Config as AzureConfig};

use gcp_data_api::router as gcp_router;
use gcp_data_core::{StorageEngine as GcpStorage, Config as GcpConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Config {
    /// AWS Port (default 4566)
    #[arg(long, default_value_t = 4566, env = "CLOUDEMU_AWS_PORT")]
    aws_port: u16,

    /// Azure Port (standard Azurite Blob 10000, but using 4568 to keep in 45xx range? Or 10000?)
    /// Let's use 10000 to be standard-compliant.
    #[arg(long, default_value_t = 10000, env = "CLOUDEMU_AZURE_PORT")]
    azure_port: u16,

    /// GCP Port (default 4567, or 4443 standard?)
    /// Let's use 4567 as validated.
    #[arg(long, default_value_t = 4567, env = "CLOUDEMU_GCP_PORT")]
    gcp_port: u16,

    /// Host
    #[arg(long, default_value = "127.0.0.1", env = "CLOUDEMU_HOST")]
    host: String,

    /// Base Data Directory
    #[arg(long, default_value = ".cloudemu", env = "CLOUDEMU_DATA_DIR")]
    data_dir: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::parse();

    info!("Starting CloudEmu Unified Server");
    info!("Base Data Directory: {:?}", config.data_dir);
    info!("----------------------------------------");
    info!("AWS Service   : http://{}:{}", config.host, config.aws_port);
    info!("Azure Service : http://{}:{}", config.host, config.azure_port);
    info!("GCP Service   : http://{}:{}", config.host, config.gcp_port);
    info!("----------------------------------------");

    // Start AWS
    let aws_host = config.host.clone();
    let aws_port = config.aws_port;
    let aws_dir = config.data_dir.join("aws");
    
    // AWS start function is async and blocking (binds listener). Spawn it.
    let aws_handle = task::spawn(async move {
        // We use aws_control_facade's ingress. 
        // Note: aws_control_facade re-exports gateway -> ingress
        // But main.rs calls gateway::ingress::start
        // Let's rely on aws_control_facade::gateway::ingress::start if it is public.
        // It was public in the file view.
        if let Err(e) = aws_control_facade::gateway::ingress::start(&aws_host, aws_port, aws_dir).await {
            error!("AWS Server failed: {:?}", e);
        }
    });

    // Start Azure
    let azure_host = config.host.clone();
    let azure_port = config.azure_port;
    let azure_dir = config.data_dir.join("azure");
    let azure_handle = task::spawn(async move {
        let core_config = AzureConfig::default()
            .data_dir(azure_dir);
        let storage = match AzureStorage::new(&core_config) {
            Ok(s) => Arc::new(s),
            Err(e) => {
                error!("Failed to init Azure storage: {:?}", e);
                return;
            }
        };
        
        let app = azure_router::create_router(storage);
        let addr = format!("{}:{}", azure_host, azure_port);
        info!("Azure listening on {}", addr);
        
        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                error!("Failed to bind Azure port {}: {:?}", addr, e);
                return;
            }
        };

        if let Err(e) = axum::serve(listener, app).await {
            error!("Azure Server failed: {:?}", e);
        }
    });

    // Start GCP
    let gcp_host = config.host.clone();
    let gcp_port = config.gcp_port;
    let gcp_dir = config.data_dir.join("gcp");
    let gcp_handle = task::spawn(async move {
        let core_config = GcpConfig::default()
            .data_dir(gcp_dir);
        let storage = match GcpStorage::new(&core_config) {
            Ok(s) => Arc::new(s),
            Err(e) => {
                error!("Failed to init GCP storage: {:?}", e);
                return;
            }
        };
        
        let app = gcp_router::create_router(storage);
        let addr = format!("{}:{}", gcp_host, gcp_port);
        info!("GCP listening on {}", addr);
        
        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                error!("Failed to bind GCP port {}: {:?}", addr, e);
                return;
            }
        };

        if let Err(e) = axum::serve(listener, app).await {
            error!("GCP Server failed: {:?}", e);
        }
    });

    // Wait for all
    let _ = tokio::join!(aws_handle, azure_handle, gcp_handle);

    Ok(())
}
