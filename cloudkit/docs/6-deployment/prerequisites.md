# Prerequisites

System requirements and setup for CloudKit SDK development and usage.

## System Requirements

### Operating Systems

- **Linux**: Ubuntu 20.04+ or equivalent
- **macOS**: 11.0+ (Big Sur or later)
- **Windows**: 10/11 (native or WSL2)

### Required Software

| Tool | Minimum Version | Recommended | Purpose |
|------|----------------|-------------|---------|
| **Rust** | 1.70+ | 1.75+ | Language |
| **Cargo** | 1.70+ | 1.75+ | Build system |
| **Git** | 2.30+ | Latest | Version control |

### Cloud Provider SDKs (Optional)

| Provider | Requirement | Purpose |
|----------|-------------|---------|
| **AWS** | AWS credentials configured | AWS operations |
| **Azure** | Azure CLI + authentication | Azure operations |
| **GCP** | gcloud SDK + auth | GCP operations |
| **Oracle** | OCI CLI (future) | Oracle Cloud (planned) |

## Installation

### 1. Install Rust Toolchain

**Linux/macOS**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows**:
- Download from [rustup.rs](https://rustup.rs/)

**Verify**:
```bash
rustc --version  # 1.70+
cargo --version
```

### 2. Clone Repository

```bash
git clone https://github.com/sweengineeringlabs/swe-cloud.git
cd cloud/cloudkit
```

### 3. Build CloudKit

```bash
# Build all crates
cargo build --workspace

# Run tests (requires cloud credentials)
cargo test --workspace

# Build with WASM support
cargo build --target wasm32-unknown-unknown --package cloudkit_spi
```

## Cloud Provider Setup

### AWS Configuration

```bash
# Install AWS CLI
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install

# Configure credentials
aws configure
# Enter: Access Key ID, Secret Access Key, Region, Output format
```

**Environment Variables**:
```bash
export AWS_REGION=us-east-1
export AWS_ACCESS_KEY_ID=your_key
export AWS_SECRET_ACCESS_KEY=your_secret
```

### Azure Configuration

```bash
# Install Azure CLI
curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash

# Login
az login
```

**Environment Variables**:
```bash
export AZURE_TENANT_ID=your_tenant_id
export AZURE_CLIENT_ID=your_client_id
export AZURE_CLIENT_SECRET=your_secret
```

### GCP Configuration

```bash
# Install gcloud SDK
curl https://sdk.cloud.google.com | bash
exec -l $SHELL

# Initialize and login
gcloud init
gcloud auth application-default login
```

**Environment Variables**:
```bash
export GOOGLE_APPLICATION_CREDENTIALS=/path/to/service-account.json
export GCP_PROJECT_ID=your_project_id
```

## Development Components

### Rust Components

```bash
# Formatters and linters
rustup component add rustfmt
rustup component add clippy

# WASM target
rustup target add wasm32-unknown-unknown
```

### Optional Development Tools

```bash
# Cargo tools
cargo install cargo-watch    # Auto-rebuild on changes
cargo install cargo-expand   # Macro expansion
cargo install cargo-audit    # Security audits
```

## Verification

### Check Prerequisites

```bash
# Verify Rust installation
rustc --version
cargo --version

# Verify cloud CLIs (if installed)
aws --version
az --version
gcloud --version
```

### Build and Test

```bash
cd cloudkit

# Build all crates
cargo build --workspace

# Run unit tests (no cloud required)
cargo test --lib --workspace

# Run integration tests (requires cloud credentials)
cargo test --test '*' --workspace

# Check WASM compatibility
cargo check --target wasm32-unknown-unknown -p cloudkit_spi
```

### Expected Output

```
Compiling cloudkit_spi v0.1.0
Compiling cloudkit_api v0.1.0
Compiling cloudkit_core v0.1.0
Compiling cloudkit v0.1.0
    Finished dev [unoptimized + debuginfo] target(s)
```

## Troubleshooting

### Rust Issues

**Problem**: Old Rust version  
**Solution**: `rustup update stable`

**Problem**: Missing components  
**Solution**: `rustup component add rustfmt clippy`

### Cloud Credential Issues

**AWS**: Verify with `aws sts get-caller-identity`  
**Azure**: Verify with `az account show`  
**GCP**: Verify with `gcloud auth list`

### WASM Build Issues

**Problem**: WASM target not installed  
**Solution**: `rustup target add wasm32-unknown-unknown`

**Problem**: std not available for WASM  
**Solution**: CloudKit SPI is no_std compatible, use `cloudkit_spi` only

## Next Steps

- See [Installation Guide](installation.md) for library usage
- See [Developer Guide](../4-development/developer-guide.md) for contributing
- See [Architecture](../3-design/architecture.md) for system design
- See [WASM Guide](../4-development/guide/wasm.md) for edge deployment

---

**Last Updated**: 2026-01-14
