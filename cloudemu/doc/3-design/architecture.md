# CloudEmu Architecture

## Overview

CloudEmu implements a **Full Stratified Encapsulation Architecture (SEA)** across all providers and both control-plane and data-plane components. This results in a highly modular, consistent, and maintainable codebase.

## Architectural Principles

### 1. Stratified Encapsulation

Each provider's control-plane and data-plane is divided into 4 distinct layers:

```
Facade Layer    (HTTP routing, public API)
    ↓
Core Layer      (Business logic, service implementations)
    ↓
API Layer       (Traits, service contracts)
    ↓
SPI Layer       (Foundation types, errors, base traits)
```

### 2. Provider Isolation

Each provider (AWS, Azure, GCP) has its own complete stack, ensuring:
- No cross-provider contamination
- Independent versioning capability
- Clear provider-specific logic separation

### 3. Plane Separation

**Control-Plane**: Handles API requests, service emulation, routing
**Data-Plane**: Manages persistence, storage, data retrieval

This separation allows:
- Independent scaling of API vs storage
- Different optimization strategies per plane
- Clear architectural boundaries

## Crate Structure

### Provider Crates (24 total: 8 per provider)

#### AWS Example

**Control-Plane** (4 crates):
```
aws-control-spi    → Foundation (types, errors)
aws-control-api    → Service traits (S3Service, DynamoDBService, etc.)
aws-control-core   → Service implementations (actual emulation logic)
aws-control-facade → HTTP routing (Axum handlers)
```

**Data-Plane** (4 crates):
```
aws-data-spi    → Storage foundation types
aws-data-api    → Storage service traits
aws-data-core   → Storage implementations
aws-data-facade → Storage API endpoints
```

Azure and GCP follow identical patterns.

### Global Crates (5 total)

```
cloudemu_spi    → Global foundation (shared types, base traits)
cloudemu_api    → Global service contracts
cloudemu_core   → Provider orchestration (feature flags, re-exports)
cloudemu_server → HTTP server runtime
data-plane      → Shared storage engine (SQLite-based)
```

## Dependency Flow

### Vertical (Within a Provider)

```
facade
  ↓ depends on
core
  ↓ depends on
api
  ↓ depends on
spi
  ↓ depends on
cloudemu_spi (global)
```

### Horizontal (Across Providers)

Providers are **independent**. No provider directly depends on another.

```
aws/        azure/      gcp/
  ↓           ↓           ↓
       cloudemu_spi (shared)
```

### Server Integration

```
cloudemu_server
  ↓ depends on
aws-control-facade, azure-control-facade, gcp-control-facade
  ↓ which depend on
respective core implementations
```

## Layer Responsibilities

### SPI Layer (Foundation)

**Purpose**: Provide foundation types and base traits

**Contents**:
- Re-exports from `cloudemu_spi`
- Provider-specific error types
- Provider-specific base types
- Extension traits

**Example** (`aws-control-spi/src/lib.rs`):
```rust
pub use cloudemu_spi::*;

pub mod types {
    // AWS-specific types
}

pub mod error {
    use thiserror::Error;
    
    #[derive(Error, Debug)]
    pub enum AwsControlError {
        #[error("AWS control error: {0}")]
        Generic(String),
    }
}
```

### API Layer (Contracts)

**Purpose**: Define service contracts as traits

**Contents**:
- Service traits (e.g., `S3Service`, `DynamoDBService`)
- Request/response types
- Service-specific errors

**Example** (`aws-control-api/src/s3.rs`):
```rust
#[async_trait]
pub trait S3Service {
    async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>) 
        -> Result<(), String>;
    async fn get_object(&self, bucket: &str, key: &str) 
        -> Result<Vec<u8>, String>;
}
```

### Core Layer (Implementation)

**Purpose**: Implement service logic

**Contents**:
- Service implementations
- Business logic
- Service modules (s3/, dynamodb/, sqs/, etc.)
- Adapters and utilities

