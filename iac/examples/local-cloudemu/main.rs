//! CloudEmu Integration Example
//!
//! This example demonstrates how to configure CloudKit to work with CloudEmu,
//! the multi-cloud local emulator.
//!
//! Run with: `cargo run --example cloudemu_integration --features "aws, s3, dynamodb"`

use cloudkit::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("CloudKit + CloudEmu Integration Example");
    println!("=====================================\n");

    // Method 1: Using the CloudKit Facade (Recommended)
    // Automatically sets up correct local endpoints (e.g., http://localhost:4566 for AWS)
    println!("1. Connecting via Facade...");
    let client = CloudKit::cloudemu(ProviderType::Aws)
        .region(Region::aws_us_east_1())
        .build()
        .await?;

    println!("   ✓ Connected to CloudEmu AWS endpoint");

    // Method 2: Manually configuring the builder
    // Useful if you are using provider-specific crates directly or have a custom emulator port
    println!("\n2. Connecting via Direct Builder...");
    let aws = cloudkit_aws::AwsBuilder::new()
        .endpoint("http://localhost:4566")
        .region(Region::aws_us_east_1())
        .build()
        .await?;

    println!("   ✓ Connected to custom endpoint: http://localhost:4566");

    // Demonstrate functionality (if CloudEmu is running)
    println!("\n3. Attempting S3 Operation...");
    match aws.storage().create_bucket("cloudkit-test-bucket").await {
        Ok(_) => println!("   ✓ Bucket created successfully!"),
        Err(e) => {
            if e.to_string().contains("Connection refused") {
                println!("   x Could not connect to CloudEmu.");
                println!("     Run `cargo run -p cloudemu-server` in a separate terminal.");
            } else {
                println!("   x Error: {}", e);
            }
        }
    }

    Ok(())
}
