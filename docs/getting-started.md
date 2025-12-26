# Quick Start Guide

Get up and running with CloudKit in 5 minutes.

## Prerequisites

- Rust 1.85 or later
- Cargo (comes with Rust)
- Cloud provider credentials (AWS, Azure, GCP, or Oracle)

## Installation

Add CloudKit to your `Cargo.toml`:

```toml
[dependencies]
cloudkit = "0.1"

# Add provider crates as needed
cloudkit-aws = "0.1"     # For AWS
cloudkit-azure = "0.1"   # For Azure
cloudkit-gcp = "0.1"     # For GCP
cloudkit-oracle = "0.1"  # For Oracle Cloud

# Required runtime
tokio = { version = "1", features = ["full"] }
```

## Basic Example

### AWS S3 Upload

```rust
use cloudkit::prelude::*;
use cloudkit_aws::AwsBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create AWS client
    let aws = AwsBuilder::new()
        .region(Region::aws_us_east_1())
        .build()
        .await?;

    // Upload a file
    aws.storage()
        .put_object("my-bucket", "hello.txt", b"Hello, World!")
        .await?;

    println!("Upload complete!");
    Ok(())
}
```

### Provider-Agnostic Code

Write code that works with any cloud provider:

```rust
use cloudkit::prelude::*;

// This function works with AWS, Azure, GCP, or Oracle
async fn backup_data<S: ObjectStorage>(
    storage: &S,
    bucket: &str,
    key: &str,
    data: &[u8],
) -> CloudResult<()> {
    storage.put_object(bucket, key, data).await?;
    println!("Backed up {} bytes to {}/{}", data.len(), bucket, key);
    Ok(())
}
```

## Environment Variables

Set up your credentials:

### AWS
```bash
export AWS_ACCESS_KEY_ID=your-access-key
export AWS_SECRET_ACCESS_KEY=your-secret-key
export AWS_REGION=us-east-1
```

### Azure
```bash
export AZURE_STORAGE_ACCOUNT=your-account
export AZURE_STORAGE_KEY=your-key
```

### GCP
```bash
export GOOGLE_APPLICATION_CREDENTIALS=/path/to/credentials.json
export GCP_PROJECT_ID=your-project-id
```

### Oracle Cloud
```bash
export OCI_TENANCY_OCID=your-tenancy
export OCI_USER_OCID=your-user
export OCI_PRIVATE_KEY_PATH=/path/to/key.pem
```

## Next Steps

1. Read the [Architecture Overview](architecture.md) to understand the design
2. Explore [Provider Documentation](providers/README.md) for your cloud
3. Learn about [Error Handling](error-handling.md)
4. Check out more [Examples](../examples/)

## Common Operations

### List Objects

```rust
let objects = storage.list_objects("bucket", ListOptions::new().prefix("folder/")).await?;

for obj in objects.items {
    println!("{}: {} bytes", obj.key, obj.size);
}
```

### Download Object

```rust
let data = storage.get_object("bucket", "key").await?;
let content = String::from_utf8_lossy(&data);
println!("Content: {}", content);
```

### Delete Object

```rust
storage.delete_object("bucket", "key").await?;
```

### Generate Presigned URL

```rust
use std::time::Duration;

let url = storage
    .presigned_get_url("bucket", "key", Duration::from_secs(3600))
    .await?;

println!("Download URL: {}", url);
```

## Troubleshooting

### Credential Errors

If you get `AuthError::MissingCredentials`:
1. Check environment variables are set correctly
2. Verify credential file paths exist
3. Ensure IAM permissions are correct

### Network Errors

If you get `NetworkError::Connection`:
1. Check internet connectivity
2. Verify firewall rules
3. Check if using a proxy and configure it

### Rate Limiting

If you get `CloudError::RateLimited`:
1. Wait for the suggested `retry_after` duration
2. Implement exponential backoff (built-in by default)
3. Request quota increases from your cloud provider

## Getting Help

- [GitHub Issues](https://github.com/phdsystems/cloudkit/issues)
- [Documentation](https://docs.rs/cloudkit)
- [Examples](../examples/)
