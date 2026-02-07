# CloudEmu Documentation Hub

**Audience**: Architects, Developers, DevOps Engineers

Welcome to the CloudEmu documentation. CloudEmu is a production-grade local cloud emulator (AWS, Azure, GCP, Oracle) designed for development, testing, and offline workflows.

## WHAT: Local Cloud Development Environment

CloudEmu provides a fast, accurate emulation of AWS, Azure, GCP, and Oracle services that runs entirely on your local machine using a unified storage engine.

## WHY: Problems Solved

- **Cost**: No cloud bills for development/testing.
- **Speed**: Instant deployment loop (no Waiting for CloudFormation).
- **Offline**: Work anywhere without internet access.
- **Testing**: Deterministic integration tests.

## HOW: Usage

### 1. Start the Emulator

```bash
cargo run --bin cloudemu-server
```

This starts the multi-cloud server listening on:
- **AWS**: `http://localhost:4566`
- **Azure**: `http://localhost:10000`
- **GCP**: `http://localhost:4567`
- **Oracle**: `http://localhost:4568`

### 2. Configure Your Tools

#### AWS (S3, DynamoDB, etc.)
```bash
export AWS_ENDPOINT_URL=http://localhost:4566
```

#### Azure (Blob Storage, etc.)
Set your connection string to point to localhost:
```bash
export AZURE_STORAGE_CONNECTION_STRING="DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;BlobEndpoint=http://127.0.0.1:10000/devstoreaccount1;"
```

#### GCP (Cloud Storage, etc.)
Configure the storage emulator host:
```bash
export STORAGE_EMULATOR_HOST=http://localhost:4567
```

## Documentation Map

- **[Glossary](./glossary.md)**: Key terms and service mappings.
- **1-Requirements**: [FinOps Backlog](./1-requirements/finops-backlog.md)
- **3-Design**:
    - [Architecture](./3-design/architecture.md)
    - [Storage Engine](./3-design/storage-engine-architecture.md)
    - [Implementation Status](./3-design/implementation-status.md)
- **4-Development**:
    - [Backlog](./backlog.md)
- **6-Deployment**:
    - [Introduction](./6-deployment/installation.md)
    - [Prerequisites](./6-deployment/prerequisites.md)

## Configuration

| Environment Variable | Default | Description |
| :--- | :--- | :--- |
| `CLOUDEMU_HOST` | `127.0.0.1` | Bind address |
| `CLOUDEMU_AWS_PORT` | `4566` | AWS port |
| `CLOUDEMU_AZURE_PORT` | `10000` | Azure port |
| `CLOUDEMU_GCP_PORT` | `4567` | GCP port |
| `CLOUDEMU_ORACLE_PORT` | `4568` | Oracle port |
| `CLOUDEMU_DATA_DIR` | `.cloudemu` | Persistent data directory |

---

**Last Updated**: 2026-01-16
**Version**: 1.0.1
