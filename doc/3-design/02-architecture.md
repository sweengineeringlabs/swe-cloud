# 02 - Architecture Design

## Document Information

| Field | Value |
|-------|-------|
| **Version** | 1.0.0 |
| **Status** | Design Complete |
| **Last Updated** | 2025-12-26 |

---

## 1. Architectural Overview

CloudKit uses **Stratified Encapsulation Architecture (SEA)**, a layered architecture pattern that promotes separation of concerns and extensibility.

### Layer Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                  │
│                     LAYER 5: FACADE                              │
│                                                                  │
│   Public API surface, entry points, prelude                      │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │  CloudKit::aws()  │  CloudKit::azure()  │  prelude::*   │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│                     LAYER 4: CORE                                │
│                                                                  │
│   Default implementations, internal utilities                    │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │  CloudContext  │  OperationExecutor  │  ProviderType    │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│                     LAYER 3: API                                 │
│                                                                  │
│   Service contracts (traits) for cloud operations               │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │ ObjectStorage │ KeyValueStore │ MessageQueue │ PubSub   │   │
│   │ Functions     │               │              │          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│                     LAYER 2: SPI                                 │
│                                                                  │
│   Service Provider Interface - Extension points                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │ AuthProvider │ RetryPolicy │ MetricsCollector │ Logger  │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│                     LAYER 1: COMMON                              │
│                                                                  │
│   Shared types, errors, utilities                                │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │ CloudError │ Region │ Credentials │ CloudConfig │ Types │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. Layer Details

### Layer 1: Common

**Purpose**: Foundation layer with shared types used across all other layers.

**Contents**:
```
common/
├── mod.rs          # Module exports
├── error.rs        # CloudError, AuthError, NetworkError
├── config.rs       # CloudConfig, Credentials
├── region.rs       # Region definitions
└── types.rs        # ResourceId, ObjectMetadata, etc.
```

**Dependencies**: None (foundation layer)

**Key Types**:

| Type | Description |
|------|-------------|
| `CloudError` | Unified error enum for all operations |
| `CloudResult<T>` | Result type alias |
| `Region` | Cloud region identifier |
| `Credentials` | Authentication credentials |
| `CloudConfig` | Configuration options |

### Layer 2: SPI (Service Provider Interface)

**Purpose**: Extension points for customizing SDK behavior.

**Contents**:
```
spi/
├── mod.rs          # Module exports
├── auth.rs         # AuthProvider trait
├── retry.rs        # RetryPolicy trait
├── metrics.rs      # MetricsCollector trait
└── logger.rs       # Logger trait
```

**Dependencies**: Common

**Key Traits**:

| Trait | Purpose | Default Implementation |
|-------|---------|------------------------|
| `AuthProvider` | Custom authentication | `EnvAuthProvider` |
| `RetryPolicy` | Custom retry strategies | `ExponentialBackoff` |
| `MetricsCollector` | Observability | `NoopMetrics` |
| `Logger` | Logging | `TracingLogger` |

### Layer 3: API

**Purpose**: Service contracts that define cloud operations.

**Contents**:
```
api/
├── mod.rs              # Module exports
├── object_storage.rs   # ObjectStorage trait
├── kv_store.rs         # KeyValueStore trait
├── message_queue.rs    # MessageQueue trait
├── pubsub.rs           # PubSub trait
└── functions.rs        # Functions trait
```

**Dependencies**: Common

**Key Traits**:

| Trait | Implements | Services |
|-------|------------|----------|
| `ObjectStorage` | Blob storage | S3, Blob, GCS |
| `KeyValueStore` | NoSQL | DynamoDB, Cosmos |
| `MessageQueue` | Queues | SQS, Service Bus |
| `PubSub` | Messaging | SNS, Event Grid |
| `Functions` | Serverless | Lambda, Functions |

### Layer 4: Core

**Purpose**: Internal implementations and utilities.

**Contents**:
```
core/
├── mod.rs          # Module exports
├── client.rs       # CloudContext, ProviderType
└── executor.rs     # OperationExecutor
```

**Dependencies**: Common, SPI, API

**Key Types**:

| Type | Purpose |
|------|---------|
| `CloudContext` | Shared client configuration |
| `ProviderType` | Provider enumeration |
| `OperationExecutor` | Retry and metrics wrapper |

### Layer 5: Facade

**Purpose**: Public API entry points.

**Contents**:
```
facade/
├── mod.rs          # Module exports
└── cloudkit.rs     # CloudKit entry point
```

**Dependencies**: All layers

---

