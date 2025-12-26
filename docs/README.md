# CloudKit Documentation

Welcome to the CloudKit documentation. CloudKit is a unified multi-cloud SDK for Rust, built using Stratified Encapsulation Architecture (SEA).

## 📚 Documentation Index

### Core Concepts
- [CloudKit Overview](cloudkit-overview.md) - What is CloudKit and why use it
- [WebAssembly Guide](wasm.md) - Understanding WASM and its role

### Getting Started
- [Quick Start Guide](getting-started.md) - Get up and running in 5 minutes
- [Installation](installation.md) - Detailed installation instructions
- [Configuration](configuration.md) - Environment variables and config options

### Architecture
- [Architecture Overview](architecture.md) - SEA layers and design decisions
- [Error Handling](error-handling.md) - Understanding CloudError types
- [Provider Design](providers/README.md) - How providers implement traits

### Providers
- [AWS Provider](providers/aws.md) - S3, DynamoDB, SQS, SNS, Lambda
- [Azure Provider](providers/azure.md) - Blob Storage, Cosmos DB, Service Bus
- [GCP Provider](providers/gcp.md) - Cloud Storage, Pub/Sub, BigQuery
- [Oracle Provider](providers/oracle.md) - Object Storage, Streaming

### API Reference
- [Object Storage](api/object-storage.md) - Blob/object storage operations
- [Key-Value Store](api/kv-store.md) - NoSQL operations
- [Message Queue](api/message-queue.md) - Queue operations
- [Pub/Sub](api/pubsub.md) - Publish-subscribe messaging
- [Functions](api/functions.md) - Serverless invocation

### Extension Points (SPI)
- [Authentication](spi/authentication.md) - Custom auth providers
- [Retry Policies](spi/retry.md) - Custom retry strategies
- [Metrics](spi/metrics.md) - Observability integration
- [Logging](spi/logging.md) - Custom logging

### Examples
- [Basic Usage](../examples/basic_usage.rs) - Simple AWS example
- [Multi-Cloud](../examples/multi_cloud.rs) - Provider-agnostic code
- [Custom Provider](../examples/custom_provider.rs) - SPI implementations
- [Error Handling](../examples/error_handling.rs) - Error handling patterns
- [Testing](../examples/testing.rs) - Testing with mocks

### Contributing
- [Development Setup](contributing/setup.md) - Setting up for development
- [Code Style](contributing/code-style.md) - Rust coding conventions
- [Testing Guide](contributing/testing.md) - How to test changes
- [Release Process](contributing/release.md) - Versioning and releases

## Quick Links

| Resource | Link |
|----------|------|
| API Docs | [docs.rs/cloudkit](https://docs.rs/cloudkit) |
| Repository | [github.com/phdsystems/cloudkit](https://github.com/phdsystems/cloudkit) |
| Issues | [GitHub Issues](https://github.com/phdsystems/cloudkit/issues) |
| Changelog | [CHANGELOG.md](../CHANGELOG.md) |

## Version Compatibility

| CloudKit Version | Rust Version | AWS SDK | Azure SDK | GCP SDK |
|-----------------|--------------|---------|-----------|---------|
| 0.1.x | 1.85+ | 1.x | 0.21.x | 0.22.x |
