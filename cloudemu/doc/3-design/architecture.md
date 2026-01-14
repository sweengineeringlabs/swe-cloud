# CloudEmu Architecture

**Audience**: Developers, Contributors, and System Architects.

## WHAT: Two-Layer Emulation Design

CloudEmu uses a **Control Plane / Data Plane** architecture to separate HTTP orchestration from persistence logic. This design allows for modular service development, efficient testing, and future extensibility (e.g., adding support for Azure or GCP emulation).

**Scope**:
- Request lifecycle from HTTP ingress to storage.
- Separation of concerns between the API boundary and persistence.
- Crate organization (`control-plane` and `data-plane`).

## WHY: Separation of Concerns

### Problems Addressed

1. **Monolithic Storage Logic**
   - Impact: All service storage operations in a single 1700+ line file.
   - Consequence: Difficult to test, maintain, and extend individual services.

2. **HTTP and Storage Coupling**
   - Impact: Service handlers directly calling storage methods.
   - Consequence: Hard to swap storage backends or add caching layers.

### Benefits
- **Testability**: Each plane can be unit-tested independently.
- **Modularity**: Adding a new AWS service only requires a handler and storage method.
- **Performance**: Future optimizations (e.g., caching, connection pooling) can be added to the Data Plane without affecting the Control Plane.

## HOW: Request Lifecycle

```
┌─────────────────────────────────────────────────────────────────┐
│                     Request Lifecycle                            │
│                                                                   │
│   1. HTTP Request (Terraform/AWS SDK/CLI)                        │
│      │                                                            │
│      ▼                                                            │
│   ┌──────────────────────────────────────────────────────────┐   │
│   │  CONTROL PLANE (crates/control-plane)                    │   │
│   │                                                           │   │
│   │  a) Ingress: Axum HTTP Server (0.0.0.0:4566)            │   │
│   │  b) Gateway: Route to service endpoint                   │   │
│   │  c) Dispatcher: Parse AWS headers (x-amz-target)        │   │
│   │  d) Service Handler: Execute service logic              │   │
│   │     - services/s3/                                        │   │
│   │     - services/dynamodb/                                  │   │
│   │     - services/sqs/                                       │   │
│   │     - services/lambda/                                    │   │
│   │     - services/secrets/                                   │   │
│   │     - services/kms/                                       │   │
│   │     - services/events/                                    │   │
│   │     - services/monitoring/                                │   │
│   │     - services/identity/                                  │   │
│   │     - services/workflows/                                 │   │
│   └──────────────────────┬───────────────────────────────────┘   │
│                          │                                        │
│                          ▼                                        │
│   ┌──────────────────────────────────────────────────────────┐   │
│   │  DATA PLANE (crates/data-plane)                          │   │
│   │                                                           │   │
│   │  a) Storage Engine: Unified persistence API              │   │
│   │     - SQLite (metadata, indexes, policies)               │   │
│   │     - Filesystem (S3 object blobs)                       │   │
│   │  b) Configuration: Global emulator settings              │   │
│   └──────────────────────────────────────────────────────────┘   │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow Example (S3 PutObject)

```
1. Client → PUT /my-bucket/hello.txt
2. Gateway → Route to S3 handler
3. Dispatcher → Identify "s3:PutObject"
4. S3 Handler → Validate bucket, parse body
5. Storage Engine → Insert metadata row (SQLite)
6. Storage Engine → Write blob to .cloudemu/objects/{hash}
7. S3 Handler → Generate XML response
8. Client ← 200 OK
```

## Crate Structure

```
cloudemu/
├── crates/
│   ├── control-plane/       # HTTP orchestration
│   │   ├── src/
│   │   │   ├── gateway/     # Axum router
│   │   │   ├── services/    # AWS service handlers
│   │   │   └── lib.rs       # Public API
│   └── data-plane/          # Persistence
│       ├── src/
│       │   ├── storage/     # SQLite + Filesystem
│       │   ├── config.rs    # Configuration
│       │   └── lib.rs       # Public API
```

---

## Summary

The Control Plane / Data Plane architecture provides a clean separation between HTTP orchestration and persistence. This design makes CloudEmu modular, testable, and extensible for future enhancements.

**Key Takeaways**:
1. Control Plane handles all HTTP routing and service logic.
2. Data Plane handles all persistence (SQLite and filesystem).
3. Each AWS service is a self-contained module in `services/`.

---

**Related Documentation**:
- [Backlog](../4-development/backlog.md)
- [Overview](../overview.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0
