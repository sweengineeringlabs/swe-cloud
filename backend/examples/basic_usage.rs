//! Basic usage example for CloudKit.
//!
//! This example demonstrates how to use CloudKit with AWS S3.
//!
//! Run with: `cargo run --example basic_usage --features aws`

use cloudkit::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("CloudKit Basic Usage Example");
    println!("============================\n");

    // Create an AWS client
    let aws = CloudKit::aws()
        .region(Region::aws_us_east_1())
        .build()
        .await?;

    println!("✓ Created AWS client for region: {}", aws.context().region());

    // Example: List buckets (would require real credentials)
    println!("\nAttempting to list buckets...");
    
    // Note: In a real scenario, you would:
    // let buckets = aws.storage().list_buckets().await?;
    // for bucket in buckets {
    //     println!("  - {}", bucket.name);
    // }

    println!("✓ Example complete!");

    Ok(())
}
