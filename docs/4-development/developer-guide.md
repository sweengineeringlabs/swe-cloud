# Developer Guide

**Audience**: Developers, Contributors

## Prerequisites

- Rust 1.85+ (`rustup update stable`)
- Terraform 1.x (for IAC modules)
- Node.js 18+ (for SWE Cloud UI)
- Go 1.21+ (for IAC validation tests)

## Getting Started

```bash
# Clone the repository
git clone https://github.com/sweengineeringlabs/swe-cloud.git
cd cloud

# Build all Rust crates
cargo build --workspace

# Run tests
cargo test --workspace

# Start the emulator
cargo run --bin cloudemu-server
```

## Workspace Structure

```
cloud/
  cloudemu/          — Multi-cloud emulator
    aws/             — AWS provider (control + data plane)
    azure/           — Azure provider
    gcp/             — GCP provider
    oracle/          — Oracle provider
    zero/            — ZeroCloud (private cloud)
    server/          — Unified server binary
  cloudkit/          — Multi-cloud SDK
    crates/          — Workspace crates (spi, api, core, facade)
  iac/               — Infrastructure as Code
    aws/             — AWS Terraform modules
    azure/           — Azure modules
    gcp/             — GCP modules
    facade/          — Unified facade
  apps/
    cloudcost/       — FinOps engine
    swe-cloud-ui/    — Web dashboard
  docs/              — Workspace-level documentation
```

## SEA Architecture

All Rust crates follow the SEA layering pattern. See [Architecture](../3-design/architecture.md) for details.

## Coding Standards

- Format: `cargo fmt --all`
- Lint: `cargo clippy --workspace`
- Test: `cargo test --workspace`
- Docs: `cargo doc --workspace --no-deps`
