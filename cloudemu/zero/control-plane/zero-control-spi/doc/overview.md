# zero-control-spi Overview

## WHAT
The Service Provider Interface (SPI) for ZeroCloud. It defines the core traits (`ComputeDriver`, `StorageDriver`, `NetworkDriver`) and shared request/response types.

## WHY
| Problem | Solution |
|---------|----------|
| Vendor lock-in | Trait-based abstraction allowed multiple backends (Docker, KVM). |
| Tight Coupling | Core logic only depends on traits, not implementations. |
| Inconsistent APIs | Type-safe `ZeroRequest` and `ZeroResponse` unify communication. |

## HOW

```rust
use async_trait::async_trait;
use zero_control_spi::{ComputeDriver, ZeroResult, WorkloadStatus};

struct MyDriver;

#[async_trait]
impl ComputeDriver for MyDriver {
    async fn create_workload(&self, id: &str, image: &str, cpu: f32, mem_mb: i32) -> ZeroResult<WorkloadStatus> {
        // Implementation logic
    }
}
```

## Documentation

| Document | Description |
|----------|-------------|
| [Developer Guide](../../../docs/4-development/developer-guide.md) | Full development guide |

---

**Status**: Stable
