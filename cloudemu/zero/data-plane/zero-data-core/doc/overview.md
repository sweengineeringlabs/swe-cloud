# zero-data-core Overview

## WHAT
The Data Plane Engine for ZeroCloud. It aggregates drivers and provides a high-level `ZeroEngine` for managing nodes and physical resources.

## WHY
| Problem | Solution |
|---------|----------|
| Platform Complexity | `ZeroEngine` abstracts OS differences (Windows/Linux). |
| Resource State | Built-in SQLite persistence for tracking nodes. |
| Configuration | `auto()` mode simplifies environmental driver selection. |

## HOW

```rust
use zero_data_core::ZeroEngine;
use std::sync::Arc;

// Auto-detect best drivers for current OS
let engine = ZeroEngine::auto()?;
let node = engine.register_node("local-host", "127.0.0.1")?;
```

## Documentation

| Document | Description |
|----------|-------------|
| [Architecture Hub](../../../docs/3-design/architecture.md) | System design details |

---

**Status**: Beta
