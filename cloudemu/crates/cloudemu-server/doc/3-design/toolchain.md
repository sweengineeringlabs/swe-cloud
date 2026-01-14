# Toolchain

## Overview

CloudEmu Server requires Rust toolchain and optional cloud CLI tools for development and testing.

## Tools

### Rust Compiler

| | |
|---|---|
| **What** | Systems programming language compiler with memory safety guarantees |
| **Version** | 1.70+ (recommended: 1.75+) |
| **Install** | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |

**Why we use it**: CloudEmu is built in Rust for performance, safety, and async capabilities.

**How we use it**:
```bash
# Build release binary
cargo build --release -p cloudemu-server

# Run server
cargo run -p cloudemu-server

# Run tests
cargo test -p cloudemu-server
```

### Cargo

| | |
|---|---|
| **What** | Rust's package manager and build system |
| **Version** | 1.70+ (bundled with Rust) |
| **Install** | Included with Rust installation |

**Why we use it**: Manages dependencies, builds multi-crate workspace, runs tests.

**How we use it**:
```bash
# Workspace operations
cargo build --workspace
cargo test --workspace
cargo clippy --workspace
```

### Tokio Runtime

| | |
|---|---|
| **What** | Async runtime for Rust |
| **Version** | 1.35+ (specified in Cargo.toml) |
| **Install** | Automatic via Cargo |

**Why we use it**: Enables high-performance concurrent server operations.

**How we use it**:
```rust
#[tokio::main]
async fn main() {
    // Spawn concurrent providers
    tokio::spawn(start_aws_provider());
    tokio::spawn(start_azure_provider());
}
```

### Axum Web Framework

| | |
|---|---|
| **What** | Web framework built on Tokio and Hyper |
| **Version** | 0.7+ |
| **Install** | Automatic via Cargo |

**Why we use it**: Provides HTTP server infrastructure with excellent performance and type safety.

**How we use it**:
```rust
async fn handle_request(
    State(provider): State<Arc<impl CloudProviderTrait>>,
    req: Request<Body>,
) -> Response
```

### Clap

| | |
|---|---|
| **What** | Command-line argument parser |
| **Version** | 4.5+ |
| **Install** | Automatic via Cargo |

**Why we use it**: Provides CLI configuration for ports and provider flags.

**How we use it**:
```rust
#[derive(Parser)]
struct AppConfig {
    #[arg(long, env = "CLOUDEMU_AWS_PORT", default_value = "4566")]
    aws_port: u16,
}
```

## Optional Development Tools

### Rustfmt

| | |
|---|---|
| **What** | Rust code formatter |
| **Version** | Latest |
| **Install** | `rustup component add rustfmt` |

**Why we use it**: Ensures consistent code style.

**How we use it**:
```bash
cargo fmt --all --check
```

### Clippy

| | |
|---|---|
| **What** | Rust linter |
| **Version** | Latest |
| **Install** | `rustup component add clippy` |

**Why we use it**: Catches common mistakes and suggests idiomatic patterns.

**How we use it**:
```bash
cargo clippy --workspace -- -D warnings
```

### AWS CLI (Optional)

| | |
|---|---|
| **What** | AWS command-line interface |
| **Version** | 2.0+ |
| **Install** | https://aws.amazon.com/cli/ |

**Why we use it**: Testing CloudEmu AWS compatibility.

**How we use it**:
```bash
export AWS_ENDPOINT_URL=http://localhost:4566
aws s3 mb s3://test-bucket
```

## Version Matrix

| Tool | Minimum | Recommended | Notes |
|------|---------|-------------|-------|
| Rust | 1.70 | 1.75+ | Async traits stabilized in 1.75 |
| Cargo | 1.70 | 1.75+ | Bundled with Rust |
| Tokio | 1.35 | Latest | Specified in Cargo.toml |
| Axum | 0.7 | 0.7+ | Uses Tokio 1.x |
| Clap | 4.5 | Latest | Derive macros |

## Verification

### Check Toolchain

```bash
# Verify Rust installation
rustc --version
# Expected: rustc 1.75.0 (or higher)

cargo --version
# Expected: cargo 1.75.0 (or higher)

# Verify components
rustup component list | grep -E 'rustfmt|clippy'
# Expected: rustfmt-x86_64-...-installed
#           clippy-x86_64-...-installed
```

### Build and Test

```bash
# Navigate to cloudemu-server
cd crates/cloudemu-server

# Check compilation
cargo check
# Expected: Finished dev [unoptimized + debuginfo]

# Run tests
cargo test
# Expected: test result: ok. 1 passed

# Run server
cargo run
# Expected: Server listening on ports 4566, 4567, 4568
```

### Verify Runtime

```bash
# Start server in background
cargo run -p cloudemu-server &

# Test AWS endpoint
curl http://localhost:4566/health
# Expected: 200 OK

# Test Azure endpoint
curl http://localhost:4567/devstoreaccount1/?comp=list
# Expected: 200 OK with XML response

# Test GCP endpoint (skeleton)
curl http://localhost:4568/
# Expected: 404 (no services yet)
```

---

**Last Updated**: 2026-01-14
