use clap::Parser;
use std::path::PathBuf;
use tracing::info;
use aws_control_facade::gateway;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Config {
    /// AWS Port
    #[arg(long, default_value_t = 4566, env = "CLOUDEMU_AWS_PORT")]
    port: u16,

    /// Data directory
    #[arg(long, default_value = ".cloudemu/aws", env = "CLOUDEMU_DATA_DIR")]
    data_dir: PathBuf,

    /// Host
    #[arg(long, default_value = "0.0.0.0", env = "CLOUDEMU_HOST")]
    host: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::parse();

    info!("Starting CloudEmu AWS Server on {}:{}", config.host, config.port);
    info!("Data Directory: {:?}", config.data_dir);

    gateway::ingress::start(&config.host, config.port, config.data_dir).await?;
    
    Ok(())
}
