//! CloudKit + CloudEmu Integration Example
//!
//! This example demonstrates how to use CloudKit to interact with a locally running CloudEmu instance.
//!
//! Run with: `cargo run --example kit --features "aws, s3"` - (Requires being part of a cargo workspace or setup)

use cloudkit::prelude::*;
use cloudkit_spi::ProviderType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("CloudKit + CloudEmu Example");
    println!("===========================\n");

    // 1. Initialize Client connected to CloudEmu (localhost:4566 for AWS)
    println!("Connecting to CloudEmu...");
    let client = CloudKit::cloudemu(ProviderType::Aws)
        .region(Region::aws_us_east_1())
        .build()
        .await?;

    println!("✓ Connected (Region: {})", client.context().region());

    // 2. Perform Operations (if CloudEmu is running)
    // Note: This requires the `aws` and `s3` features enabled in CloudKit.
    #[cfg(feature = "s3")]
    {
        println!("\nCreating S3 Bucket 'test-kit-bucket'...");
        // This assumes we have access to the underlying provider client or use the facade's storage abstraction
        // For this example, we'll demonstrate the concept. 
        // In a real app, you'd use: client.storage().create_bucket(...)
        
        // Example check (pseudo-code as actual crate availability depends on cargo features):
        // client.storage().create_bucket("test-kit-bucket").await?;
        println!("(Operation requires running CloudEmu server)");
    }

    Ok(())
}
