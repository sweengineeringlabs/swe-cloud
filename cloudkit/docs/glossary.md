# Glossary

Alphabetized list of terms used in the CloudKit Framework.

---

## A

**API Layer (Layer 3)**
: The layer containing service contracts (traits) that define cloud operations.

**AuthProvider**
: An SPI component responsible for fetching and managing cloud credentials.

---

## C

**CloudConfig**
: Configuration settings for a CloudKit instance.

**CloudError**
: The unified error type used across the entire SDK.

**Common Layer (Layer 1)**
: The foundation layer containing shared types, errors, and utilities.

**Core Layer (Layer 4)**
: The layer responsible for orchestration logic and provider-specific implementations.

---

## F

**Facade Layer (Layer 5)**
: The public-facing layer that provides the simplified entry point for users.

---

## S

**SEA (Stratified Encapsulation Architecture)**
: The 5-layer architectural pattern used to isolate concerns and enable extensibility.

**SPI Layer (Layer 2)**
: The Service Provider Interface layer, providing extension points like retries and logging.

---

## See Also

- [Documentation Hub](overview.md)
- [Architecture Details](3-design/architecture.md)
