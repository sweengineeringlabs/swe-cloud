# CloudKit Architecture

## Overview

CloudKit is a multi-cloud SDK built using **Stratified Encapsulation Architecture (SEA)**, providing a unified interface for interacting with AWS, Azure, GCP, and Oracle Cloud.

## SEA Layers

```
┌─────────────────────────────────────────────────────────────────┐
│              FACADE (cloudkit crate)                            │
│  CloudKit::aws(), CloudKit::azure(), prelude::*                 │
├─────────────────────────────────────────────────────────────────┤
│              CORE (cloudkit_core crate)                         │
│  OperationExecutor, Provider Management                         │
├─────────────────────────────────────────────────────────────────┤
│              API (cloudkit_api crate)                           │
│  ObjectStorage, KeyValueStore, MessageQueue, PubSub, Functions  │
├─────────────────────────────────────────────────────────────────┤
│              SPI (cloudkit_spi crate)                           │
│  CloudContext, AuthProvider, RetryPolicy, MetricsCollector      │
├─────────────────────────────────────────────────────────────────┤
│              COMMON (cloudkit_spi crate)                        │
│  CloudError, Region, Credentials, Config, Types                 │
└─────────────────────────────────────────────────────────────────┘
```

## Layer Details

### 1. SPI & Common Layer (`cloudkit_spi`)

Foundation crate containing shared types, errors, configuration, AND extension points.

**Common Contents:**
- `CloudContext` - Context moved here (from Core) to allow deeper integration
- `CloudError` - Unified error type
- `CloudResult<T>` - Result type alias
- `Region` - Cloud region definitions
- `Credentials` - Authentication credentials
- `CloudConfig` - Configuration options

**SPI Extension Points:**
- `AuthProvider` - Custom authentication (Vault, AWS IAM, OIDC)
- `RetryPolicy` - Custom retry strategies
- `MetricsCollector` - Observability integration (Prometheus, DataDog)
- `Logger` - Custom logging

**Principles:**
- Standalone crate with minimal dependencies
- Pure data types and traits
- Serializable with serde

### 2. API Layer (`cloudkit_api`)

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

### 3. Core Layer (`cloudkit_core`)

Internal implementations and utilities.

**Contents:**
- `OperationExecutor` - Retry and metrics wrapper
- `ProviderType` - Provider enumeration

**Principles:**
- Not part of public API stability guarantee
- Provider crates depend on this
- Contains shared implementation details

### 4. Facade Layer (`cloudkit`)

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
                   (imports cloudkit facade)
                                 ▼
┌──────────────────────────────────────────────────────────────────┐
│                           cloudkit                                │
│                   (re-exports everything)                        │
└──────────────────────────────────────────────────────────────────┘
          │              │              │              │
          ▼              ▼              ▼              ▼
    ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
    │cloudkit- │  │cloudkit- │  │cloudkit- │  │cloudkit- │
    │   aws    │  │  azure   │  │   gcp    │  │  oracle  │
    └──────────┘  └──────────┘  └──────────┘  └──────────┘
          │              │              │              │
          │              │              │              │
          ▼              ▼              ▼              ▼
    ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
    │cloudkit_spi │ │cloudkit_api │ │cloudkit_spi │ │cloudkit_api │ (Foundation)
    └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘
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
