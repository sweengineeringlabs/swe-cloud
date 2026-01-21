//! SQS Example (CloudKit Facade)
//!
//! Demonstrates Queue messaging operations using the detailed CloudKit facade.
//!
//! Run: `cargo run --example 08_messaging_sqs --features "aws, sqs"`

use cloudkit::prelude::*;
use cloudkit_spi::ProviderType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("CloudKit Facade: SQS Example");
    println!("============================\n");

    // 1. Initialize Client via Facade
    let client = CloudKit::cloudemu(ProviderType::Aws)
        .region(Region::aws_us_east_1())
        .build()
        .await?;

    println!("✓ Connected to {}", client.provider());

    // 2. Queue interface would be accessed via provider-specific crate
    // For now, just demonstrate that we have a valid context
    println!("✓ Context obtained for provider: {}", client.provider());
    println!("\nOperations available:");
    println!("  - Send Message");
    println!("  - Receive Messages");
    println!("  - Delete Message");

    Ok(())
}
