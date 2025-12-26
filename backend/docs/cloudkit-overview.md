# CloudKit Overview

## What is CloudKit?

**CloudKit** is a unified multi-cloud SDK for Rust that provides a single, consistent API for interacting with multiple cloud providers. Instead of learning and maintaining separate SDKs for AWS, Azure, GCP, and Oracle Cloud, developers can use CloudKit's unified interface and deploy to any cloud.

## Problem Statement

Modern applications often need to:
- Support multiple cloud providers for vendor flexibility
- Migrate between clouds without rewriting code
- Use the best services from different providers
- Meet regulatory requirements for data residency

Without a unified SDK, developers must:
- Learn multiple SDK APIs
- Maintain separate codepaths for each cloud
- Handle different error types and patterns
- Duplicate testing across providers

## Solution

CloudKit provides:

```
┌──────────────────────────────────────────────────────────────────┐
│                     Your Application Code                         │
│                                                                    │
│    storage.put_object("bucket", "key", data).await?               │
│                                                                    │
├──────────────────────────────────────────────────────────────────┤
│                         CloudKit                                   │
│                                                                    │
│    ┌─────────────┬─────────────┬─────────────┬─────────────┐     │
│    │ ObjectStorage│KeyValueStore│MessageQueue │   PubSub   │     │
│    │   trait     │    trait    │    trait    │   trait    │     │
│    └─────────────┴─────────────┴─────────────┴─────────────┘     │
│                                                                    │
├──────────────────────────────────────────────────────────────────┤
│                      Provider Implementations                      │
│                                                                    │
│    ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌───────────┐       │
│    │    AWS    │ │   Azure   │ │    GCP    │ │  Oracle   │       │
│    │    S3     │ │   Blob    │ │    GCS    │ │  Object   │       │
│    │ DynamoDB  │ │  Cosmos   │ │ Firestore │ │  NoSQL    │       │
│    │   SQS     │ │  Service  │ │  Pub/Sub  │ │ Streaming │       │
│    │   SNS     │ │   Bus     │ │           │ │           │       │
│    │  Lambda   │ │ Functions │ │ Functions │ │ Functions │       │
│    └───────────┘ └───────────┘ └───────────┘ └───────────┘       │
└──────────────────────────────────────────────────────────────────┘
```

## Core Principles

### 1. Provider Agnostic

Write code that works with any cloud:

```rust
// This function works with AWS, Azure, GCP, or Oracle
async fn backup<S: ObjectStorage>(storage: &S, data: &[u8]) -> CloudResult<()> {
    storage.put_object("backup-bucket", "data.bin", data).await
}
```

### 2. Type Safe

Leverage Rust's type system for compile-time safety:

```rust
// Wrong provider operations caught at compile time
let aws = CloudKit::aws().build().await?;
// aws.cosmos()  // ← Compile error! Cosmos is Azure-only
```

### 3. Async First

Built on Tokio for high-performance async operations:

```rust
// Concurrent uploads
let (result1, result2) = tokio::join!(
    storage.put_object("bucket", "file1", data1),
    storage.put_object("bucket", "file2", data2),
);
```

### 4. Extensible

Use SPIs to customize behavior:

```rust
// Custom authentication (e.g., HashiCorp Vault)
struct VaultAuth { /* ... */ }
impl AuthProvider for VaultAuth { /* ... */ }

// Custom retry policy
struct CircuitBreaker { /* ... */ }
impl RetryPolicy for CircuitBreaker { /* ... */ }
```

## Architecture (SEA)

CloudKit uses **Stratified Encapsulation Architecture** with five layers:

### Layer 1: Common
Shared types and utilities with no dependencies on other layers.
- `CloudError` - Unified error type
- `CloudResult<T>` - Result alias
- `Region` - Cloud region definitions
- `Credentials` - Authentication credentials
- `CloudConfig` - Configuration options

### Layer 2: SPI (Service Provider Interface)
Extension points for customizing SDK behavior.
- `AuthProvider` - Custom authentication
- `RetryPolicy` - Custom retry strategies
- `MetricsCollector` - Observability integration
- `Logger` - Custom logging

### Layer 3: API
Service contracts that define cloud operations.
- `ObjectStorage` - Blob/object storage
- `KeyValueStore` - NoSQL operations
- `MessageQueue` - Queue operations
- `PubSub` - Publish-subscribe messaging
- `Functions` - Serverless invocation

### Layer 4: Core
Internal implementations and utilities.
- `CloudContext` - Shared client context
- `OperationExecutor` - Retry and metrics wrapper
- `ProviderType` - Provider enumeration

### Layer 5: Facade
Public API entry points.
- `CloudKit` - Main entry point
- `prelude` - Convenient re-exports

## Supported Services

| Service | AWS | Azure | GCP | Oracle |
|---------|-----|-------|-----|--------|
| Object Storage | S3 | Blob Storage | Cloud Storage | Object Storage |
| Key-Value Store | DynamoDB | Cosmos DB | Firestore | NoSQL |
| Message Queue | SQS | Service Bus | Cloud Tasks | Streaming |
| Pub/Sub | SNS | Event Grid | Pub/Sub | - |
| Functions | Lambda | Functions | Cloud Functions | Functions |

## Use Cases

### Multi-Cloud Strategy
Deploy the same application to multiple clouds for redundancy or regional compliance.

### Cloud Migration
Migrate from one cloud to another without rewriting application code.

### Best-of-Breed
Use specific services from different providers (e.g., AWS S3 + Azure Cosmos DB).

### Disaster Recovery
Replicate data across clouds for business continuity.

### Vendor Negotiation
Avoid vendor lock-in and maintain negotiating leverage.

## Project Structure

```
cloudkit/
├── crates/
│   ├── cloudkit/           # Core library
│   │   └── src/
│   │       ├── common/     # Shared types
│   │       ├── spi/        # Extension points
│   │       ├── api/        # Service traits
│   │       ├── core/       # Implementations
│   │       └── facade/     # Public API
│   ├── cloudkit-aws/       # AWS provider
│   ├── cloudkit-azure/     # Azure provider
│   ├── cloudkit-gcp/       # GCP provider
│   └── cloudkit-oracle/    # Oracle provider
├── docs/                   # Documentation
└── examples/               # Usage examples
```

## Getting Started

See [Getting Started Guide](getting-started.md) for installation and first steps.

## Related Documentation

- [Architecture Details](architecture.md)
- [Configuration](configuration.md)
- [Error Handling](error-handling.md)
- [Provider Guides](providers/README.md)
- [WASM Support](wasm.md)
