# CloudEmu Documentation Hub

Welcome to the CloudEmu documentation. CloudEmu is a production-grade local cloud emulator (AWS, Azure, GCP) designed for development, testing, and offline workflows.

## WHAT: Local Cloud Development Environment

CloudEmu provides a fast, accurate emulation of AWS, Azure, and GCP services that runs entirely on your local machine using a unified storage engine.

## Quick Start

### 1. Start the Emulator

```bash
cargo run --bin cloudemu-server
```

This starts the multi-cloud server listening on:
- **AWS**: `http://localhost:4566`
- **Azure**: `http://localhost:4567`
- **GCP**: `http://localhost:4568`

### 2. Configure Your Tools

#### AWS (S3, DynamoDB, etc.)
```bash
export AWS_ENDPOINT_URL=http://localhost:4566
```

#### Azure (Blob Storage, etc.)
Set your connection string to point to localhost:
```bash
export AZURE_STORAGE_CONNECTION_STRING="DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;BlobEndpoint=http://127.0.0.1:4567/devstoreaccount1;"
```

#### GCP (Cloud Storage, etc.)
Configure the storage emulator host:
```bash
export STORAGE_EMULATOR_HOST=http://localhost:4568
```

## Core Documentation

- **[Glossary](./glossary.md)**: Key terms and service mappings.
- **[Architecture](./3-design/architecture.md)**: Control Plane and Data Plane design.
- **[Storage Engine](./3-design/storage-engine-architecture.md)**: Deep dive into the "X-backed" storage implementation.
- **[Implementation Status](./3-design/implementation-status.md)**: Current feature matrix.

## Configuration

| Environment Variable | Default | Description |
| :--- | :--- | :--- |
| `CLOUDEMU_HOST` | `0.0.0.0` | Bind address |
| `CLOUDEMU_PORT` | `4566` | AWS HTTP listen port |
| `CLOUDEMU_AZURE_PORT` | `4567` | Azure HTTP listen port |
| `CLOUDEMU_GCP_PORT` | `4568` | GCP HTTP listen port |
| `CLOUDEMU_DATA_DIR` | `.cloudemu` | Persistent data directory |

---

**Last Updated**: 2026-01-15  
**Version**: 1.0.0 (Feature Complete)
