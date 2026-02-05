use clap::Parser;
use std::sync::Arc;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tracing::info;
use azure_control_facade::{router, AzureProvider};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Config {
    /// Azure Port
    #[arg(long, default_value_t = 4567, env = "CLOUDEMU_AZURE_PORT")]
    port: u16,

    /// Host
    #[arg(long, default_value = "0.0.0.0", env = "CLOUDEMU_HOST")]
    host: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::parse();

    info!("Starting CloudEmu Azure Server on {}:{}", config.host, config.port);

    let provider = Arc::new(AzureProvider::new());
    let app = router(provider);

    let host_ip: std::net::IpAddr = config.host.parse()?;
    let addr = SocketAddr::from((host_ip, config.port));
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
