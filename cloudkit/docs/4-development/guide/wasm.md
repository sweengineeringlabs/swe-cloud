# WebAssembly (WASM) Guide

**Audience**: Developers building browser-based or edge-computing applications with Rust.

## WHAT: CloudKit in Web Environments

This guide explores the current and future state of WebAssembly (WASM) support in CloudKit. It details the architecture of WASM, target environments, and the challenges of porting a native cloud SDK to a sandboxed environment.

**Scope**:
- WASM architecture and Rust targets.
- Ecosystem components (`wasm-bindgen`, `wasm-pack`).
- Differences between Native and WASM execution.
- Optimization and performance tips.

## WHY: Cloud Access at the Edge

### Problems Addressed

1. **Backend Dependency**
   - Impact: Browser apps must call a dedicated backend to interact with cloud storage or queues.
   - Consequence: Increased latency and infrastructure cost for simple operations.

2. **Environment Incompatibility**
   - Impact: standard library `std` features (threads, file system) are not available in WASM.
   - Consequence: Existing Rust SDKs often fail to compile for WASM targets.

### Benefits
- **Edge Performance**: Run CloudKit logic directly in Cloudflare Workers or Fastly Compute.
- **Code Sharing**: Share validation and orchestration logic between the server and the frontend.

## HOW: Integrating with WASM

### Current Limitations

CloudKit is primarily designed for native server-side use. To use it in WASM, several components must be feature-gated:

| Feature | Native Implementation | WASM Implementation |
|---------|-----------------------|---------------------|
| **Async** | `tokio` | `wasm-bindgen-futures` |
| **HTTP** | `reqwest` | `fetch` API |
| **FS** | `std::fs` | Not available |

### Build Instructions

To build a project for the web:

```bash
# Add the target
rustup target add wasm32-unknown-unknown

# Build using wasm-pack
wasm-pack build --target web
```

### Optimization Profile

Add these to your `Cargo.toml` for the smallest binary size:

```toml
[profile.release]
lto = true
opt-level = "s"
codegen-units = 1
panic = "abort"
```

---

## Summary

WASM is a critical frontier for CloudKit. While natively optimized for the server, moving towards a feature-gated approach will allow developers to bring the same unified API to edge and browser environments.

**Key Takeaways**:
1. Most `std` I/O is unavailable in WASM.
2. Binary size is a primary constraint; optimize early.
3. Use `wasm-pack` for the best development experience.

---

**Related Documentation**:
- [Architecture Details](../../3-design/architecture.md)
- [Developer Guide](../developer-guide.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0
