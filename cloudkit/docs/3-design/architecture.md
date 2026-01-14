# CloudKit SEA Architecture

**Audience**: Architects, System Designers, and Core Contributors.

## WHAT: Stratified Encapsulation Architecture (SEA)

CloudKit is built using **SEA**, a layered architecture pattern that promotes separation of concerns, high testability, and unlimited extensibility. It divides the SDK into five distinct layers, each with a strict responsibility.

**Scope**:
- Layer definitions (Common, SPI, API, Core, Facade).
- Dependency management rules.
- Provider abstraction strategy.
- Extension mechanisms.

## WHY: Reliability in Multi-Cloud

### Problems Addressed

1. **API Fragmentation**
   - Impact: Different cloud SDKs have vastly different patterns (e.g., AWS SDK for Rust vs GCP SDK).
   - Consequence: High learning curve and boilerplate-heavy application code.

2. **Tight Coupling**
   - Impact: Application logic depends directly on provider-specific types.
   - Consequence: Impossible to switch providers or mock services for testing.

3. **Inconsistent Observability**
   - Impact: Retries, logging, and metrics are handled differently for each service.
   - Consequence: Difficult to debug production issues across cloud boundaries.

### Benefits
- **Unified Interface**: Write logic once and use it across AWS, Azure, and GCP.
- **Provider Agnostic**: Swap underlying providers at runtime or compile-time without changing business logic.
- **Enhanced Reliability**: Universal retry and error handling logic applied at the Core layer.

## HOW: The 5-Layer Implementation

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│              FACADE (Public entry point - Layer 5)               │
├─────────────────────────────────────────────────────────────────┤
│              CORE (Orchestration & Providers - Layer 4)          │
├─────────────────────────────────────────────────────────────────┤
│              API (Service Traits - Layer 3)                      │
├─────────────────────────────────────────────────────────────────┤
│              SPI (Extension Points - Layer 2)                    │
├─────────────────────────────────────────────────────────────────┤
│              COMMON (Foundational Types - Layer 1)               │
└─────────────────────────────────────────────────────────────────┘
```

### Layer Details

#### Layer 1: Common
Shared constants, `CloudError`, and `CloudConfig`. Foundation for all other layers.

#### Layer 2: SPI
Traits for `AuthProvider`, `RetryPolicy`, and `MetricsCollector`. Allows users to override internal SDK behaviors.

#### Layer 3: API
Service contracts (traits) such as `ObjectStorage`, `MessageQueue`, and `PubSub`. These define the "what" without specifying the "how".

#### Layer 4: Core
The engine of the SDK. It houses the `CloudContext` and the actual implementations for AWS, Azure, and GCP that fulfill Layer 3 traits.

#### Layer 5: Facade
The ergonomic entry point. Re-exports essential types and provides the `CloudKit` builder.

## Summary

The SEA architecture ensures that CloudKit remains maintainable as it scales to more services and providers. By isolating provider-specific logic in the Core layer and exposing clean traits in the API layer, it achieves true multi-cloud portability.

**Key Takeaways**:
1. Dependencies only flow downwards (Layer 5 -> Layer 1).
2. All cloud operations are defined by traits in Layer 3.
3. Extension and customization are handled through SPI (Layer 2).

---

**Related Documentation**:
- [Developer Guide](../4-development/developer-guide.md)
- [Glossary](../glossary.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0
