# CloudKit Architecture

## Overview

CloudKit is a multi-cloud SDK built using **Stratified Encapsulation Architecture (SEA)**, providing a unified interface for interacting with AWS, Azure, GCP, and Oracle Cloud.

## SEA Layers

```
┌─────────────────────────────────────────────────────────────────┐
│                     FACADE (Public API)                          │
│  CloudKit::aws(), CloudKit::azure(), prelude::*                 │
├─────────────────────────────────────────────────────────────────┤
│                     CORE (Implementations)                       │
│  CloudContext, OperationExecutor, Provider Clients              │
├─────────────────────────────────────────────────────────────────┤
│                     API (Contracts)                              │
│  ObjectStorage, KeyValueStore, MessageQueue, PubSub, Functions  │
├─────────────────────────────────────────────────────────────────┤
│                     SPI (Extension Points)                       │
│  AuthProvider, RetryPolicy, MetricsCollector, Logger            │
├─────────────────────────────────────────────────────────────────┤
│                     COMMON (Shared)                              │
│  CloudError, Region, Credentials, Config, Types                 │
└─────────────────────────────────────────────────────────────────┘
```

## Layer Details

### 1. Common Layer (`cloudkit::common`)

Foundation layer with shared types used across all other layers.

**Contents:**
- `CloudError` - Unified error type
- `CloudResult<T>` - Result type alias
- `Region` - Cloud region definitions
- `Credentials` - Authentication credentials
- `CloudConfig` - Configuration options
- `ResourceId`, `ObjectMetadata`, `BucketMetadata` - Data types

**Principles:**
- No dependencies on other CloudKit layers
- Pure data types and utilities
- Serializable with serde

### 2. SPI Layer (`cloudkit::spi`)

Service Provider Interface for extending SDK behavior.

**Extension Points:**
- `AuthProvider` - Custom authentication (Vault, AWS IAM, OIDC)
- `RetryPolicy` - Custom retry strategies
- `MetricsCollector` - Observability integration (Prometheus, DataDog)
- `Logger` - Custom logging

**Principles:**
- Traits only, no implementations (except defaults)
- Users implement these to customize behavior
- Dependency injection via builder pattern

### 3. API Layer (`cloudkit::api`)

Service contracts defining cloud operations.

**Service Traits:**
- `ObjectStorage` - Blob storage (S3, Blob, GCS, OCI Object Storage)
- `KeyValueStore` - NoSQL (DynamoDB, Cosmos, Firestore)
- `MessageQueue` - Queues (SQS, Service Bus, Cloud Tasks)
- `PubSub` - Topics (SNS, Event Grid, Pub/Sub)
- `Functions` - Serverless (Lambda, Azure Functions, Cloud Functions)

**Principles:**
- Provider-agnostic interfaces
- Async-first design
- Comprehensive options via builder pattern

### 4. Core Layer (`cloudkit::core`)

Internal implementations and utilities.

**Contents:**
- `CloudContext` - Shared client context
- `OperationExecutor` - Retry and metrics wrapper
- `ProviderType` - Provider enumeration

**Principles:**
- Not part of public API stability guarantee
- Provider crates depend on this
- Contains shared implementation details

### 5. Facade Layer (`cloudkit::facade`)

Public entry points for the SDK.

**Contents:**
- `CloudKit` - Main entry point
- Provider builders (`AwsBuilder`, `AzureBuilder`, etc.)

**Principles:**
- Stable public API
- Feature-gated provider support
- Ergonomic builder pattern

## Provider Crates

Each cloud provider has its own crate:

| Crate | Provider | Services |
|-------|----------|----------|
| `cloudkit-aws` | AWS | S3, DynamoDB, SQS, SNS, Lambda |
| `cloudkit-azure` | Azure | Blob Storage, Cosmos DB, Service Bus |
| `cloudkit-gcp` | GCP | Cloud Storage, Pub/Sub, BigQuery |
| `cloudkit-oracle` | OCI | Object Storage, Streaming, Functions |

## Dependency Flow

```
┌──────────────────────────────────────────────────────────────────┐
│                        User Application                           │
└──────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│                           cloudkit                                │
│  (facade + api + spi + common + core)                            │
└──────────────────────────────────────────────────────────────────┘
          │              │              │              │
          ▼              ▼              ▼              ▼
    ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
    │cloudkit- │  │cloudkit- │  │cloudkit- │  │cloudkit- │
    │   aws    │  │  azure   │  │   gcp    │  │  oracle  │
    └──────────┘  └──────────┘  └──────────┘  └──────────┘
          │              │              │              │
          ▼              ▼              ▼              ▼
    ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
    │ AWS SDK  │  │Azure SDK │  │ GCP SDK  │  │ OCI SDK  │
    └──────────┘  └──────────┘  └──────────┘  └──────────┘
```

## Design Decisions

### 1. Workspace Structure

Using Cargo workspace for:
- Shared dependencies via `workspace.dependencies`
- Independent versioning per crate
- Parallel compilation
- Feature isolation

### 2. Async-First

All operations are async:
- Built on Tokio runtime
- Non-blocking I/O
- Efficient resource usage

### 3. Builder Pattern

Extensive use of builders for:
- Optional configuration
- Fluent API
- Type-safe construction

### 4. Feature Flags

Granular feature flags for:
- Minimizing binary size
- Conditional compilation
- Provider selection

## Error Handling

Unified error type with:
- Provider-specific details
- Retryable error detection
- Structured error information

```rust
pub enum CloudError {
    Auth(AuthError),
    Network(NetworkError),
    NotFound { resource_type, resource_id },
    RateLimited { retry_after },
    Provider { provider, code, message },
    // ...
}
```

## Observability

Built-in support for:
- **Tracing** - Via `tracing` crate
- **Metrics** - Via `MetricsCollector` SPI
- **Logging** - Via `Logger` SPI

## Testing Strategy

1. **Unit Tests** - Per-module tests
2. **Integration Tests** - Provider-specific
3. **Mock Support** - Via traits and `mockall`
4. **Wiremock** - HTTP mocking for SDK testing
