# CloudKit Core Crate

## WHAT: The SDK Engine

`cloudkit_core` is the engine layer (Layer 4) of the SDK. It implements the service contracts defined in `cloudkit_api` for AWS, Azure, and GCP. It also contains the `CloudContext` which manages client lifecycle, execution, and provider-specific configurations.

**Prerequisites**:
- Must be used within the context of a `CloudKit` facade.
- Cloud provider SDKs (handled via Cargo features).

## WHY: Centralized Orchestration

### Problems Solved
- **Provider Divergence**: Consolidating different cloud SDK behaviors into a single execution pattern.
- **Shared Logic**: Centralizing retry, timeout, and observability logic that should apply to all providers.

## HOW: Usage Example

Internal use only by the Facade:
```rust
// Core logic for operation execution
let result = executor.execute(|| {
    provider_client.some_operation(request)
}).await?;
```

## Examples and Tests
- **Provider Tests**: Each provider subdirectory (`aws/`, `gcp/`, `azure/`) contains unit tests for its specific implementation.
- **Mock Tests**: Extensive use of `mockall` to verify core orchestration logic without network calls.

---

**Last Updated**: 2026-01-14
