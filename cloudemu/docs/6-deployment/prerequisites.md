# Prerequisites

System requirements and setup instructions for CloudEmu development and usage.

## System Requirements

### Operating Systems

- **Linux**: Ubuntu 20.04+ or equivalent
- **macOS**: 11.0+ (Big Sur or later)
- **Windows**: 10/11 with WSL2 recommended

### Required Software

| Tool | Minimum Version | Recommended | Purpose |
|------|----------------|-------------|---------|
| **Rust** | 1.70+ | 1.75+ | Core language |
| **Cargo** | 1.70+ | 1.75+ | Build tool |
| **Git** | 2.30+ | Latest | Version control |

### Optional Tools

| Tool | Version | Purpose |
|------|---------|---------|
| **AWS CLI** | 2.0+ | Testing AWS compatibility |
| **Azure CLI** | 2.40+ | Testing Azure compatibility |
| **Terraform** | 1.5+ | Infrastructure testing |
| **Docker** | 20.10+ | Container deployment |

## Installation

### 1. Install Rust

**Linux/macOS**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows**:
- Download from [rustup.rs](https://rustup.rs/)
- Or use WSL2 and follow Linux instructions

**Verify**:
```bash
rustc --version  # Should show 1.70+
cargo --version
```

### 2. Install Git

**Linux (Debian/Ubuntu)**:
```bash
sudo apt update
sudo apt install git
```

**macOS**:
```bash
brew install git
```

**Windows**:
- Download from [git-scm.com](https://git-scm.com/)

### 3. Clone Repository

```bash
git clone https://github.com/sweengineeringlabs/swe-cloud.git
cd cloud/cloudemu
```

### 4. Build CloudEmu

```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Build release binary
cargo build --release -p cloudemu-server
```

## Development Dependencies

### Additional Rust Components

```bash
# Format checker
rustup component add rustfmt

# Linter
rustup component add clippy
```

### Database (for data-plane)

CloudEmu uses SQLite (included in data-plane crate, no separate installation needed).

## Verification

Run the verification script:

```bash
# Check all prerequisites
rustc --version
cargo --version
git --version

# Build and test
cd cloudemu
cargo build --workspace
cargo test --workspace

# Run server
cargo run -p cloudemu-server
```

Expected output:
```
Starting CloudEmu Unified Server
...
AWS Service   : http://127.0.0.1:4566
Azure Service : http://127.0.0.1:10000
GCP Service   : http://127.0.0.1:4567
Oracle Service: http://127.0.0.1:4568
```

## Troubleshooting

### Rust Installation Issues

**Problem**: `rustc` not found  
**Solution**: Add to PATH: `source $HOME/.cargo/env`

**Problem**: Old Rust version  
**Solution**: Update rustup: `rustup update`

### Build Issues

**Problem**: Compilation errors  
**Solution**: Clean and rebuild:
```bash
cargo clean
cargo build --workspace
```

**Problem**: Test failures  
**Solution**: Check internet connection (some tests may require AWS/Azure endpoints)

### Platform-Specific

**Windows**: Use WSL2 for best compatibility  
**macOS**: Ensure Xcode Command Line Tools installed: `xcode-select --install`  
**Linux**: Install build essentials: `sudo apt install build-essential`

## Next Steps

- See [Developer Guide](../4-development/developer-guide.md) for contribution workflow
- See [Architecture](../3-design/architecture.md) for system design

---

**Last Updated**: 2026-01-16
