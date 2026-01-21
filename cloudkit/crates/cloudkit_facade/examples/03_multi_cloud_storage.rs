//! Multi-cloud example for CloudKit.
//!
//! This example demonstrates provider-agnostic code that works with any cloud.
//!
//! Run with: `cargo run --example multi_cloud --features full`

use bytes::Bytes;
use cloudkit::prelude::*;

/// Upload data to any cloud storage provider.
///
/// This function demonstrates provider-agnostic code.
async fn upload_to_storage<S: ObjectStorage>(
    storage: &S,
    bucket: &str,
    key: &str,
    data: &[u8],
) -> CloudResult<()> {
    println!("  Uploading {} bytes to {}/{}...", data.len(), bucket, key);
    
    storage.put_object_with_options(
        bucket,
        key,
        data,
        PutOptions::new()
            .content_type("application/octet-stream")
            .metadata("uploaded-by", "cloudkit-example"),
    ).await?;

    println!("  ✓ Upload complete");
    Ok(())
}

/// Download data from any cloud storage provider.
async fn download_from_storage<S: ObjectStorage>(
    storage: &S,
    bucket: &str,
    key: &str,
) -> CloudResult<bytes::Bytes> {
    println!("  Downloading {}/{}...", bucket, key);
    
    let data = storage.get_object(bucket, key).await?;
    
    println!("  ✓ Downloaded {} bytes", data.len());
    Ok(data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    println!("CloudKit Multi-Cloud Example");
    println!("============================\n");

    // Demonstrate building clients for different providers
    println!("Building cloud clients...\n");

    // AWS
    let aws = CloudKit::aws()
        .region(Region::aws_us_east_1())
        .build()
        .await?;
    println!("✓ AWS client ready (region: {})", aws.region().code());

    // Azure
    let azure = CloudKit::azure()
        .region(Region::azure_east_us())
        .build()
        .await?;
    println!("✓ Azure client ready (region: {})", azure.region().code());

    // GCP
    let gcp = CloudKit::gcp()
        .region(Region::gcp_us_central1())
        .build()
        .await?;
    println!("✓ GCP client ready (region: {})", gcp.region().code());

    // Oracle
    let oracle = CloudKit::oracle()
        .region(Region::oracle_af_johannesburg_1())
        .build()
        .await?;
    println!("✓ Oracle client ready (region: {})", oracle.region().code());

    println!("\n✓ All providers initialized successfully!");
    println!("\nIn a real application, you could use any of these providers");
    println!("with the same ObjectStorage trait interface.");

    Ok(())
}
