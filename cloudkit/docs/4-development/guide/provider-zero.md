# ZeroCloud Provider Guide

**Audience**: Developers deploying to high-performance private cloud or local hybrid environments.

## WHAT: Native Private Cloud Support

The ZeroCloud provider for CloudKit offers native, high-performance integration with the ZeroCloud platform. Unlike public cloud providers that require internet-bound API calls, ZeroCloud is designed for ultra-low latency local and private deployments.

**Scope**:
- Native mapping of CloudKit traits to ZeroCloud services.
- Local high-performance data plane orchestration.
- Seamless developer experience via the Zero SDK.

## WHY: When to choose ZeroCloud?

| Advantage | Benefit |
|-----------|---------|
| **Data Sovereignty** | Keep all data within your physical infrastructure. |
| **Ultra-low Latency** | Bypass public internet overhead for local workloads. |
| **No egress costs** | Unlimited data movement without per-GB billing. |
| **Hybrid Parity** | Use the same CloudKit code for logic, target ZeroCloud locally. |

## HOW: Usage and Configuration

### 1. Quick Start

Initialize the ZeroCloud provider via the `CloudKit` facade:

```rust
use cloudkit::prelude::*;

#[tokio::main]
async fn main() -> CloudResult<()> {
    let cloud = CloudKit::zero()
        .endpoint("http://localhost:8080")
        .build()
        .await?;

    // Interact with ZeroStore (Object Storage)
    cloud.storage().create_bucket("local-data").await?;
    
    Ok(())
}
```

### 2. Service Mapping

| CloudKit Trait | ZeroCloud Service | Notes |
|----------------|-------------------|-------|
| `ObjectStorage` | **ZeroStore** | S3-compatible, backed by high-speed FS. |
| `KeyValueStore` | **ZeroDB** | DynamoDB-compatible, SQL-backed NoSQL. |
| `Functions` | **ZeroFunc** | Native process execution for Python/Node. |
| `MessageQueue` | **ZeroQueue** | Persistent messaging with visibility timeouts. |
| `IdentityProvider` | **ZeroID** | Local IAM for users, groups, and policies. |
| `Networking` | **ZeroLB** | ALB-compatible reverse proxy data plane. |

### 3. Feature Flags

Ensure you have the `zero` feature enabled in your `cloudkit-core` or use `cloudkit-zero` directly:

```toml
[dependencies]
cloudkit = { version = "0.1", features = ["zero"] }
```

---

## Summary

ZeroCloud brings the power of multi-cloud engineering to your private infrastructure. By using the ZeroCloud provider, you can maintain a single codebase that scales from a local developer machine to a high-performance private data center, all while staying compatible with public cloud patterns.

**Key Takeaways**:
1. ZeroCloud is the "Private Cloud" option for CloudKit.
2. It maps native services (ZeroStore, ZeroDB, etc.) to universal CloudKit traits.
3. Perfect for hybrid architectures where local performance is critical.

---

**Related Documentation**:
- [ZeroCloud Documentation Hub](../../../../cloudemu/zero/docs/overview.md)
- [Providers Overview](./providers-overview.md)
- [Architecture Hub](../../3-design/architecture.md)

**Last Updated**: 2026-01-17  
**Version**: 0.1.0
