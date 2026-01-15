use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct AppConfig {
    /// AWS Port
    #[arg(long, default_value_t = 4566, env = "CLOUDEMU_AWS_PORT")]
    pub aws_port: u16,

    /// Azure Port
    #[arg(long, default_value_t = 4567, env = "CLOUDEMU_AZURE_PORT")]
    pub azure_port: u16,

    /// GCP Port
    #[arg(long, default_value_t = 4568, env = "CLOUDEMU_GCP_PORT")]
    pub gcp_port: u16,

    /// Enable AWS
    #[arg(long, default_value_t = true, env = "CLOUDEMU_ENABLE_AWS")]
    pub enable_aws: bool,

    /// Enable Azure
    #[arg(long, default_value_t = true, env = "CLOUDEMU_ENABLE_AZURE")]
    pub enable_azure: bool,

    /// Enable GCP
    #[arg(long, default_value_t = true, env = "CLOUDEMU_ENABLE_GCP")]
    pub enable_gcp: bool,

    /// Data directory
    #[arg(long, default_value = ".cloudemu", env = "CLOUDEMU_DATA_DIR")]
    pub data_dir: PathBuf,
}
