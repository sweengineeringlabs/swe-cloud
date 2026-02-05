# CloudKit Installation & Deployment

**Audience**: DevOps Engineers and SREs.

## WHAT: Integrating CloudKit into Production

CloudKit is delivered as a series of Rust crates. This guide covers how to include the library in your microservices and configure them for production use.

**Scope**:
- Cargo dependency management.
- Configuration via environment or files.
- Production-ready retries and timeouts.
- Monitoring and tracing setup.

## WHY: Production Reliability

### Problems Addressed

1. **Dependency Overload**
   - Impact: Including all cloud provider SDKs in every microservice.
   - Consequence: Large binary sizes and long build times.

2. **Opaque Failures**
   - Impact: Infrastructure operations failing without clear logging or metrics.
   - Consequence: High MTTR (Mean Time to Recovery).

### Benefits
- **Optimized Builds**: Only compile the cloud features you need.
- **Unified Observability**: Consistent metrics and logs across all cloud resources.

## HOW: Installation Steps

### 1. Cargo Configuration

Choose your providers and services using feature flags:

```toml
[dependencies]
cloudkit = "0.1"
cloudkit-aws = { version = "0.1", features = ["s3", "lamda"] }
```

### 2. Context Initialization

In your production `main.rs`, initialize the `CloudKit` context with specific production settings:

```rust
let config = CloudConfig::builder()
    .retry_policy(RetryPolicy::exponential(5)) // 5 retries for production
    .timeout(Duration::from_secs(60))
    .build()?;

let cloud = CloudKit::aws().with_config(config).build().await?;
```

### 3. Observability Setup

Enable the `tracing` feature to see detailed operation logs:

```rust
tracing_subscriber::fmt::init();
```

---

## Summary

Deploying CloudKit in production is straightforward. By utilizing feature flags for lean binaries and configuring standardized retry and observability policies, you ensure your infrastructure code is robust and maintainable.

**Key Takeaways**:
1. Use feature flags to keep binaries small.
2. Initialize with high-reliability retry policies for production.
3. Integrate with standard `tracing` and `metrics` collectors.

---

**Related Documentation**:
- [Developer Guide](../4-development/developer-guide.md)
- [Testing Strategy](../5-testing/testing-strategy.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0
