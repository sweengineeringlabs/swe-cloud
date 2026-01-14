# Cloud Providers Overview

**Audience**: Architects and Developers planning multi-cloud deployments.

## WHAT: Supported Cloud Ecosystems

CloudKit provides a unified abstraction layer for AWS, Azure, and GCP. This guide compares how each provider maps to CloudKit traits and which services are currently supported at the Core layer.

**Scope**:
- Provider service mapping matrix (e.g., S3 vs. Blob vs. GCS).
- Feature flag management for minimizing binary size.
- Decision matrix for choosing a cloud provider.

## WHY: Multi-Cloud Portability

### Problems Addressed

1. **Service Inconsistency**
   - Impact: Different clouds have unique concepts for the same service (e.g., SQS vs. Pub/Sub).
   - Consequence: "Write once, deploy anywhere" is impossible without a standardized mapping.

2. **Crate Bloat**
   - Impact: Including all cloud provider SDKs significantly increases binary size.
   - Consequence: Slow deployment times and higher memory footprint.

### Benefits
- **Trait-Based Abstraction**: The same business logic can run on any supported provider.
- **Granular Feature Flags**: Only compile the providers and services you actually use.

## HOW: Provider Mapping & Configuration

### 1. Service Matrix (API Layer Mappings)

| Service Type | AWS | Azure | GCP |
| :--- | :--- | :--- | :--- |
| **Object Storage** | S3 | Blob Storage | Cloud Storage |
| **Key-Value Store** | DynamoDB | Cosmos DB | Firestore |
| **Message Queue** | SQS | Service Bus | Cloud Tasks |
| **Pub/Sub** | SNS | Event Grid | Pub/Sub |

### 2. Feature Flag Management

Each provider crate is divided into granular features to keep the build lean:

```toml
[dependencies]
cloudkit-aws = { version = "0.1", features = ["s3", "sqs"] } # Only includes S3 and SQS
```

### 3. Cross-Provider Pattern

```rust
use cloudkit::prelude::*;

async fn upload<S: ObjectStorage>(storage: &S) -> CloudResult<()> {
    storage.put_object("bucket", "config.json", b"{...}").await
}
```

---

## Summary

By providing a consistent trait-based mapping across AWS, Azure, and GCP, CloudKit eliminates the complexity of multi-cloud engineering. Developers can focus on building features rather than learning provider-specific SDK idiosyncrasies.

**Key Takeaways**:
1. Check the [Service Matrix](#1-service-matrix-api-layer-mappings) to verify provider support.
2. Use feature flags to minimize your `cloudkit-core` dependency surface.
3. Test your business logic against the `ObjectStorage` trait, not a concrete implementation.

---

**Related Documentation**:
- [AWS Provider Guide](./provider-aws.md)
- [Architecture Hub](../../3-design/architecture.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0
