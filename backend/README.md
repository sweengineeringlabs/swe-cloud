# CloudKit - Multi-Cloud SDK for Rust

[![Rust](https://img.shields.io/badge/Rust-1.85+-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue)](LICENSE)
[![Architecture](https://img.shields.io/badge/Architecture-SEA-green)](docs/architecture.md)

A unified, type-safe Rust SDK for interacting with multiple cloud providers through a single, consistent API.

## 🌟 Features

- **Unified Interface** - Single API for AWS, Azure, GCP, and Oracle Cloud
- **Type-Safe** - Leverage Rust's type system for compile-time safety
- **Async-First** - Built on Tokio for high-performance async operations
- **SEA Architecture** - Clean, layered architecture for extensibility
- **Provider Agnostic** - Write once, deploy anywhere

## 📦 Supported Cloud Providers

| Provider | Status | Services |
|----------|--------|----------|
| **AWS** | 🟢 Active | S3, DynamoDB, SQS, SNS, Lambda |
| **Azure** | � Active | Blob Storage, Cosmos DB, Key Vault, Monitor, Event Grid, Identity, Service Bus |
| **GCP** | 🟡 In Progress | Cloud Storage, Pub/Sub, BigQuery |
| **Oracle** | 🔵 Planned | Object Storage, Autonomous DB, Streaming |

## 🏗️ Architecture (SEA - Stratified Encapsulation Architecture)

```
┌─────────────────────────────────────────────────────────────────┐
│                         FACADE LAYER                             │
│  Public API re-exports, unified entry points, prelude           │
├─────────────────────────────────────────────────────────────────┤
│                          CORE LAYER                              │
│  Provider implementations (AWS, Azure, GCP, Oracle)             │
├─────────────────────────────────────────────────────────────────┤
│                           API LAYER                              │
│  Internal contracts, service traits, request/response types     │
├─────────────────────────────────────────────────────────────────┤
│                           SPI LAYER                              │
│  Extension points: Auth, Retry, Logging, Metrics providers      │
├─────────────────────────────────────────────────────────────────┤
│                         COMMON LAYER                             │
│  Shared types, errors, utilities, configuration                 │
└─────────────────────────────────────────────────────────────────┘
```

### Layer Responsibilities

| Layer | Purpose | Examples |
|-------|---------|----------|
| **Common** | Shared utilities and types | `CloudError`, `Region`, `Credentials` |
| **SPI** | Extension points for customization | `AuthProvider`, `RetryPolicy`, `MetricsCollector` |
| **API** | Service contracts (traits) | `ObjectStorage`, `MessageQueue`, `KeyValueStore` |
| **Core** | Provider implementations | `AwsS3`, `AzureBlob`, `GcsStorage` |
| **Facade** | Public API surface | `CloudKit::aws()`, `prelude::*` |

## 🚀 Quick Start

### Installation

Add CloudKit to your `Cargo.toml`:

```toml
[dependencies]
cloudkit = "0.1"

# Enable specific providers
cloudkit-aws = { version = "0.1", optional = true }
cloudkit-azure = { version = "0.1", optional = true }
cloudkit-gcp = { version = "0.1", optional = true }

[features]
default = ["aws"]
aws = ["cloudkit-aws"]
azure = ["cloudkit-azure"]
gcp = ["cloudkit-gcp"]
all-providers = ["aws", "azure", "gcp"]
```

### Basic Usage

```rust
use cloudkit::prelude::*;

#[tokio::main]
async fn main() -> Result<(), CloudError> {
    // Initialize with AWS
    let cloud = CloudKit::aws()
        .region(Region::UsEast1)
        .build()
        .await?;

    // Upload an object (same API for any provider)
    cloud.storage()
        .put_object("my-bucket", "hello.txt", b"Hello, Cloud!")
        .await?;

    // Download an object
    let data = cloud.storage()
        .get_object("my-bucket", "hello.txt")
        .await?;

    println!("Content: {}", String::from_utf8_lossy(&data));

    Ok(())
}
```

### Provider-Agnostic Code

```rust
use cloudkit::prelude::*;

// This function works with ANY cloud provider
async fn backup_data<S: ObjectStorage>(
    storage: &S,
    bucket: &str,
    key: &str,
    data: &[u8],
) -> Result<(), CloudError> {
    storage.put_object(bucket, key, data).await?;
    
    // Verify upload
    let metadata = storage.head_object(bucket, key).await?;
    tracing::info!("Uploaded {} bytes to {}/{}", metadata.size, bucket, key);
    
    Ok(())
}

// Use with different providers
async fn run() -> Result<(), CloudError> {
    let aws = CloudKit::aws().build().await?;
    let azure = CloudKit::azure().build().await?;
    
    backup_data(aws.storage(), "aws-bucket", "data.bin", b"data").await?;
    backup_data(azure.storage(), "azure-container", "data.bin", b"data").await?;
    
    Ok(())
}
```

## 📂 Project Structure

```
cloud/
├── Cargo.toml                    # Workspace configuration
├── README.md                     # This file
├── docs/
│   ├── architecture.md           # Detailed architecture docs
│   ├── providers/                # Provider-specific guides
│   └── examples/                 # Examples and tutorials
├── crates/
│   ├── cloudkit/                 # Core library
│   │   └── src/
│   │       ├── common/           # Shared types, errors, config
│   │       ├── spi/              # Extension points (traits)
│   │       ├── api/              # Service contracts
│   │       ├── core/             # Default implementations
│   │       ├── facade/           # Public API
│   │       ├── lib.rs            # Crate root
│   │       └── prelude.rs        # Convenient re-exports
│   ├── cloudkit-aws/             # AWS provider
│   ├── cloudkit-azure/           # Azure provider
│   ├── cloudkit-gcp/             # GCP provider
│   └── cloudkit-oracle/          # Oracle Cloud provider
└── examples/
    ├── basic_usage.rs
    ├── multi_cloud.rs
    └── custom_provider.rs
```

## 🔧 Configuration

### Environment Variables

```bash
# AWS
export AWS_ACCESS_KEY_ID=your-key
export AWS_SECRET_ACCESS_KEY=your-secret
export AWS_REGION=us-east-1

# Azure
export AZURE_STORAGE_ACCOUNT=your-account
export AZURE_STORAGE_KEY=your-key

# GCP
export GOOGLE_APPLICATION_CREDENTIALS=/path/to/credentials.json
export GCP_PROJECT_ID=your-project
```

### Programmatic Configuration

```rust
use cloudkit::prelude::*;

let config = CloudConfig::builder()
    .region(Region::UsEast1)
    .credentials(Credentials::from_env()?)
    .retry_policy(RetryPolicy::exponential(3))
    .timeout(Duration::from_secs(30))
    .build()?;

let cloud = CloudKit::aws().with_config(config).build().await?;
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with specific provider
cargo test --features aws

# Run integration tests (requires credentials)
cargo test --features integration

# Run with coverage
cargo llvm-cov --all-features
```

## 📖 Documentation

- [Architecture Guide](docs/architecture.md)
- [API Reference](https://docs.rs/cloudkit)
- [AWS Provider](docs/providers/aws.md)
- [Azure Provider](docs/providers/azure.md)
- [GCP Provider](docs/providers/gcp.md)
- [Examples](examples/)

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

**Built with ❤️ by PHD Systems**
