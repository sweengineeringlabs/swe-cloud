# CloudKit Documentation Hub

Welcome to the CloudKit documentation. CloudKit is a unified multi-cloud Rust SDK designed with the **Stratified Encapsulation Architecture (SEA)** to provide a unified entry point for multiple cloud providers.

## WHAT: The Unified Rust Cloud SDK

CloudKit provides a single, type-safe API for interacting with AWS, Azure, and GCP. Instead of learning and maintaining separate SDKs, developers can use CloudKit's unified traits and switch providers with a single configuration change.

**Scope**:
- Unified traits for Storage, Queues, Databases, and Messaging.
- SEA architecture for clean separation of concerns.
- Integrated retry, error handling, and observability.
- WASM-compatible foundation for edge computing.

## WHY: Solving Multi-Cloud Complexity

### Problems Addressed

1. **Vendor Lock-in**
   - Impact: Application code tied to provider-specific SDK idiosyncrasies.
   - Consequence: Rewriting logic when migrating clouds or supporting multiple clouds.

2. **Developer Cognitive Load**
   - Impact: Learning three different APIs for the same concept (e.g., S3 vs. Blob vs. GCS).
   - Consequence: High friction when switching between projects or providers.

3. **Inconsistent Operations**
   - Impact: Different retry and error patterns across cloud SDKs.
   - Consequence: Unpredictable production behavior and difficult debugging.

### Benefits
- **Write Once, Deploy Anywhere**: Logic is provider-agnostic.
- **Strict Reliability**: Standardized retry and error policies applied at the Core layer.
- **High Ergonomics**: public Facade designed for maximum developer productivity.

## Quick Navigation

| Target Audience | Recommended Starting Point |
| :--- | :--- |
| **New Users** | [Getting Started](./4-development/guide/getting-started.md) |
| **Developers** | [Developer Guide](./4-development/developer-guide.md) |
| **Architects** | [Architecture Specification](./3-design/architecture.md) |
| **Operations** | [Deployment Guide](./6-deployment/installation.md) |

## Core Documentation

- **[Glossary](./glossary.md)**: Definitions of SEA layers and terminology.
- **[Architecture Hub](./3-design/architecture.md)**: Details on the 5-layer design and dependency flow.
- **[WASM Support](./4-development/guide/wasm.md)**: Using CloudKit in WebAssembly environments.
- **[Provider Overview](./4-development/guide/providers-overview.md)**: Comparison matrix and feature mapping.

## Crate Overview (SEA Layers)

CloudKit is composed of several crates that mirror the SEA layers:

- **[cloudkit_facade](../crates/cloudkit_facade/doc/overview.md)**: Public API surface and entry points.
- **[cloudkit_core](../crates/cloudkit_core/doc/overview.md)**: Orchestration logic and provider implementations.
- **[cloudkit_api](../crates/cloudkit_api/doc/overview.md)**: Service contracts and traits.
- **[cloudkit_spi](../crates/cloudkit_spi/doc/overview.md)**: Foundation and extension points.

---

**Last Updated**: 2026-01-14  
**Version**: 0.1.0
