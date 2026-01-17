# ZeroCloud Documentation Hub

ZeroCloud is a high-performance, functional private cloud platform offering native services for Compute, Storage, Database, Functions, and more.

## Quick Navigation

-   **[3-Design](3-design/architecture.md)**: Architecture, ADRs, and system design.
    - [Cloud Plane Guide](3-design/cloudplane_guide.md): Control vs. Data Plane distinction.
-   **[4-Development](4-development/developer-guide.md)**: Coding standards, setup, and contribution guides.
-   **[AWS-Zero Parity](2-planning/aws_parity_manifesto.md)**: Functional and Data-Plane comparison.
-   **[Zero SDK Rust](../sdk/zero-sdk-rust/README.md)**: Native Rust client documentation.
-   **[Glossary](glossary.md)**: Project terminology.

## Crates List

### Foundation
-   **zero-control-spi** - Shared traits and types.
-   **zero-data-core** - Driver aggregation and node management.

### Control Plane
-   **zero-control-core** - API orchestration logic (Store, DB, Func, Queue, IAM, LB).
-   **zero-control-facade** - HTTP REST API server.

### Clients & SDKs
-   **zero-sdk-rust** - High-level Rust client library.
-   **zero-cli** - Management CLI utility.

### Integrations
-   **cloudkit-zero** - CloudKit adapter for ZeroCloud.

## Backlog
See [backlog.md](backlog.md) for planned features and priorities.
