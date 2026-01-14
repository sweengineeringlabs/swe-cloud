//! DynamoDB Example (CloudKit Facade)
//!
//! Demonstrates Key-Value store operations using the detailed CloudKit facade.
//!
//! Run: `cargo run --example 07_database_dynamodb --features "aws, dynamodb"`

use cloudkit::prelude::*;
use cloudkit_spi::ProviderType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("CloudKit Facade: DynamoDB Example");
    println!("=================================\n");

    // 1. Initialize Client via Facade (Local Emulator)
    let client = CloudKit::cloudemu(ProviderType::Aws)
        .region(Region::aws_us_east_1())
        .build()
        .await?;
        
    println!("✓ Connected to {}", client.context().provider());

    // 2. Access the Key-Value Store interface
    // Note: The specific return type here depends on the provider (DynamoDbStore)
    // but typically implements a common KeyValue trait if available.
    let _kv = client.aws().kv_store();
    
    println!("✓ Key-Value Client obtained");
    println!("\nIn a real application, you can now perform operations:");
    println!("  - Put Item");
    println!("  - Get Item");
    println!("  - Query / Scan");

    Ok(())
}
