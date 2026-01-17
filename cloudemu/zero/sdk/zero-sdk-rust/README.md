# Zero SDK Rust Overview

> **Scope**: High-level overview only. Implementation details belong in [Developer Guide](../../docs/4-development/developer-guide.md).

## Audience

Rust developers who want to interact with the ZeroCloud platform programmatically using a type-safe, asynchronous client library.

## WHAT

The Zero SDK for Rust provides a comprehensive set of client implementations for all ZeroCloud services, including Store, DB, Func, Queue, IAM, and LB.

Key capabilities:
- **ZeroStore API** - High-performance object storage operations.
- **ZeroDB API** - Document and key-value database interactions.
- **ZeroFunc API** - Invocation and management of serverless functions.
- **ZeroQueue API** - Reliable asynchronous messaging.
- **ZeroID API** - Identity management for users and groups.
- **ZeroLB API** - Load balancer and reverse proxy configuration.

## WHY

| Problem | Solution |
|---------|----------|
| Manual HTTP calls are error-prone | Type-safe wrappers for all API endpoints |
| Complex service orchestration | Unified `ZeroClient` to access all services |
| Async boilerplate | Built-in `tokio` support for high-concurrency apps |

## HOW

```rust
use zero_sdk::ZeroClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = ZeroClient::new("http://localhost:8080");
    
    // Create a bucket in ZeroStore
    client.store().create_bucket("my-bucket").await?;
    
    // Send a message to ZeroQueue
    client.queue().send_message("my-queue", "Hello Zero!").await?;
    
    Ok(())
}
```

## Documentation

| Document | Description |
|----------|-------------|
| [Developer Guide](../../docs/4-development/developer-guide.md) | Build, test, API reference |
| [Architecture](../../docs/3-design/architecture.md) | System design and SEA layers |
| [Backlog](../../docs/backlog.md) | Planned features |

---

**Status**: Stable (v0.1.0)
