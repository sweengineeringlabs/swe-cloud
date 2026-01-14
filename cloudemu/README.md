# CloudEmu

**Production-Grade Local Cloud Emulator for AWS Services**

CloudEmu is a fast, production-like AWS service emulator designed for local development and testing. It provides accurate AWS API responses and seamlessly integrates with Terraform, AWS SDKs, and the AWS CLI.

## Quick Start

```bash
# Start the emulator
cargo run -p cloudemu

# Use with Terraform or AWS CLI
export AWS_ENDPOINT_URL=http://localhost:4566
aws s3 mb s3://my-bucket
```

## Features

- 🎯 **Production-Like Behavior** - Accurate AWS API responses
- 🏗️ **Terraform Compatible** - Deploy infrastructure locally
- 💾 **Persistent Storage** - SQLite metadata + filesystem blobs
- 🔄 **S3 Versioning** - Full version control workflow
- 🚀 **Fast Startup** - Ready in milliseconds

## Supported Services

| Service | Implementation Status |
| :--- | :--- |
| **S3** | ✅ Full versioning, policies, metadata |
| **DynamoDB** | ✅ Basic CRUD operations |
| **SQS** | ✅ Message queues with visibility timeouts |
| **SNS** | ✅ Topics and subscriptions |
| **Lambda** | ✅ Function management (mock invocations) |
| **Secrets Manager** | ✅ Secret storage and versioning |
| **KMS** | ✅ Key management and encryption |
| **EventBridge** | ✅ Event buses and rules |
| **CloudWatch** | ✅ Metrics and log streams |
| **Cognito** | ✅ User pools and authentication |
| **Step Functions** | ✅ State machine tracking |

## Documentation

For comprehensive documentation, see the **[Documentation Hub](./doc/overview.md)**.

### Quick Links
- [Getting Started](./doc/overview.md#quick-start)
- [Architecture](./doc/3-design/architecture.md)
- [Backlog](./doc/4-development/backlog.md)

## Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

MIT - See [LICENSE](./LICENSE) for details.

---

**Start testing your cloud infrastructure locally, today! 🚀**
