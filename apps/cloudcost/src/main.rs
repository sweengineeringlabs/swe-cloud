//! CloudCost - Standalone FinOps Engine
//! 
//! Architecture:
//! - Ingest: Fetches pricing from providers (AWS/Azure/GCP/Oracle)
//! - Core: Normalizes to FOCUS standard
//! - Calc: Estimates costs based on usage/resources

pub mod ingest;
pub mod core;
pub mod calc;

use clap::{Parser, Subcommand};
use crate::ingest::Ingestor;
use tabled::{Table, Tabled};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List prices for a service
    Prices {
        /// Provider (aws, azure, gcp, oracle)
        #[arg(long)]
        provider: String,
        
        /// Service name (e.g. AmazonEC2) usually ignored now as fetch_prices fetches typical services
        #[arg(long)]
        service: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Prices { provider, service: _ } => {
            println!("Fetching prices for {}...", provider);
            
            let ingest = Ingestor::new().await;
            match ingest.fetch_prices(provider).await {
                Ok(items) => {
                    if items.is_empty() {
                         println!("No pricing data found.");
                    } else {
                        // We need FocusItem to derive Tabled or map it
                        // Since FocusItem is external, let's wrap or clone into a Tabled struct
                        // Or imply Tabled on FocusItem in core
                        // For speed, let's create a local display struct or just print JSON
                        // Or modify core/mod.rs to derive Tabled
                        
                        println!("Found {} items:", items.len());
                        // Just print summary for now as Tabled setup requires editing core
                        for item in items {
                            println!("- [{}] {} ({}): {} {} / {}", item.provider, item.service_category, item.sku, item.billed_cost, item.currency, item.usage_unit);
                        }
                    }
                },
                Err(e) => eprintln!("Error fetching prices: {}", e),
            }
        }
    }
}
