//! CloudEmu - Local Cloud Emulator
//!
//! A production-grade local cloud emulator for development and testing.

use cloudemu::start_server;
use data_plane::Config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cloudemu=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Print banner
    println!(r#"
   _____ _                 _ ______                
  / ____| |               | |  ____|               
 | |    | | ___  _   _  __| | |__   _ __ ___  _   _ 
 | |    | |/ _ \| | | |/ _` |  __| | '_ ` _ \| | | |
 | |____| | (_) | |_| | (_| | |____| | | | | | |_| |
  \_____|_|\___/ \__,_|\__,_|______|_| |_| |_|\__,_|
                                                    
  Production-Grade Local Cloud Emulator v{}
"#, env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = Config::from_env();
    
    println!("  ═══════════════════════════════════════════════════");
    println!("  Endpoint:    http://{}:{}", config.host, config.port);
    println!("  Data Dir:    {}", config.data_dir.display());
    println!("  Region:      {}", config.region);
    println!("  Account ID:  {}", config.account_id);
    println!("  ═══════════════════════════════════════════════════");
    println!();
    println!("  Terraform Configuration:");
    println!("  ─────────────────────────");
    println!(r#"  provider "aws" {{
    endpoints {{
      s3 = "http://{}:{}"
    }}
    region                      = "{}"
    skip_credentials_validation = true
    skip_metadata_api_check     = true
    skip_requesting_account_id  = true
    s3_use_path_style           = true
  }}"#, config.host, config.port, config.region);
    println!();
    println!("  AWS CLI:");
    println!("  ─────────");
    println!("  aws --endpoint-url=http://{}:{} s3 ls", config.host, config.port);
    println!();

    // Start server
    start_server(config).await?;

    Ok(())
}
