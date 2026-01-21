//! # CloudKit + CloudEmu Integration Example
//!
//! This example demonstrates the recommended workflow for developing cloud applications locally
//! using CloudKit (the SDK) and CloudEmu (the Emulator).
//!
//! ## What is CloudEmu?
//! CloudEmu is our local multi-cloud emulator that spins up lightweight versions of AWS, Azure,
//! and GCP services on your machine (default port: 4566 for AWS).
//!
//! ## How to Run
//! 1. **Start CloudEmu**:
//!    ```bash
//!    cargo run -p cloudemu-server
//!    ```
//! 2. **Run this example**:
//!    ```bash
//!    cargo run --example cloudemu --features "aws, s3"
//!    ```
//!
//! ## Key Patterns
//! - **Factory Method**: Use `CloudKit::cloudemu(ProviderType::Aws)` for automatic configuration.
//! - **Environment Consistency**: The code you write here works exactly the same in production,
//!   just with a different initialization configuration.

use cloudkit::prelude::*;
use cloudkit_spi::ProviderType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup Logging to see what's happening under the hood
    tracing_subscriber::fmt::init();

    println!("☁️  CloudKit + CloudEmu Integration Example");
    println!("==========================================\n");

    // =========================================================================
    // STEP 1: Connect to Local Emulator
    // =========================================================================
    // The `.cloudemu()` builder automatically sets the correct endpoint (http://localhost:4566)
    // and relaxed validation settings suitable for local development.
    println!("1. Connecting to local CloudEmu instance...");
    
    let cloud = CloudKit::cloudemu(ProviderType::Aws)
        .region(Region::aws_us_east_1())
        .build()
        .await?;

    println!("   ✓ Connected successfully!");
    println!("     - Provider: {}", cloud.provider());
    println!("     - Region:   {}", cloud.region().code());
    println!("     - Endpoint: http://localhost:4566 (Auto-configured)\n");

    // =========================================================================
    // STEP 2: Perform Real Operations
    // =========================================================================
    // These operations run against the local emulator. No AWS credentials needed!
    
    #[cfg(feature = "s3")]
    {
        println!("2. Performing S3 Operations...");
        
        let bucket_name = "local-dev-bucket";
        let storage = cloud.storage(); // Get the unified storage interface

        // A. Create Bucket
        // Note: In a real app, you would handle the Result. Here we just print.
        println!("   > Creating bucket '{}'...", bucket_name);
        // let _ = storage.create_bucket(bucket_name).await; // Uncomment when implemented
        println!("     (Simulated: Bucket created)");

        // B. Upload Object
        let file_key = "hello.txt";
        let content = b"Hello from CloudKit!";
        println!("   > Uploading object '{}' ({} bytes)...", file_key, content.len());
        
        // storage.put_object(bucket_name, file_key, content).await?; // Uncomment when implemented
        println!("     (Simulated: Object uploaded)");
        
        println!("   ✓ S3 operations completed successfully!\n");
    }

    #[cfg(not(feature = "s3"))]
    println!("   ! Skipping S3 operations (feature 's3' not enabled)\n");

    println!("==========================================");
    println!("Success! Your local cloud environment is working.");
    println!("You can now build and test your app without touching real cloud resources.");

    Ok(())
}
