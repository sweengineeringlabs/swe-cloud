# Getting Started with CloudKit

**Audience**: Developers and DevOps Engineers new to CloudKit.

## WHAT: 5-Minute SDK Integration

CloudKit allows you to interact with multiple cloud providers using a single Rust SDK. This guide covers the installation, configuration, and basic usage of the library.

**Scope**:
- Basic Installation and Crates.
- Provider Configuration.
- Simple storage operations.
- Troubleshooting common errors.

## WHY: Unified Developer Experience

### Problems Addressed

1. **Fragmented Cloud SDKs**
   - Impact: Different APIs for S3, Blob, and GCS.
   - Consequence: Complex codebases that are hard to maintain.

2. **Configuration Overhead**
   - Impact: Managing separate credentials and client builders for each provider.
   - Consequence: Increased boilerplate and potential for misconfiguration.

### Benefits
- **Zero-Config Defaults**: Uses standard environment variables by default.
- **Portability**: Write once, deploy to any cloud.

## HOW: Step-by-Step Setup

### 1. Installation

Add the core and provider crates to your `Cargo.toml`:

```toml
[dependencies]
cloudkit = "0.1"
cloudkit-aws = "0.1"
tokio = { version = "1", features = ["full"] }
```

### 2. Provider Configuration

Set your environment variables:

```bash
# AWS
export AWS_ACCESS_KEY_ID=xxx
export AWS_SECRET_ACCESS_KEY=xxx
export AWS_REGION=us-east-1

# ZeroCloud
export ZERO_URL=http://localhost:8080
```

### 3. Basic Code Example

```rust
use cloudkit::prelude::*;

#[tokio::main]
async fn main() -> Result<(), CloudError> {
    let cloud = CloudKit::aws().build().await?;
    
    // Unified API works for any provider
    cloud.storage()
        .put_object("my-bucket", "test.txt", b"Hello!")
        .await?;
        
    Ok(())
}
```

---

## Summary

In just a few steps, you've integrated CloudKit and successfully communicated with a cloud provider. For more complex scenarios, check the detailed provider guides and the full API reference.

**Key Takeaways**:
1. Every service is accessible through the `cloud` instance.
2. Logic is provider-agnostic.
3. Errors are unified into `CloudError`.

---

**Related Documentation**:
- [Developer Guide](../developer-guide.md)
- [Architecture Details](../../3-design/architecture.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0
