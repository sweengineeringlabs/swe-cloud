# CloudEmu Crates

CloudEmu follows the **Stratified Encapsulation Architecture (SEA)**, mirroring the CloudKit SDK structure for consistency.

## Architecture Overview

```
cloudemu/crates/
├── cloudemu_spi/       Foundation Layer - Core types, errors, traits
├── cloudemu_api/       API Layer - Service contracts and trait definitions
├── cloudemu_core/      Orchestration Layer - Provider implementations
│   ├── aws/           (cloudemu-aws) AWS emulation
│   ├── azure/         (cloudemu-azure) Azure emulation  
│   └── gcp/           (cloudemu-gcp) GCP emulation
├── cloudemu_server/    Server Layer - HTTP server and runtime
└── data-plane/         Storage engine for resource persistence
```

## Layer Responsibilities

### 1. `cloudemu_spi` (Service Provider Interface)
- **Purpose**: Foundation types and extension points
- **Provides**: 
  - Core error types (`CloudError`)
  - Provider traits (`CloudProviderTrait`)
  - Request/Response types
  - Service type enums
- **Dependencies**: Pure (no internal dependencies)

### 2. `cloudemu_api` (API Layer)
- **Purpose**: Service contract definitions
- **Provides**: 
  - Storage service traits
  - Database service traits
  - Messaging service traits
- **Dependencies**: `cloudemu_spi`

### 3. `cloudemu_core` (Core Orchestration)
- **Purpose**: Provider orchestration and feature flags
- **Provides**: 
  - Re-exports of provider crates
  - Feature-gated provider selection
- **Dependencies**: `cloudemu_spi`, `cloudemu_api`, provider crates (optional)

#### Provider Crates (nested in `cloudemu_core/`)

**`cloudemu-aws`** (AWS Provider)
- S3, DynamoDB, SQS, SNS, Lambda, Secrets Manager, KMS, EventBridge, CloudWatch, Cognito, Step Functions
- Default port: `4566`

**`cloudemu-azure`** (Azure Provider)
- Blob Storage, Cosmos DB, Service Bus, Functions
- Default port: `4567`

**`cloudemu-gcp`** (GCP Provider)
- Cloud Storage, Firestore, Pub/Sub, Cloud Functions
- Default port: `4568`

### 4. `cloudemu_server` (Server/Facade)
- **Purpose**: HTTP server runtime and multi-provider orchestration
- **Provides**: 
  - Main server binary
  - Provider routing
  - Configuration management
- **Dependencies**: All layers

### 5. `data-plane` (Storage Engine)
- **Purpose**: Persistent storage for emulated resources
- **Provides**: 
  - File-based resource storage
  - Query and indexing
  - Transaction support
- **Dependencies**: Standalone (workspace-level)

## Feature Flags

The `cloudemu_core` crate uses feature flags to enable specific providers:

```toml
[dependencies]
cloudemu_core = { version = "0.2", features = ["aws", "azure"] }
```

Available features:
- `aws` - Enable AWS emulation
- `azure` - Enable Azure emulation
- `gcp` - Enable GCP emulation
- `full` - Enable all providers

## Usage

### Running the Server

```bash
# All providers (default)
cargo run -p cloudemu_server

# Specific providers
cargo run -p cloudemu_server -- --enable-aws --enable-azure

# Custom ports
cargo run -p cloudemu_server -- --aws-port 4566 --azure-port 4567
```

### Using as a Library

```rust
use cloudemu_spi::{CloudProviderTrait, Request};
use cloudemu_aws::AwsProvider;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let provider = Arc::new(AwsProvider::new(/* config */));
    
    let req = Request {
        method: "GET".to_string(),
        path: "/health".to_string(),
        headers: Default::default(),
        body: vec![],
    };
    
    let response = provider.handle_request(req).await.unwrap();
    println!("Status: {}", response.status);
}
```

## Comparison with CloudKit

CloudEmu's architecture mirrors CloudKit for consistency:

| CloudKit        | CloudEmu          | Purpose |
|----------------|-------------------|---------|
| `cloudkit_spi` | `cloudemu_spi`    | Foundation types |
| `cloudkit_api` | `cloudemu_api`    | Service traits |
| `cloudkit_core`| `cloudemu_core`   | Orchestration |
| `cloudkit_facade` | `cloudemu_server` | Public API |
| Provider crates | Provider crates  | Implementation |

This alignment makes it easy to understand both codebases and maintain consistency between the SDK and emulator.
