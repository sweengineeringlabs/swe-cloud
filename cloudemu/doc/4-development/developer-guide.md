# CloudEmu Developer Guide

## Getting Started

### Prerequisites

- Rust 1.85+
- Cargo workspace understanding
- Familiarity with async Rust (tokio)
- Understanding of HTTP/REST APIs

### Repository Structure

```
swe-cloud/
├── cloudemu/           Main emulator
│   ├── aws/           AWS provider (8 crates)
│   ├── azure/         Azure provider (8 crates)
│   ├── gcp/           GCP provider (8 crates)
│   └── crates/        Global crates (5 crates)
├── cloudkit/          SDK
└── iac/               Infrastructure examples
```

## Development Workflow

### 1. Building

```bash
# Build entire workspace
cargo build

# Build specific provider
cargo build -p aws-control-core

# Build with features
cargo build -p aws-control-core --features "s3,dynamodb,sqs"

# Build server
cargo build -p cloudemu_server
```

### 2. Running

```bash
# Run server (all providers)
cargo run -p cloudemu_server

# Run with specific providers
cargo run -p cloudemu_server -- --enable-aws

# Run with custom ports
cargo run -p cloudemu_server -- --aws-port 4566
```

### 3. Testing

```bash
# Test a specific crate
cargo test -p aws-control-core

# Test with features
cargo test -p aws-control-core --features full

# Integration tests
cargo test -p cloudemu_server --test integration

# All tests
cargo test --workspace
```

## Adding a New Service

### Example: Adding SES (Simple Email Service) to AWS

#### 1. Define API (`aws-control-api/src/ses.rs`)

```rust
use async_trait::async_trait;

#[async_trait]
pub trait SesService {
    async fn send_email(
        &self,
        from: &str,
        to: Vec<String>,
        subject: &str,
        body: &str
    ) -> Result<String, String>; // Returns message ID
}
```

#### 2. Implement Core (`aws-control-core/src/services/ses/`)

```rust
// service.rs
use async_trait::async_trait;
use aws_control_api::SesService;

pub struct SesServiceImpl {
    storage: Arc<DataPlane>,
}

#[async_trait]
impl SesService for SesServiceImpl {
    async fn send_email(
        &self, 
        from: &str, 
        to: Vec<String>,
        subject: &str, 
        body: &str
    ) -> Result<String, String> {
        let message_id = uuid::Uuid::new_v4().to_string();
        
        // Store email in data-plane
        self.storage.store_email(message_id.clone(), from, to, subject, body)
            .await?;
        
        Ok(message_id)
    }
}
```

#### 3. Add HTTP Handler (`aws-control-core/src/services/ses/handlers.rs`)

```rust
use axum::{Json, extract::State};

pub async fn send_email_handler(
    State(service): State<Arc<SesServiceImpl>>,
    Json(req): Json<SendEmailRequest>
) -> Result<Json<SendEmailResponse>, StatusCode> {
    let message_id = service.send_email(
        &req.from,
        req.to,
        &req.subject,
        &req.body
    ).await?;
    
    Ok(Json(SendEmailResponse { message_id }))
}
```

#### 4. Register Route (`aws-control-core/src/gateway/router.rs`)

```rust
pub fn create_router() -> Router {
    Router::new()
        .route("/ses/send-email", post(ses::handlers::send_email_handler))
        // ... other routes
}
```

#### 5. Update Feature Flag (`aws-control-core/Cargo.toml`)

```toml
[features]
ses = []
full = ["s3", "dynamodb", "sqs", "ses", ...]
```

#### 6. Add Tests (`aws-control-core/src/services/ses/tests.rs`)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_send_email() {
        let service = SesServiceImpl::new();
        
        let msg_id = service.send_email(
            "sender@example.com",
            vec!["recipient@example.com".to_string()],
            "Test Subject",
            "Test Body"
        ).await.unwrap();
        
        assert!(!msg_id.is_empty());
    }
}
```

## Adding a New Provider

### Example: Adding Oracle Cloud

#### 1. Create Directory Structure

```bash
mkdir -p cloudemu/oracle/control-plane
mkdir -p cloudemu/oracle/data-plane
```

#### 2. Create 8 Crates

Using the generator script:
```powershell
# Modify cloudemu/generate-sea-crates.ps1 to include "oracle"
$providers = @("aws", "azure", "gcp", "oracle")
```

Or manually:
```bash
# Control-plane
mkdir cloudemu/oracle/control-plane/oracle-control-spi
mkdir cloudemu/oracle/control-plane/oracle-control-api
mkdir cloudemu/oracle/control-plane/oracle-control-core
mkdir cloudemu/oracle/control-plane/oracle-control-facade

