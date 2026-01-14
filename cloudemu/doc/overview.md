# CloudEmu Documentation Hub

Welcome to the CloudEmu documentation. CloudEmu is a production-grade local cloud emulator (AWS, Azure, GCP) designed for development, testing, and offline workflows.

## WHAT: Local Cloud Development Environment

CloudEmu provides a fast, accurate emulation of AWS, Azure, and GCP services that runs entirely on your local machine.

...

## Quick Start

### 1. Start the Emulator

```bash
cargo run -p cloudemu-server
```

### 2. Configure Your Tools

```bash
export AWS_ENDPOINT_URL=http://localhost:4566
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test
```

### 3. Use with Terraform

```hcl
provider "aws" {
  endpoints {
    s3 = "http://localhost:4566"
  }
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  s3_use_path_style           = true
}
```

### 4. Deploy Infrastructure

```bash
terraform init
terraform apply
```

## Core Documentation

- **[Glossary](./glossary.md)**: Key terms and AWS service mappings.
- **[Architecture](./3-design/architecture.md)**: Control Plane and Data Plane design.
- **[Backlog](./4-development/backlog.md)**: Feature roadmap and priorities.

## Configuration

| Environment Variable | Default | Description |
| :--- | :--- | :--- |
| `CLOUDEMU_HOST` | `0.0.0.0` | Bind address |
| `CLOUDEMU_PORT` | `4566` | HTTP listen port |
| `CLOUDEMU_DATA_DIR` | `.cloudemu` | Persistent data directory |
| `CLOUDEMU_REGION` | `us-east-1` | Default AWS region |
| `CLOUDEMU_ACCOUNT_ID` | `000000000000` | Mock AWS account ID |

---

**Last Updated**: 2026-01-14  
**Version**: 0.1.0
