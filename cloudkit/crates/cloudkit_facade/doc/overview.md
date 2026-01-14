# CloudKit Facade Crate

## WHAT: The Unified Entry Point

`cloudkit_facade` is the public-facing layer (Layer 5) of the CloudKit SDK. It provides the `CloudKit` struct and common preludes that allow users to interact with any supported cloud provider through a single, consistent API.

**Prerequisites**:
- Rust `1.85+`
- Configured cloud credentials (e.g., Environment variables or ~/.aws/credentials).

## WHY: Developer Ergonomics

### Problems Solved
- **Complex Initialization**: Hiding the complexity of configuring multiple provider clients.
- **API Discovery**: Providing a single hub (`CloudKit`) to access Storage, Queues, Databases, and more.

## HOW: Usage Example

```rust
use cloudkit::prelude::*;

#[tokio::main]
async fn main() -> Result<(), CloudError> {
    let cloud = CloudKit::aws().build().await?;
    
    cloud.storage().put_object("bucket", "key", b"data").await?;
    
    Ok(())
}
```

## Examples and Tests
- **Integration Tests**: See `tests/` for full stack validation.
- **Examples**: Root `examples/` directory contains numerous usage scenarios.

---

**Last Updated**: 2026-01-14