# Data-plane
mkdir cloudemu/oracle/data-plane/oracle-data-spi
mkdir cloudemu/oracle/data-plane/oracle-data-api
mkdir cloudemu/oracle/data-plane/oracle-data-core
mkdir cloudemu/oracle/data-plane/oracle-data-facade
```

#### 3. Follow Existing Patterns

Copy structure from AWS/Azure/GCP and adapt for Oracle services:
- Object Storage → `oracle-control-api/src/objectstorage.rs`
- Autonomous DB → `oracle-control-api/src/autonomousdb.rs`

#### 4. Update Workspace

Add to `Cargo.toml`:
```toml
members = [
    # ... existing
    "cloudemu/oracle/control-plane/oracle-control-spi",
    "cloudemu/oracle/control-plane/oracle-control-api",
    # ... etc
]
```

#### 5. Wire into Server

Update `cloudemu_server/src/main.rs`:
```rust
if config.enable_oracle {
    let provider = Arc::new(oracle_control_facade::OracleProvider::new());
    tasks.spawn(async move {
        start_provider_server(provider, config.oracle_port, "Oracle").await
    });
}
```

## Best Practices

### Code Organization

1. **One service per module** in core layer
2. **Clear separation** of concerns across layers
3. **Use preludes** for common imports
4. **Document public APIs** thoroughly

### Error Handling

```rust
// Use provider-specific errors in SPI
pub enum AwsControlError {
    NotFound { resource: String },
    Validation(String),
    Internal(String),
}

// Convert to HTTP responses in facade
impl IntoResponse for AwsControlError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound { resource } => 
                (StatusCode::NOT_FOUND, resource).into_response(),
            Self::Validation(msg) => 
                (StatusCode::BAD_REQUEST, msg).into_response(),
            Self::Internal(msg) => 
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}
```

### Testing Strategy

1. **Unit tests** in core layer (business logic)
2. **Integration tests** in facade layer (HTTP)
3. **End-to-end tests** in server crate
4. **Mock data-plane** for fast tests

Example:
```rust
// Unit test example
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_s3_put_object() {
        let storage = MockStorage::new();
        let service = S3Service::new(storage);
        
        service.put_object("bucket", "key", b"data")
            .await
            .unwrap();
    }
}
```

### Performance Considerations

1. **Use async throughout** - All I/O should be async
2. **Pool connections** - Reuse HTTP clients, DB connections
3. **Lazy initialization** - Don't load all services upfront
4. **Feature flags** - Only compile what's needed
5. **Profiling** - Use `cargo flamegraph` for hot spots

### Documentation

```rust
//! Module documentation at top of files

/// Detailed API documentation
/// 
/// # Examples
/// 
/// ```
/// let service = S3Service::new();
/// service.put_object("bucket", "key", b"data").await?;
/// ```
pub async fn put_object(&self, ...) -> Result<...> {
    // Implementation
}
```

## Debugging

### Logging

```rust
use tracing::{info, debug, warn, error};

#[tracing::instrument]
async fn put_object(&self, bucket: &str, key: &str) {
    debug!("Storing object in bucket: {}", bucket);
    // ...
}
```

Run with logging:
```bash
RUST_LOG=debug cargo run -p cloudemu_server
```

### Debugging Tips

1. **Use `dbg!()` macro** for quick debugging
2. **Enable backtraces**: `RUST_BACKTRACE=1`
3. **Use VS Code debugger** with CodeLLDB extension
4. **Check HTTP requests**: Use curl/Postman to test endpoints
5. **Inspect storage**: Check `data/` directory for persisted resources

## Common Issues

### Import Errors

**Problem**: `use cloudemu_spi::CloudError` doesn't work

**Solution**: Use provider-specific SPI
```rust
use aws_control_spi::CloudError; // ✅
```

### Path Errors

**Problem**: Can't find dependency

**Solution**: Check relative paths (3 levels up from provider crates)
```toml
cloudemu_spi = { path = "../../../crates/cloudemu_spi" }
```

### Feature Flag Issues

**Problem**: Service code not compiling

**Solution**: Enable features in tests
```rust
#[cfg(feature = "s3")]
mod s3_tests {
    // tests
}
```

## Resources

- [Architecture Documentation](../3-design/architecture.md)
- [Crates Overview](../../crates/README.md)
- [CloudKit SDK](../../../cloudkit/README.md)

---

**Happy Coding!** 🚀
