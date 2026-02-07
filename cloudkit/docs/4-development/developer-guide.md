# CloudKit Developer Guide

**Audience**: Internal developers and open-source contributors.

## WHAT: Development Standards & Workflow

This guide details the process for adding new features, services, or providers to the CloudKit SDK. It ensures that all code follows the SEA architectural principles and remains high-quality and high-performance.

**Scope**:
- Crate structure and development dependencies.
- Step-by-step guide for adding new services.
- Coding and documentation standards.
- Testing and benchmarking requirements.

## WHY: Consistency & Quality

### Problems Addressed

1. **Architectural Drift**
   - Impact: New services ignoring the SEA layering.
   - Consequence: Spaghetti code and breaking cross-provider abstractions.

2. **Performance Regressions**
   - Impact: Non-optimized async code or excessive cloning.
   - Consequence: Lower throughput and higher memory usage.

### Benefits
- **Scalable Architecture**: Easy for new contributors to find where code belongs.
- **Predictable APIs**: Users get a consistent experience across all CloudKit services.

## HOW: Development Workflow

### 1. Adding a New Cloud Service (e.g., "Secrets Manager")

**Step 1: Contract Definition (`cloudkit_api`)**
Define a new trait in `cloudkit_api/src/secrets.rs`. Use async-trait where necessary.

**Step 2: Core Implementation (`cloudkit_core`)**
Implement the new trait for each cloud provider.
- `cloudkit_core/aws/src/secrets.rs`
- `cloudkit_core/gcp/src/secrets.rs`

**Step 3: Facade Integration (`cloudkit_facade`)**
Add a `secrets()` method to the `CloudKit` facade and re-export the new types.

**Step 4: Update Documentation**
- Add an `overview.md` to the relevant sub-crates.
- Update the main Documentation Hub.

**Step 5: Testing**
- Add unit tests for each implementation.
- Add an integration example in the `examples/` directory.

### 2. Standards

- **Rust Version**: Minimum supported version is 1.85.
- **Async**: Use `tokio` for all async operations.
- **Error Handling**: Map all provider errors to `CloudError`.
- **Docs**: Every public items must be documented with `///` and include an example `# Examples`.

---

## Summary

Following this guide ensures that CloudKit continues to provide a world-class developer experience. By respecting the SEA layers and maintaining high testing standards, we guarantee the reliability of our multi-cloud abstractions.

**Key Takeaways**:
1. Layers are strict; avoid upward dependencies.
2. Every new service needs a trait in `cloudkit_api`.
3. Performance matters; avoid blocking operations in async code.

---

**Related Documentation**:
- [Architecture Hub](../3-design/architecture.md)
- [Contributing Guidelines](../../CONTRIBUTING.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0
