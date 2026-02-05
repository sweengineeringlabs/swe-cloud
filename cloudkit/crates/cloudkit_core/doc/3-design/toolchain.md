# Toolchain

## Overview

CloudKit Core orchestrates multi-cloud operations and requires cloud provider SDKs, async runtime, and testing tools.

## Tools

### Rust Compiler

| | |
|---|---|
| **What** | Systems programming language compiler |
| **Version** | 1.70+ (recommended: 1.75+) |
| **Install** | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |

**Why we use it**: Core implementation layer with async/await and complex type system usage.

**How we use it**:
```bash
cargo build -p cloudkit_core
cargo test -p cloudkit_core --lib  # Unit tests
cargo test -p cloudkit_core --test '*'  # Integration tests
```

### Tokio Runtime

| | |
|---|---|
| **What** | Async runtime for Rust |
| **Version** | 1.35+ |
| **Install** | Automatic via Cargo |

**Why we use it**: Enables concurrent cloud operations with retry policies.

**How we use it**:
```rust
pub async fn execute<F, Fut, T>(&self, operation: F) -> CloudResult<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = CloudResult<T>>,
{
    // Retry logic with backoff
    for attempt in 0..max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if e.is_retriable() => {
                tokio::time::sleep(backoff(attempt)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

### AWS SDK for Rust

| | |
|---|---|
| **What** | Official AWS SDK |
| **Version** | aws-sdk-s3 0.35+, aws-config 1.0+ |
| **Install** | Automatic via Cargo |

**Why we use it**: Provides AWS service clients (S3, DynamoDB, SQS, etc.).

**How we use it**:
```rust
use aws_sdk_s3::Client as S3Client;

pub struct AwsProvider {
    s3_client: S3Client,
    dynamodb_client: DynamoDbClient,
}
```

### Azure SDK for Rust

| | |
|---|---|
| **What** | Azure SDK (community-maintained) |
| **Version** | azure_storage 0.19+, azure_identity 0.19+ |
| **Install** | Automatic via Cargo |

**Why we use it**: Provides Azure service clients (Blob, Queue, Table).

**How we use it**:
```rust
use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::*;

pub struct AzureProvider {
    blob_client: ContainerClient,
}
```

### Google Cloud SDK

| | |
|---|---|
| **What** | GCP client libraries |
| **Version** | google-cloud-storage 0.16+ |
| **Install** | Automatic via Cargo |

**Why we use it**: Provides GCP service clients (GCS, Pub/Sub).

**How we use it**:
```rust
use google_cloud_storage::client::Client;

pub struct GcpProvider {
    storage_client: Client,
}
```

## Development Tools

### Cargo-nextest

| | |
|---|---|
| **What** | Modern test runner |
| **Version** | Latest |
| **Install** | `cargo install cargo-nextest` |

**Why we use it**: Faster test execution with better output formatting.

**How we use it**:
```bash
cargo nextest run -p cloudkit_core
```

### Mockall (dev-dependency)

| | |
|---|---|
| **What** | Mock object library |
| **Version** | 0.12+ |
| **Install** | Automatic via Cargo (dev-dependencies) |

**Why we use it**: Mock cloud SDK calls in unit tests.

**How we use it**:
```rust
#[cfg(test)]
use mockall::{automock, predicate::*};

#[automock]
trait CloudProvider {
    async fn put_object(&self, bucket: &str, key: &str) -> Result<()>;
}
```

## Version Matrix

| Tool | Minimum | Recommended | Purpose |
|------|---------|-------------|---------|
| Rust | 1.70 | 1.75+ | Core language |
| Tokio | 1.35 | Latest | Async runtime |
| AWS SDK | 0.35 | Latest | AWS operations |
| Azure SDK | 0.19 | Latest | Azure operations |
| GCP SDK | 0.16 | Latest | GCP operations |
| cargo-nextest | - | Latest | Fast testing |
| mockall | 0.12 | Latest | Unit testing |

## Verification

### Build Verification

```bash
# Standard build
cargo build -p cloudkit_core
# Expected: Finished dev

# Check compilation with all features
cargo check -p cloudkit_core --all-features
# Expected: Finished dev
```

### Test Verification

```bash
# Unit tests (no cloud credentials required)
cargo test -p cloudkit_core --lib
# Expected: test result: ok

# Integration tests (requires cloud credentials)
export AWS_REGION=us-east-1
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test

cargo test -p cloudkit_core --test aws_integration
# Expected: test result: ok (or skipped if no credentials)
```

### Cloud SDK Verification

```bash
# Verify AWS SDK
cargo tree -p cloudkit_core | grep aws-sdk
# Expected: aws-sdk-s3, aws-sdk-dynamodb, etc.

# Verify Azure SDK
cargo tree -p cloudkit_core | grep azure
# Expected: azure_storage, azure_identity

# Verify GCP SDK
cargo tree -p cloudkit_core | grep google-cloud
# Expected: google-cloud-storage
```

### Provider Test

```bash
# Create provider test
cat > crates/cloudkit_core/examples/provider_test.rs << 'EOF'
use cloudkit_core::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Try to initialize AWS provider
    let aws = AwsProvider::from_env().await?;
    println!("AWS Provider initialized");
    
    // Try to initialize Azure provider
    let azure = AzureProvider::from_env().await?;
    println!("Azure Provider initialized");
    
    Ok(())
}
EOF

cargo run --example provider_test -p cloudkit_core
# Expected: 
# AWS Provider initialized
# Azure Provider initialized
```

## Cloud Credentials Setup

### AWS

```bash
# Configure AWS credentials
aws configure
# Or set environment variables:
export AWS_REGION=us-east-1
export AWS_ACCESS_KEY_ID=your_key_id
export AWS_SECRET_ACCESS_KEY=your_secret_key
```

### Azure

```bash
# Login via Azure CLI
az login

# Or use environment variables:
export AZURE_TENANT_ID=your_tenant_id
export AZURE_CLIENT_ID=your_client_id
export AZURE_CLIENT_SECRET=your_client_secret
```

### GCP

```bash
# Login via gcloud
gcloud auth application-default login

# Or use service account:
export GOOGLE_APPLICATION_CREDENTIALS=/path/to/service-account.json
```

## Troubleshooting

### Build Issues

**Problem**: AWS SDK compilation errors  
**Solution**: Update AWS SDK versions in Cargo.toml to latest compatible set

**Problem**: Azure SDK authentication failures  
**Solution**: Run `az login` or set environment variables correctly

### Test Issues

**Problem**: Integration tests fail with "no credentials"  
**Solution**: Run unit tests only: `cargo test --lib -p cloudkit_core`

**Problem**: Tests timeout  
**Solution**: Increase test timeout: `cargo test -- --test-threads=1 --nocapture`

---

**Last Updated**: 2026-01-14
