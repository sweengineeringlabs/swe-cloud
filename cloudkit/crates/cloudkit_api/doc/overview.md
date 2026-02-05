# CloudKit API Crate

## WHAT: Service Contracts (Traits)

`cloudkit_api` is the contract layer (Layer 3) of the SDK. It defines provider-agnostic traits for all supported cloud services, such as `ObjectStorage`, `KeyValueStore`, and `MessageQueue`. It also defines the request and response models used by these services.

**Prerequisites**:
- Depends on `cloudkit_spi` for foundational types.

## WHY: Standardizing Cloud Interactions

### Problems Solved
- **API Leakage**: Preventing provider-specific types (e.g., `aws_sdk_s3::types::Object`) from leaking into application logic.
- **Service Consistency**: Ensuring that a "PutObject" operation has the same signature regardless of the backend cloud.

## HOW: Usage Example

Defining a service implementation:
```rust
#[async_trait]
impl ObjectStorage for MyProviderStorage {
    async fn put_object(&self, bucket: &str, key: &str, data: &[u8]) -> CloudResult<()> {
        // Implementation
    }
}
```

## Examples and Tests
- **Contract Tests**: This crate contains the test traits that provider implementations must pass to ensure compatibility.

---

**Last Updated**: 2026-01-14
