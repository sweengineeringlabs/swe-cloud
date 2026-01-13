# CloudEmu Architecture

The CloudEmu architecture is designed to be clean, decoupled, and highly maintainable. It follows a strict hierarchical lifecycle for processing requests, clearly separating the API boundary (Control Plane) from the persistence logic (Data Plane).

## Request Lifecycle

Requests flow through the following layers:

1.  **Ingress Controller**: Manages the physical TCP/HTTP server, listener binding, and initial state initialization (`ingress.rs`).
2.  **Gateway**: The central routing layer (Axum Router) that directs traffic to appropriate endpoints based on high-level paths or protocols (`gateway.rs`).
3.  **Dispatcher**: Inspects AWS-specific headers (e.g., `x-amz-target`) or request types to route the generic request to the correct internal Service Handler (`dispatcher.rs`).
4.  **Service Handler**: Implements the AWS service-specific logic (S3, DynamoDB, SQS, etc.), validating requests and preparing responses (`services/`).
5.  **Storage Engine**: The persistence layer that handles data durability, metadata management (SQLite), and blob storage (Filesystem) (`data-plane` crate).

## Crate Split

-   **Control Plane**: The high-level API orchestrator containing the Ingress, Gateway, Dispatcher, and Service Handlers.
-   **Data Plane**: The low-level infrastructure layer responsible for persistence, configuration, and internal data structures.
