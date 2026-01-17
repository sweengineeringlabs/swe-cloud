use clap::{Parser, Subcommand};
use zero_control_core::ZeroProvider;
use zero_data_core::ZeroEngine;
use zero_control_spi::{ZeroRequest, ZeroService};
use std::sync::Arc;
use colored::*;
use serde_json::json;

#[derive(Parser)]
#[command(name = "zero")]
#[command(about = "ZeroCloud Private Cloud CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Force the use of native OS drivers (Hyper-V / KVM) instead of Docker
    #[arg(short, long, global = true)]
    pub native: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage Workloads
    Workload {
        #[command(subcommand)]
        action: WorkloadAction,
    },
    /// Manage Volumes
    Volume {
        #[command(subcommand)]
        action: VolumeAction,
    },
    /// Manage Nodes
    Node {
        #[command(subcommand)]
        action: NodeAction,
    },
    /// Manage Networks
    Network {
        #[command(subcommand)]
        action: NetworkAction,
    },
}

#[derive(Subcommand)]
pub enum WorkloadAction {
    /// Create a new workload
    Up {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        image: String,
    },
    /// Delete a workload
    Down {
        #[arg(short, long)]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum VolumeAction {
    /// Create a new volume
    Create {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        size: i32,
    },
}

#[derive(Subcommand)]
pub enum NodeAction {
    /// List all nodes
    List,
}

#[derive(Subcommand)]
pub enum NetworkAction {
    /// Create a new virtual network
    Create {
        #[arg(short, long)]
        id: String,
        #[arg(short, long, default_value = "10.0.0.0/24")]
        cidr: String,
    },
}

pub async fn run_cli(cli: Cli) -> anyhow::Result<()> {
    // Pre-flight checks
    check_wsl_preflight();

    // Initialize Zero Engine (Auto-detect or Force Native)
    let engine = if cli.native {
        println!("{} forcing native OS drivers...", "🔧".blue());
        ZeroEngine::native()
    } else {
        ZeroEngine::auto()
    }.map_err(|e| anyhow::anyhow!("Failed to init ZeroEngine: {}", e))?;
    
    let provider = ZeroProvider::new(Arc::new(engine));
    execute_command(cli.command, &provider).await
}

fn check_wsl_preflight() {
    #[cfg(target_os = "linux")]
    {
        let is_wsl = std::fs::read_to_string("/proc/version")
            .map(|v| v.to_lowercase().contains("microsoft") || v.to_lowercase().contains("wsl"))
            .unwrap_or(false);

        if is_wsl {
             let kvm_exists = std::path::Path::new("/dev/kvm").exists();
             if !kvm_exists {
                 println!("{}", "⚠️  WSL 2 detected but /dev/kvm is missing!".yellow().bold());
                 println!("{}", "To enable KVM on WSL 2, ensure nested virtualization is enabled in your .wslconfig:".yellow());
                 println!("{}", "  [wsl2]\n  nestedVirtualization=true".white());
                 println!("{}", "Then run 'wsl --shutdown' in PowerShell and restart your terminal.".yellow());
                 println!();
             }
        }
    }
}

pub async fn execute_command(command: Commands, provider: &ZeroProvider) -> anyhow::Result<()> {
    match command {
        Commands::Workload { action } => match action {
            WorkloadAction::Up { id, image } => {
                println!("{} Workload {} with image {}...", "🚀 Starting".green(), id.bold(), image.cyan());
                let req = ZeroRequest {
                    method: "POST".into(),
                    path: "/v1/workloads".into(),
                    headers: std::collections::HashMap::new(),
                    body: json!({ "id": id, "image": image }).to_string().into_bytes(),
                };
                let resp = provider.handle_request(req).await?;
                println!("{} Response: {}", "✅".green(), String::from_utf8_lossy(&resp.body));
            }
            WorkloadAction::Down { id } => {
                println!("{} Workload {}...", "🛑 Stopping".red(), id.bold());
                let req = ZeroRequest {
                    method: "DELETE".into(),
                    path: "/v1/workloads".into(),
                    headers: std::collections::HashMap::new(),
                    body: json!({ "id": id }).to_string().into_bytes(),
                };
                let resp = provider.handle_request(req).await?;
                println!("{} Response: {}", "✅".green(), String::from_utf8_lossy(&resp.body));
            }
        },
        Commands::Volume { action } => match action {
            VolumeAction::Create { id, size } => {
                println!("{} Volume {} ({} GB)...", "📂 Provisioning".blue(), id.bold(), size);
                let req = ZeroRequest {
                    method: "POST".into(),
                    path: "/v1/volumes".into(),
                    headers: std::collections::HashMap::new(),
                    body: json!({ "id": id, "size_gb": size }).to_string().into_bytes(),
                };
                let resp = provider.handle_request(req).await?;
                println!("{} Response: {}", "✅".green(), String::from_utf8_lossy(&resp.body));
            }
        },
        Commands::Node { action } => match action {
            NodeAction::List => {
                let req = ZeroRequest {
                    method: "GET".into(),
                    path: "/v1/nodes".into(),
                    headers: std::collections::HashMap::new(),
                    body: vec![],
                };
                let resp = provider.handle_request(req).await?;
                println!("{}", "📋 Local Compute Nodes:".bold().underline());
                println!("{}", String::from_utf8_lossy(&resp.body));
            }
        }
        Commands::Network { action } => match action {
            NetworkAction::Create { id, cidr } => {
                println!("{} Network {} with CIDR {}...", "🌐 Creating".cyan(), id.bold(), cidr.yellow());
                let req = ZeroRequest {
                    method: "POST".into(),
                    path: "/v1/networks".into(),
                    headers: std::collections::HashMap::new(),
                    body: json!({ "id": id, "cidr": cidr }).to_string().into_bytes(),
                };
                let resp = provider.handle_request(req).await?;
                println!("{} Response: {}", "✅".green(), String::from_utf8_lossy(&resp.body));
            }
        }
    }

    Ok(())
}