## 3. Dependency Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    Dependency Direction                          │
│                                                                  │
│                         ▲ Depends On                             │
│                         │                                        │
│   ┌─────────────────────┴─────────────────────┐                 │
│   │              FACADE                        │                 │
│   └─────────────────────┬─────────────────────┘                 │
│                         │                                        │
│   ┌─────────────────────┴─────────────────────┐                 │
│   │               CORE                         │                 │
│   └─────────────────────┬─────────────────────┘                 │
│                         │                                        │
│         ┌───────────────┼───────────────┐                       │
│         ▼               ▼               ▼                        │
│   ┌───────────┐   ┌───────────┐   ┌───────────┐                 │
│   │    API    │   │    SPI    │   │           │                 │
│   └─────┬─────┘   └─────┬─────┘   │           │                 │
│         │               │          │           │                 │
│         └───────────────┼──────────┘           │                 │
│                         ▼                                        │
│                  ┌───────────┐                                   │
│                  │  COMMON   │                                   │
│                  └───────────┘                                   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. Crate Structure

### Workspace Layout

```
cloudkit/
├── Cargo.toml                  # Workspace manifest
├── crates/
│   ├── cloudkit/               # Core library
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs          # Crate root
│   │       ├── prelude.rs      # Re-exports
│   │       ├── common/         # Layer 1
│   │       ├── spi/            # Layer 2
│   │       ├── api/            # Layer 3
│   │       ├── core/           # Layer 4
│   │       └── facade/         # Layer 5
│   │
│   ├── cloudkit-aws/           # AWS provider
│   ├── cloudkit-azure/         # Azure provider
│   ├── cloudkit-gcp/           # GCP provider
│   └── cloudkit-oracle/        # Oracle provider
│
├── doc/                        # Documentation
├── examples/                   # Usage examples
└── tests/                      # Integration tests
```

### Provider Crate Structure

```
cloudkit-aws/
├── Cargo.toml
└── src/
    ├── lib.rs          # Crate root, feature exports
    ├── builder.rs      # AwsBuilder, AwsClient
    ├── s3.rs           # S3Storage : ObjectStorage
    ├── dynamodb.rs     # DynamoDbStore : KeyValueStore
    ├── sqs.rs          # SqsQueue : MessageQueue
    ├── sns.rs          # SnsPubSub : PubSub
    └── lambda.rs       # LambdaFunctions : Functions
```

---

## 5. Design Decisions

### Decision 1: Workspace Structure

**Context**: How to organize multiple related crates

**Decision**: Use Cargo workspace with separate provider crates

**Rationale**:
- Users only pay for providers they use
- Parallel compilation
- Independent versioning possible
- Clearer dependency boundaries

### Decision 2: Trait-Based Abstraction

**Context**: How to abstract provider differences

**Decision**: Use Rust traits for service contracts

**Rationale**:
- Compile-time polymorphism
- Zero runtime cost
- Type-safe provider switching
- Extensible by users

### Decision 3: Async-First

**Context**: Sync vs async API

**Decision**: All operations are async

**Rationale**:
- Cloud operations are I/O bound
- Better resource utilization
- Native Tokio integration
- Standard in modern Rust

### Decision 4: Builder Pattern

**Context**: How to configure clients

**Decision**: Use builder pattern for construction

**Rationale**:
- Optional parameters without overloads
- Fluent API
- Compile-time validation
- Self-documenting code

---

## 6. Cross-Cutting Concerns

### Error Handling

```
┌─────────────────────────────────────────────────────────────────┐
│                       Error Flow                                 │
│                                                                  │
│   Provider Error ──► Conversion ──► CloudError ──► Application  │
│                                                                  │
│   AWS NoSuchKey  ──►            ──► NotFound    ──► Handled     │
│   Azure 404      ──►            ──► NotFound    ──► Handled     │
│   GCP NOT_FOUND  ──►            ──► NotFound    ──► Handled     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Retry Mechanism

```
┌─────────────────────────────────────────────────────────────────┐
│                      Retry Flow                                  │
│                                                                  │
│   Request ──► Execute ──► Success? ──► Return                   │
│                  │                                               │
│                  ▼ No                                            │
│              Retryable? ──► No ──► Return Error                 │
│                  │                                               │
│                  ▼ Yes                                           │
│              Max Attempts? ──► Yes ──► Return Error             │
│                  │                                               │
│                  ▼ No                                            │
│              Wait (backoff) ──► Execute (retry)                 │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Observability

```
┌─────────────────────────────────────────────────────────────────┐
│                    Observability Stack                           │
│                                                                  │
│   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│   │   Tracing   │  │   Metrics   │  │   Logging   │            │
│   │             │  │             │  │             │            │
│   │ Span start  │  │ Duration    │  │ Request ID  │            │
│   │ Span end    │  │ Status      │  │ Provider    │            │
│   │ Events      │  │ Retry count │  │ Operation   │            │
│   └─────────────┘  └─────────────┘  └─────────────┘            │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 7. Related Documents

- [01-overview.md](01-overview.md) - Project overview
- [03-api-design.md](03-api-design.md) - API contracts
- [07-spi-extensions.md](07-spi-extensions.md) - Extension points
