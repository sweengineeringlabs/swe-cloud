# WebAssembly (WASM) Guide

## What is WebAssembly?

**WebAssembly (WASM)** is a binary instruction format designed as a portable compilation target. It enables code written in languages like Rust, C++, and Go to run in web browsers and other environments at near-native speed.

## Key Characteristics

| Characteristic | Description |
|----------------|-------------|
| **Binary Format** | Compact bytecode, fast to parse and execute |
| **Portable** | Runs on any platform with a WASM runtime |
| **Safe** | Sandboxed execution, memory-safe by design |
| **Fast** | Near-native performance |
| **Language Agnostic** | Compile from many source languages |

## WASM Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Source Languages                             │
│    Rust    │    C/C++    │    Go    │    AssemblyScript         │
├─────────────────────────────────────────────────────────────────┤
│                        Compiler                                  │
│    rustc   │    clang    │   tinygo │    asc                    │
├─────────────────────────────────────────────────────────────────┤
│                     WebAssembly                                  │
│                    (.wasm binary)                                │
├─────────────────────────────────────────────────────────────────┤
│                     WASM Runtimes                                │
│  Browser  │  Node.js  │  Wasmtime  │  Wasmer  │  Cloudflare    │
└─────────────────────────────────────────────────────────────────┘
```

## WASM Targets in Rust

| Target | Description | Use Case |
|--------|-------------|----------|
| `wasm32-unknown-unknown` | Pure WASM, no OS | Browser, edge computing |
| `wasm32-wasi` | WASM + System Interface | CLI tools, server-side |
| `wasm32-unknown-emscripten` | WASM + Emscripten | C/C++ interop |

### Installing a WASM Target

```bash
# Pure WebAssembly (most common)
rustup target add wasm32-unknown-unknown

# WASM with System Interface
rustup target add wasm32-wasi
```

## WASM Ecosystem for Rust

### Core Tools

| Tool | Purpose |
|------|---------|
| **wasm-pack** | Build, test, and publish WASM packages |
| **wasm-bindgen** | Rust/JavaScript interop |
| **wasm-opt** | Optimize WASM binary size |
| **trunk** | WASM web application bundler |

### Frameworks

| Framework | Description |
|-----------|-------------|
| **Yew** | React-like component framework |
| **Leptos** | Signals-based reactive framework |
| **Dioxus** | Cross-platform UI framework |
| **Sycamore** | Reactive library with fine-grained updates |

## Basic WASM Example

### Rust Code (lib.rs)

```rust
use wasm_bindgen::prelude::*;

// Export function to JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Export struct
#[wasm_bindgen]
pub struct Calculator {
    value: f64,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    pub fn add(&mut self, x: f64) {
        self.value += x;
    }

    pub fn result(&self) -> f64 {
        self.value
    }
}
```

### Cargo.toml

```toml
[package]
name = "my-wasm-lib"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

### Build

```bash
# Install wasm-pack
cargo install wasm-pack

# Build for web
wasm-pack build --target web

# Build for Node.js
wasm-pack build --target nodejs

# Build for bundlers (webpack, etc.)
wasm-pack build --target bundler
```

### JavaScript Usage

```javascript
import init, { greet, Calculator } from './pkg/my_wasm_lib.js';

async function main() {
    await init();  // Initialize WASM module
    
    console.log(greet("World"));  // "Hello, World!"
    
    const calc = new Calculator();
    calc.add(5);
    calc.add(3);
    console.log(calc.result());  // 8
}

main();
```

## WASM for Edge Computing

### Cloudflare Workers

```rust
use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let url = req.url()?;
    
    match url.path() {
        "/" => Response::ok("Hello from Cloudflare Workers!"),
        "/api/data" => {
            // Use CloudKit for storage
            Response::ok("Data endpoint")
        }
        _ => Response::error("Not Found", 404),
    }
}
```

### Fastly Compute

```rust
use fastly::{Request, Response};

#[fastly::main]
fn main(req: Request) -> Result<Response, fastly::Error> {
    match req.get_path() {
        "/" => Ok(Response::from_body("Hello from Fastly!")),
        _ => Ok(Response::from_status(404)),
    }
}
```

## CloudKit + WASM (Future)

Currently, CloudKit is designed for **native server-side** use. WASM support would require modifications:

### Challenges

| Challenge | Native | WASM |
|-----------|--------|------|
| **Async Runtime** | Tokio | wasm-bindgen-futures |
| **HTTP Client** | reqwest | fetch API via gloo-net |
| **File System** | std::fs | Not available |
| **Environment Variables** | std::env | Not available |
| **Threads** | std::thread | Web Workers (limited) |

### Potential Architecture

```rust
// Feature-gated WASM support
#[cfg(not(target_arch = "wasm32"))]
mod native {
    use tokio;
    use reqwest;
    // Native implementations
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen_futures;
    use gloo_net;
    // WASM implementations
}
```

### Use Cases for CloudKit + WASM

1. **Edge Computing** - Run CloudKit at edge locations (Cloudflare, Fastly)
2. **Browser Apps** - Direct cloud access from web applications
3. **Offline-First** - Cache cloud data locally in browser
4. **Hybrid Apps** - Share code between server and client

## WASM Limitations

### Not Available in WASM

- Direct file system access
- Raw TCP/UDP sockets
- Threads (in browsers, though Web Workers exist)
- Environment variables
- Native system calls

### Available in WASM

- HTTP requests (via fetch)
- WebSocket connections
- IndexedDB (browser storage)
- Web Crypto API
- Canvas/WebGL

## WASM Performance Tips

1. **Minimize Size** - Use `wasm-opt` and LTO
2. **Avoid Panics** - Use `panic = "abort"` in release
3. **Lazy Loading** - Load WASM modules on demand
4. **Use TypedArrays** - Efficient data transfer
5. **Batch Calls** - Reduce JS/WASM boundary crossings

### Size Optimization (Cargo.toml)

```toml
[profile.release]
lto = true
opt-level = "s"  # or "z" for smallest size
codegen-units = 1
panic = "abort"

[profile.release.package."*"]
opt-level = "s"
```

## Resources

- [WebAssembly.org](https://webassembly.org/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [wasm-pack Documentation](https://rustwasm.github.io/docs/wasm-pack/)
- [Yew Framework](https://yew.rs/)
- [Leptos Framework](https://leptos.dev/)

## See Also

- [CloudKit Overview](cloudkit-overview.md)
- [Architecture](architecture.md)
- [Configuration](configuration.md)
