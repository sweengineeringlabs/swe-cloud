//! Example: Using CloudEmu with AWS SDK for Rust
//!
//! This example shows how to use the official AWS SDK for Rust
//! with CloudEmu. The same code works with real AWS - just change
//! the endpoint URL.
//!
//! # Running
//!
//! 1. Start CloudEmu Server:
//!    ```bash
//!    cargo run -p cloudemu_server
//!    ```
//!
//! 2. Run this example:
//!    ```bash
//!    # From cloudemu/aws/control-plane/aws-control-facade directory:
//!    cargo run --example aws_sdk_usage
//!    ```

use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure AWS SDK to use CloudEmu
    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url("http://localhost:4566")  // CloudEmu endpoint
        .region(aws_config::Region::new("us-east-1"))
        .load()
        .await;

    let s3 = Client::new(&config);

    println!("=== CloudEmu AWS SDK Example ===\n");

    // 1. Create a bucket
    println!("1. Creating bucket 'my-rust-bucket'...");
    s3.create_bucket()
        .bucket("my-rust-bucket")
        .send()
        .await?;
    println!("   ✓ Bucket created\n");

    // 2. Enable versioning
    println!("2. Enabling versioning...");
    s3.put_bucket_versioning()
        .bucket("my-rust-bucket")
        .versioning_configuration(
            aws_sdk_s3::types::VersioningConfiguration::builder()
                .status(aws_sdk_s3::types::BucketVersioningStatus::Enabled)
                .build()
        )
        .send()
        .await?;
    println!("   ✓ Versioning enabled\n");

    // 3. Set a bucket policy
    println!("3. Setting bucket policy...");
    let policy = r#"{
        "Version": "2012-10-17",
        "Statement": [
            {
                "Sid": "PublicReadGetObject",
                "Effect": "Allow",
                "Principal": "*",
                "Action": "s3:GetObject",
                "Resource": "arn:aws:s3:::my-rust-bucket/*"
            }
        ]
    }"#;
    
    s3.put_bucket_policy()
        .bucket("my-rust-bucket")
        .policy(policy)
        .send()
        .await?;
    println!("   ✓ Policy set\n");

    // 4. Upload objects
    println!("4. Uploading objects...");
    
    s3.put_object()
        .bucket("my-rust-bucket")
        .key("hello.txt")
        .body(aws_sdk_s3::primitives::ByteStream::from("Hello, World!".as_bytes().to_vec()))
        .content_type("text/plain")
        .send()
        .await?;
    println!("   ✓ Uploaded hello.txt");

    s3.put_object()
        .bucket("my-rust-bucket")
        .key("data/config.json")
        .body(aws_sdk_s3::primitives::ByteStream::from(r#"{"name": "my-app", "version": "1.0.0"}"#.as_bytes().to_vec()))
        .content_type("application/json")
        .send()
        .await?;
    println!("   ✓ Uploaded data/config.json\n");

    // 5. List objects
    println!("5. Listing objects...");
    let objects = s3.list_objects_v2()
        .bucket("my-rust-bucket")
        .send()
        .await?;

    for obj in objects.contents().iter() {
        println!("   - {} ({} bytes)", 
            obj.key().unwrap_or("?"), 
            obj.size().unwrap_or(0)
        );
    }
    println!();

    // 6. Get an object
    println!("6. Getting hello.txt...");
    let response = s3.get_object()
        .bucket("my-rust-bucket")
        .key("hello.txt")
        .send()
        .await?;
    
    let data = response.body.collect().await?;
    let content_bytes = data.into_bytes();
    let content = String::from_utf8_lossy(&content_bytes);
    println!("   Content: {}\n", content);

    // 7. Get bucket policy
    println!("7. Getting bucket policy...");
    let policy_response = s3.get_bucket_policy()
        .bucket("my-rust-bucket")
        .send()
        .await?;
    println!("   Policy: {}\n", policy_response.policy().unwrap_or("none"));

    // 8. Get versioning status
    println!("8. Getting versioning status...");
    let versioning = s3.get_bucket_versioning()
        .bucket("my-rust-bucket")
        .send()
        .await?;
    println!("   Status: {:?}\n", versioning.status());

    // 9. Copy object
    println!("9. Copying hello.txt to hello-copy.txt...");
    s3.copy_object()
        .bucket("my-rust-bucket")
        .key("hello-copy.txt")
        .copy_source("my-rust-bucket/hello.txt")
        .send()
        .await?;
    println!("   ✓ Object copied\n");

    // 10. Delete objects
    println!("10. Deleting objects...");
    s3.delete_object()
        .bucket("my-rust-bucket")
        .key("hello.txt")
        .send()
        .await?;
    s3.delete_object()
        .bucket("my-rust-bucket")
        .key("hello-copy.txt")
        .send()
        .await?;
    s3.delete_object()
        .bucket("my-rust-bucket")
        .key("data/config.json")
        .send()
        .await?;
    println!("   ✓ Objects deleted\n");

    // 11. Delete bucket policy
    println!("11. Deleting bucket policy...");
    s3.delete_bucket_policy()
        .bucket("my-rust-bucket")
        .send()
        .await?;
    println!("   ✓ Policy deleted\n");

    // 12. Delete bucket
    println!("12. Deleting bucket...");
    s3.delete_bucket()
        .bucket("my-rust-bucket")
        .send()
        .await?;
    println!("   ✓ Bucket deleted\n");

    println!("=== All operations completed successfully! ===");
    println!("\nThis same code works with real AWS S3.");
    println!("Just remove the .endpoint_url() line to use production AWS.");

    Ok(())
}
