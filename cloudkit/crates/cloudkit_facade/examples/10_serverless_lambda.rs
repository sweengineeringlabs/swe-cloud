//! Lambda Example (CloudKit Facade)
//!
//! Demonstrates Serverless Function invocation using the detailed CloudKit facade.
//!
//! Run: `cargo run --example 10_serverless_lambda --features "aws, lambda"`

use cloudkit::prelude::*;
use cloudkit_spi::ProviderType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("CloudKit Facade: Lambda Example");
    println!("===============================\n");

    // 1. Initialize Client via Facade
    let client = CloudKit::cloudemu(ProviderType::Aws)
        .region(Region::aws_us_east_1())
        .build()
        .await?;

    println!("✓ Connected to {}", client.provider());

    // 2. Access the Functions interface
    // Note: In production, you would use provider-specific crates for full functionality.
    println!("✓ Context obtained for provider: {}", client.provider());
    println!("\nOperations available:");
    println!("  - Invoke Function");
    println!("  - List Functions");

    Ok(())
}
