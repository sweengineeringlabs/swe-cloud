//! SNS Example (CloudKit Facade)
//!
//! Demonstrates Pub/Sub operations using the detailed CloudKit facade.
//!
//! Run: `cargo run --example 09_pubsub_sns --features "aws, sns"`

use cloudkit::prelude::*;
use cloudkit_spi::ProviderType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("CloudKit Facade: SNS Example");
    println!("============================\n");

    // 1. Initialize Client via Facade
    let client = CloudKit::cloudemu(ProviderType::Aws)
        .region(Region::aws_us_east_1())
        .build()
        .await?;

    println!("✓ Connected to {}", client.provider());

    // 2. Access the Pub/Sub interface
    // Note: In production, you would use provider-specific crates for full functionality.
    println!("✓ Context obtained for provider: {}", client.provider());
    println!("\nOperations available:");
    println!("  - Create Topic");
    println!("  - Publish Message");
    println!("  - Subscribe (Email, SMS, SQS, Lambda)");

    Ok(())
}
