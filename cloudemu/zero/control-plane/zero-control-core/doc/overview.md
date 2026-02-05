# zero-control-core Overview

## WHAT
The orchestration logic for ZeroCloud. It implements the `ZeroService` trait to handle `ZeroRequest` and map them to `ZeroEngine` operations.

## WHY
| Problem | Solution |
|---------|----------|
| Request Routing | Maps REST-like paths (/v1/workloads) to driver calls. |
| Decoupling | Serves as the "Brain" between the API Facade and the Data Plane. |

## HOW

```rust
use zero_control_core::ZeroProvider;
use std::sync::Arc;

let provider = ZeroProvider::new(Arc::new(engine));
let resp = provider.handle_request(req).await?;
```

---

**Status**: Beta
