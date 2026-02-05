# CloudKit SPI Crate

## WHAT: Foundational Types & Extensions

`cloudkit_spi` contains the foundation (Layer 1) and extension points (Layer 2) of the SDK. It defines the universal `CloudError` type, `CloudConfig` for configuration, and traits for auth, retries, and metrics that service providers use to hook into the SDK lifecycle.

**Prerequisites**:
- No internal dependencies (Layer 1).

## WHY: Deep Integration & Customization

### Problems Solved
- **Error Fragmentation**: Standardizing disparate cloud error codes into a manageable enum.
- **Provider Extension**: Allowing users to bring their own auth mechanisms or observability stacks without modifying the core SDK.

## HOW: Usage Example

Customizing a retry policy:
```rust
let config = CloudConfig::builder()
    .retry_policy(MyCustomRetry::new())
    .build()?;
```

## Examples and Tests
- **Error Tests**: Verifying the mapping from external provider errors to `CloudError`.
- **Config Tests**: Ensuring that builders accurately construct `CloudConfig`.

---

**Last Updated**: 2026-01-14
