# CloudEmu

**Unified Multi-Cloud Emulator for Local Development (AWS, Azure, GCP)**

CloudEmu is a fast, production-like local cloud emulator. It allows you to develop and test cloud applications locally by emulating APIs for AWS, Azure, and Google Cloud Platform (GCP).

## Quick Start

```bash
# Start the unified multi-cloud server
cargo run -p cloudemu-server
```

The server will start listening on the following ports:
- **AWS**: `http://localhost:4566` (e.g., S3, DynamoDB)
- **Azure**: `http://localhost:4567` (e.g., Blob Storage)
- **GCP**: `http://localhost:4568` (Connectivity Only)

## Supported Clouds

| Cloud Provider | Port | Status | Services Emulated |
| :--- | :--- | :--- | :--- |
| **AWS** | 4566 | ✅ Stable | S3, DynamoDB, SQS, SNS, Lambda, KMS, Secrets Manager, CloudWatch, EventBridge, Cognito, Step Functions |
| **Azure** | 4567 | 🔄 Beta | Blob Storage (Basic emulation) |
| **GCP** | 4568 | 🚧 Alpha | Connectivity Only (Skeleton) |

## Usage Examples

### AWS CLI
```bash
export AWS_ENDPOINT_URL=http://localhost:4566
aws s3 mb s3://my-bucket
aws s3 ls
```

### Azure (Blob Storage)
Use with standard Azure connection strings or direct HTTP calls.

```bash
# Check service health / List containers
curl "http://localhost:4567/devstoreaccount1/?comp=list"
```

## Features

- 🎯 **Multi-Cloud Support** - Orchestrates emulators for AWS, Azure, and GCP in a single process.
- 🏗️ **Terraform Compatible** - Deploy infrastructure locally using standard providers.
- 💾 **Persistent Storage** - Metadata and data persisted locally (default: `.cloudemu` directory).
- 🚀 **Fast Startup** - Async Rust implementation for millisecond startup times.

## Documentation

For comprehensive documentation, see the **[Documentation Hub](./doc/overview.md)**.

### Quick Links
- [Architecture](./doc/3-design/architecture.md)
- [Implementation Status](./doc/3-design/implementation-status.md)
- [Testing Strategy](./doc/5-testing/testing-strategy.md)

## Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

MIT - See [LICENSE](./LICENSE) for details.

---

**Start testing your cloud infrastructure locally, today! 🚀**
