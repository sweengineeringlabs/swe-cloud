# Glossary

Alphabetized list of terms used in CloudKit.

---

**API Layer** - The public-facing API layer that provides typed, ergonomic interfaces for cloud operations. Sits between Core and user code.

**ARN (Amazon Resource Name)** - Unique identifier for AWS resources following the format `arn:partition:service:region:account-id:resource`.

**Cloud Provider** - A platform offering cloud computing services (AWS, Azure, GCP, Oracle).

**CloudContext** - Core orchestration struct that manages cloud operations, retries, and metrics across providers.

**Core Layer** - Implementation layer containing the business logic and operational execution. Orchestrates between API and SPI layers.

**Executor** - Component responsible for executing cloud operations with retry logic and metrics collection.

**Facade** - Simplified, high-level interface for common multi-cloud patterns (not provider-specific).

**Operation** - A single cloud action (e.g., upload file, create bucket) that can be retried.

**Provider** - Cloud-specific implementation (e.g., AWS, Azure, GCP) that implements the SPI contract.

**Resource** - Cloud entity (bucket, database, queue) managed by a cloud provider.

**Retry Policy** - Strategy defining how and when to retry failed operations.

**SEA (Stratified Encapsulation Architecture)** - Layered architectural pattern with SPI, Core, API, and Facade layers.

**SPI (Service Provider Interface)** - Contract layer defining interfaces that cloud providers must implement.

**Trait** - Rust interface definition that specifies required methods for a type.

**Workspace** - Cargo concept grouping multiple related Rust crates under a single repository.
