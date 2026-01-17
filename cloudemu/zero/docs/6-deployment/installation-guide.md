# ZeroCloud Installation Guide

## 1. Prerequisites

- **Rust Toolchain**: 1.70+ (`rustup update stable`)
- **Docker** (Optional, for strictly isolated compute workloads)

## 2. building from Source

ZeroCloud is part of the `cloudemu` workspace.

```bash
cd cloudemu
cargo build --release -p cloudemu-server
```

## 3. Running the Server

```bash
# Default Configuration (Ports 4566-4568)
cargo run --release -p cloudemu-server
```

To enable the **ZeroCloud Native Endpoint** (Port 8080):

```bash
export ZERO_PORT=8080
cargo run --release -p cloudemu-server
```

## 4. Troubleshooting

- **Port Conflicts**: Ensure ports 4566, 4567, 4568, and 8080 are free.
- **Permissions**: ZeroStore writes to `.cloudemu/`, ensure write permissions.
