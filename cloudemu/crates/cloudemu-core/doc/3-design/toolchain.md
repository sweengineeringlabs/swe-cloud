# Toolchain

## Overview

CloudEmu Core provides shared types, traits, and abstractions for multi-cloud emulation. Development requires only Rust toolchain.

## Tools

### Rust Compiler

| | |
|---|---|
| **What** | Systems programming language compiler |
| **Version** | 1.70+ (recommended: 1.75+) |
| **Install** | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |

**Why we use it**: Core library requires async traits and advanced type system features.

**How we use it**:
```bash
cargo build -p cloudemu-core
cargo test -p cloudemu-core
```

### Async-Trait

| | |
|---|---|
| **What** | Procedural macro for async trait methods |
| **Version** | 0.1.80+ |
| **Install** | Automatic via Cargo |

**Why we use it**: Enables async methods in `CloudProviderTrait` and `StorageEngineTrait`.

**How we use it**:
```rust
#[async_trait]
pub trait CloudProviderTrait: Send + Sync {
    async fn handle_request(&self, req: Request) -> CloudResult<Response>;
}
```

### Serde

| | |
|---|---|
| **What** | Serialization/deserialization framework |
| **Version** | 1.0+ |
| **Install** | Automatic via Cargo |

**Why we use it**: Serialize cloud resource metadata and request/response bodies.

**How we use it**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}
```

### Chrono

| | |
|---|---|
| **What** | Date and time library |
| **Version** | 0.4+ |
| **Install** | Automatic via Cargo |

**Why we use it**: Timestamp cloud resources (created_at, modified_at).

**How we use it**:
```rust
pub struct CloudResource {
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}
```

## Version Matrix

| Tool | Minimum | Recommended | Purpose |
|------|---------|-------------|---------|
| Rust | 1.70 | 1.75+ | Core language |
| async-trait | 0.1.80 | Latest | Async traits |
| serde | 1.0 | Latest | Serialization |
| chrono | 0.4 | 0.4.42+ | Timestamps |

## Verification

### Build and Test

```bash
# Build library
cargo build -p cloudemu-core
# Expected: Finished dev

# Run unit tests
cargo test -p cloudemu-core
# Expected: test result: ok

# Check public API
cargo doc -p cloudemu-core --no-deps --open
```

### Integration Example

```bash
# Verify traits are usable
cd crates/cloudemu-core

# Create example implementation
cat > examples/basic_provider.rs << 'EOF'
use cloudemu_core::*;
use async_trait::async_trait;

struct MockProvider;

#[async_trait]
impl CloudProviderTrait for MockProvider {
    async fn handle_request(&self, _req: Request) -> CloudResult<Response> {
        Ok(Response::ok("Mock"))
    }
    
    fn supported_services(&self) -> Vec<ServiceType> { vec![] }
    fn default_port(&self) -> u16 { 8080 }
    fn provider_name(&self) -> &str { "mock" }
}

#[tokio::main]
async fn main() {
    let provider = MockProvider;
    let req = Request::get("/test");
    let res = provider.handle_request(req).await.unwrap();
    println!("Status: {}", res.status);
}
EOF

cargo run --example basic_provider
# Expected: Status: 200
```

---

**Last Updated**: 2026-01-14
