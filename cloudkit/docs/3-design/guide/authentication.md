# CloudKit Authentication Design

**Audience**: Security Engineers and Provider Implementers.

## WHAT: The AuthProvider SPI

CloudKit standardizes authentication across all cloud providers through the `AuthProvider` SPI (Layer 2). This allows for a unified way to fetch, cache, and rotate cloud credentials regardless of whether they come from environment variables, local profiles, or instance identity services.

**Scope**:
- The `AuthProvider` trait definition.
- Support for static keys, IAM roles, and Managed Identities.
- Credential caching and rotation logic.
- Custom authentication integration.

## WHY: Secure Multi-Cloud Access

### Problems Addressed

1. **Credential Proliferation**
   - Impact: Managing different auth patterns for AWS, Azure, and GCP.
   - Consequence: Security misconfigurations and difficult key rotation.

2. **Hard-coded Secrets**
   - Impact: Developers hard-coding access keys in source code.
   - Consequence: Credential leaks and non-compliance.

### Benefits
- **Identity Abstraction**: The core SDK doesn't care WHERE the credentials come from, only that it has a valid provider.
- **Enhanced Security**: Prefers secure, short-lived credentials (IAM roles) over long-lived keys.

## HOW: Auth Implementation

### 1. The AuthProvider Trait

```rust
#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn get_credentials(&self) -> CloudResult<Credentials>;
}
```

### 2. Built-in Providers

CloudKit includes several standard implementations:
- **`EnvAuthProvider`**: Reads from standard cloud environment variables.
- **`ProfileAuthProvider`**: Reads from localized cloud CLI profiles (e.g., `~/.aws/credentials`).
- **`InstanceAuthProvider`**: Fetches from metadata services (EC2/AKS/GKE).

### 3. Custom Integration

Users can implement their own `AuthProvider` to integrate with Enterprise Secret Managers or Vault:

```rust
struct VaultAuthProvider { ... }

#[async_trait]
impl AuthProvider for VaultAuthProvider {
    async fn get_credentials(&self) -> CloudResult<Credentials> {
        // Fetch from Vault
    }
}
```

---

## Summary

Authentication in CloudKit is designed to be both secure and flexible. By using the `AuthProvider` SPI, the SDK ensures that identity management is handled consistently across all cloud boundaries, supporting modern security practices like short-lived credentials and role-based access.

**Key Takeaways**:
1. Use `AuthProvider` trait for all credential logic.
2. Favor environmental and instance-based auth over static keys.
3. Credentials should be refreshed before expiry to ensure zero-downtime.

---

**Related Documentation**:
- [Architecture Hub](../architecture.md)
- [Developer Guide](../../4-development/developer-guide.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0
