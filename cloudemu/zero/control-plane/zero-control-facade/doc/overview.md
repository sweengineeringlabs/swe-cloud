# zero-control-facade Overview

## WHAT
The HTTP API server for ZeroCloud. Built with Axum, it exposes the `ZeroProvider` as a RESTful web service.

## WHY
| Problem | Solution |
|---------|----------|
| Remote Access | Allows the Dashboard and CLI to manage the cloud over HTTP. |
| Security | Integrates CORS and tracing for production-ready monitoring. |

## HOW

```bash
# Start the server on port 8080
cargo run -p zero-control-facade -- --port 8080
```

---

**Status**: Alpha