**Example** (aws-control-core structure):
```
aws-control-core/
├── src/
│   ├── lib.rs
│   ├── services/
│   │   ├── s3/
│   │   │   ├── mod.rs
│   │   │   ├── service.rs
│   │   │   ├── handlers.rs
│   │   │   └── xml.rs
│   │   ├── dynamodb/
│   │   ├── sqs/
│   │   └── ...
│   ├── gateway/
│   │   ├── dispatcher.rs
│   │   └── router.rs
│   └── adapters/
```

### Facade Layer (Public API)

**Purpose**: Expose HTTP API and orchestrate core

**Contents**:
- Axum HTTP routing
- Request parsing
- Response formatting
- Public API endpoints

**Example** (`aws-control-facade/src/lib.rs`):
```rust
pub use aws_control_spi;
pub use aws_control_api;
pub use aws_control_core;

// Re-export gateway/routing
pub use aws_control_core::gateway;
```

## Data Flow

### Request Flow (Control-Plane)

```
1. HTTP Request → cloudemu_server
2. Router → aws-control-facade
3. Dispatcher → aws-control-core
4. Service Implementation → aws-control-api trait
5. Storage Access → data-plane
6. Response → HTTP Response
```

### Storage Flow (Data-Plane)

```
1. Storage Request → aws-data-facade
2. Storage Logic → aws-data-core
3. Storage Trait → aws-data-api
4. Shared Engine → data-plane (SQLite)
```

## Feature Flags

### Core Layer Features

Each provider's core supports service-specific features:

```toml
[features]
default = ["s3"]
s3 = []
dynamodb = []
sqs = []
sns = []
lambda = []
full = ["s3", "dynamodb", "sqs", "sns", "lambda"]
```

### Orchestration Layer Features

`cloudemu_core` orchestrates provider selection:

```toml
[features]
default = []
aws = ["dep:aws-control-facade"]
azure = ["dep:azure-control-facade"]
gcp = ["dep:gcp-control-facade"]
full = ["aws", "azure", "gcp"]
```

## Benefits

### Modularity
- Each crate has ~single responsibility
- Easy to test individual components
- Clear boundaries reduce cognitive load

### Consistency
- All providers follow identical structure
- Switching providers requires same mental model
- Predictable code organization

### Extensibility
- Add new services by extending API + Core
- Add new providers by replicating structure
- Add new layers without disrupting existing ones

### Type Safety
- Full Rust type system across all layers
- Compile-time verification of contracts
- No runtime surprises

### Performance
- Granular compilation units
- Feature flags reduce binary size
- Clear hot paths for optimization

## Comparison with Alternatives

### vs. Monolithic Design
```
Monolithic: All code in one crate
SEA: 24 crates, clear boundaries
→ SEA wins on maintainability, loses slightly on compile time
```

### vs. Partial Layering
```
Partial: Only facade + core
Full SEA: Facade + core + API + SPI
→ Full SEA wins on clarity, extensibility
```

### vs. Per-Service Crates
```
Per-Service: s3/, dynamodb/, sqs/ as separate crates
SEA: Services within provider core
→ SEA wins on provider cohesion
```

## Migration Path

### From Old Structure
```
OLD: cloudemu/crates/cloudemu_core/aws/control-plane/
NEW: cloudemu/aws/control-plane/aws-control-core/
```

### Import Changes
```rust
// OLD
use cloudemu_spi::CloudError;

// NEW
use aws_control_spi::CloudError;
```

## Future Enhancements

### Potential Additions
1. **Observability Layer** - Monitoring and tracing
2. **Plugin System** - Dynamic service loading
3. **Multi-Region** - Regional emulation
4. **Chaos Engineering** - Failure injection

### Versioning Strategy
- Per-crate semantic versioning
- Provider independence allows breaking changes
- Global SPI versioning for cross-provider compatibility

---

**Version**: 1.0  
**Last Updated**: 2026-01-15  
**Authors**: SWE Engineering Labs
