# ZeroCloud Architecture

**Audience**: Architects, System Administrators, Security Teams

## WHAT
ZeroCloud is a stratified private cloud orchestrator that abstracts local hardware resources (Compute, Storage, Networking) into a unified API.

## WHY
-   **Isolation**: Provides hardware-level isolation for lab workloads.
-   **Performance**: Minimizes virtualization overhead through native driver integration.
-   **Interoperability**: Shares the same Service Provider Interface (SPI) as public cloud emulators.

## HOW

ZeroCloud follows the **Stratified Encapsulation Architecture (SEA)**:

1.  **SPI Layer (`zero-control-spi`)**: Defines the contracts for all drivers.
2.  **Orchestration Layer (`zero-control-core`)**: Manages state, nodes, and high-level logic.
    *   **ZeroStore**: S3-compatible object storage.
    *   **ZeroDB**: DynamoDB-compatible NoSQL database.
    *   **ZeroFunc**: Serverless function execution.
    *   **ZeroQueue**: Message queuing with visibility.
    *   **ZeroID**: Identity & Access Management.
    *   **ZeroLB**: Reverse proxy load balancing.
3.  **Data Driver Layer (`zero-data-core`)**: Communicates with the OS (Hyper-V, KVM, Docker, SQLite, Network Bridge).

### Key Decisions
See the [ADR Index](adr/README.md) for detailed technical decisions.

### Performance Strategy
ZeroCloud prioritizes native performance by utilizing Type-1 hypervisors (Hyper-V) and direct filesystem access for storage.
