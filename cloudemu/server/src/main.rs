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

use oracle_control_core::OracleProvider;
use oracle_data_core::StorageEngine as OracleStorage;
use oracle_control_spi::{Request as OracleRequest, CloudProviderTrait};
use axum::{routing::any, body::Body, extract::State};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Config {
    /// AWS Port (default 4566)
    #[arg(long, default_value_t = 4566, env = "CLOUDEMU_AWS_PORT")]
    aws_port: u16,

    /// Azure Port (standard Azurite Blob 10000)
    #[arg(long, default_value_t = 10000, env = "CLOUDEMU_AZURE_PORT")]
    azure_port: u16,

    /// GCP Port (default 4567)
    #[arg(long, default_value_t = 4567, env = "CLOUDEMU_GCP_PORT")]
    gcp_port: u16,

    /// Oracle Port (default 4568)
    #[arg(long, default_value_t = 4568, env = "CLOUDEMU_ORACLE_PORT")]
    oracle_port: u16,

    /// Host
    #[arg(long, default_value = "127.0.0.1", env = "CLOUDEMU_HOST")]
    host: String,

    /// Base Data Directory
    #[arg(long, default_value = ".cloudemu", env = "CLOUDEMU_DATA_DIR")]
    data_dir: PathBuf,
}

// Simple handler for Oracle axum adapter
async fn oracle_handler(
    State(provider): State<Arc<OracleProvider>>,
    req: axum::http::Request<Body>,
) -> axum::response::Response {
    use axum::body::Bytes;
    
    // Convert Axum Request to Control SPI Request
    let (parts, body) = req.into_parts();
    let method = parts.method.to_string();
    let path = parts.uri.path().to_string();
    let headers = parts.headers.iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    
    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(b) => b.to_vec(),
        Err(_) => return axum::response::Response::builder().status(500).body(Body::from("Body error")).unwrap(),
    };

    let spi_req = OracleRequest {
        method,
        path,
        headers,
        body: bytes,
    };

    match provider.handle_request(spi_req).await {
        Ok(res) => {
            let mut builder = axum::response::Response::builder().status(res.status);
            for (k, v) in res.headers {
                builder = builder.header(k, v);
            }
            builder.body(Body::from(res.body)).unwrap()
        },
        Err(e) => {
             axum::response::Response::builder().status(500).body(Body::from(format!("Internal Error: {}", e))).unwrap()
        }
    }
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
    info!("Oracle Service: http://{}:{}", config.host, config.oracle_port);
    info!("----------------------------------------");

    // Start AWS
    let aws_host = config.host.clone();
    let aws_port = config.aws_port;
    let aws_dir = config.data_dir.join("aws");
    
    let aws_handle = task::spawn(async move {
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

    // Start Oracle
    let oracle_host = config.host.clone();
    let oracle_port = config.oracle_port;
    let oracle_dir = config.data_dir.join("oracle");
    let oracle_handle = task::spawn(async move {
        let storage = match OracleStorage::new(oracle_dir) {
             Ok(s) => Arc::new(s),
             Err(e) => {
                 error!("Failed to init Oracle storage: {:?}", e);
                 return;
             }
        };
        let provider = Arc::new(OracleProvider::new(storage));
        
        // Oracle Router (Axum)
        let app = axum::Router::new()
            .route("/*path", any(oracle_handler))
            .with_state(provider);

        let addr = format!("{}:{}", oracle_host, oracle_port);
        info!("Oracle listening on {}", addr);

        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                error!("Failed to bind Oracle port {}: {:?}", addr, e);
                return;
            }
        };

        if let Err(e) = axum::serve(listener, app).await {
            error!("Oracle Server failed: {:?}", e);
        }
    });

    // Wait for all
    let _ = tokio::join!(aws_handle, azure_handle, gcp_handle, oracle_handle);

    Ok(())
}
