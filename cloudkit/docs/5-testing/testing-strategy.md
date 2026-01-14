# CloudKit Testing Strategy

**Audience**: QA Engineers, Core Contributors, and Security Reviewers.

## WHAT: Multi-Cloud Reliability Assurance

This document defines the testing framework used to verify the CloudKit SDK. Due to the inherent complexity of interacting with multiple proprietary cloud APIs, we use a tiered testing strategy that balances speed, cost, and coverage.

**Scope**:
- Unit testing with trait mocks.
- Functional testing of provider implementations.
- Integration testing against live cloud environments.
- Continuous Integration (CI) pipeline setup.

## WHY: Trust in the Abstraction

### Problems Addressed

1. **Provider Regressions**
   - Impact: A change in the AWS SDK breaking the `ObjectStorage` implementation.
   - Consequence: Unreliable storage operations in production.

2. **Orchestration Errors**
   - Impact: Logic errors in the `OperationExecutor` (retries, timeouts).
   - Consequence: Cascading failures or hung requests.

### Benefits
- **Zero-Cost Verification**: Mock-based unit tests run instantly and don't require cloud credentials.
- **Provider Parity**: Ensures that a "NotFound" error from Azure is handled exactly like a "NoSuchKey" error from AWS.

## HOW: Tiered Testing Hierarchy

### 1. Unit Tests (Layered)
Every SEA layer is tested independently:
- **SPI/API**: Validation of builders and models.
- **Core**: Using `mockall` to mock Layer 3 traits and verify orchestration logic.

```bash
cargo test
```

### 2. Integration Tests (Live Providers)
Functional tests that run against real sandbox accounts. These are feature-gated to prevent accidental execution.

```bash
cargo test --features integration,aws
```

### 3. Coverage Analysis
We target 85%+ code coverage across all provider implementations.

```bash
cargo llvm-cov --all-features
```

---

## Summary

Testing is the bedrock of CloudKit. By combining strict architectural layering with a tiered test suite, we ensure that our multi-cloud abstractions are as reliable as the underlying native SDKs.

**Key Takeaways**:
1. Never commit code without unit tests for orchestration logic.
2. Use `integration` feature for live tests.
3. Map all new provider errors to the centralized `CloudError` in tests.

---

**Related Documentation**:
- [Architecture Details](../3-design/architecture.md)
- [Developer Guide](../4-development/developer-guide.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0
