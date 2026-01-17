# cloudkit-zero Overview

> **Scope**: High-level overview only. Implementation details belong in [Provider Guide](../../../../docs/4-development/guide/provider-zero.md).

## Audience

Developers who need to use ZeroCloud as a provider within the CloudKit ecosystem.

## WHAT

`cloudkit-zero` is the implementation crate that maps CloudKit Service Traits (API Layer) to the ZeroCloud platform via the `zero-sdk`.

Key capabilities:
- **ZeroStore Integration** - Implements `ObjectStorage` trait.
- **ZeroDB Integration** - Implements `KeyValueStore` trait.
- **ZeroFunc Integration** - Implements `Functions` trait.
- **ZeroQueue Integration** - Implements `MessageQueue` trait.
- **ZeroID Integration** - Implements `IdentityProvider` trait.

## WHY

| Problem | Solution |
|---------|----------|
| Running CloudKit apps locally without public cloud costs | Use ZeroCloud as a local, high-performance backend |
| Keeping data on-premise for security/compliance | Leverage ZeroCloud's private infrastructures with CloudKit API |
| Testing multi-cloud logic in a controlled environment | ZeroCloud provides a predictable, native private cloud target |

## HOW

```rust
use cloudkit_zero::ZeroBuilder;
use cloudkit_api::ObjectStorage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = ZeroBuilder::new()
        .endpoint("http://localhost:8080")
        .build()
        .await?;
        
    let storage = client.storage();
    storage.create_bucket("my-local-bucket").await?;
    
    Ok(())
}
```

## Documentation

| Document | Description |
|----------|-------------|
| [Provider Zero Guide](../../../../docs/4-development/guide/provider-zero.md) | Full detailed configuration and usage |
| [Zero SDK README](../../../../../cloudemu/zero/sdk/zero-sdk-rust/README.md) | Native SDK documentation |
| [CloudKit Architecture](../../../../docs/3-design/architecture.md) | SEA Layer 4 Implementation details |

---

**Status**: Alpha (Initial Release)
