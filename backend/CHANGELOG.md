# Changelog

All notable changes to CloudKit will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure with SEA architecture
- Core `cloudkit` crate with five-layer architecture:
  - Common layer: `CloudError`, `Region`, `Credentials`, `CloudConfig`
  - SPI layer: `AuthProvider`, `RetryPolicy`, `MetricsCollector`, `Logger`
  - API layer: `ObjectStorage`, `KeyValueStore`, `MessageQueue`, `PubSub`, `Functions`
  - Core layer: `CloudContext`, `OperationExecutor`
  - Facade layer: `CloudKit` entry point
- AWS provider crate (`cloudkit-aws`) with stubs for:
  - S3 (ObjectStorage)
  - DynamoDB (KeyValueStore)
  - SQS (MessageQueue)
  - SNS (PubSub)
  - Lambda (Functions)
- Azure provider crate (`cloudkit-azure`) with builder
- GCP provider crate (`cloudkit-gcp`) with builder
- Oracle provider crate (`cloudkit-oracle`) with builder
- Comprehensive documentation:
  - Quick Start Guide
  - Architecture Overview
  - Error Handling Guide
  - Configuration Guide
  - Provider Documentation
  - SPI Documentation
- Examples:
  - Basic usage
  - Multi-cloud
  - Custom provider (SPI)
  - Error handling patterns
  - Testing with mocks

### Changed
- N/A (initial release)

### Deprecated
- N/A (initial release)

### Removed
- N/A (initial release)

### Fixed
- N/A (initial release)

### Security
- N/A (initial release)

---

## [0.1.0] - TBD

### Added
- First public release
- Full AWS S3 implementation
- Full AWS DynamoDB implementation
- Basic Azure Blob Storage support
- Basic GCP Cloud Storage support

### Provider Status

| Provider | Status | Services |
|----------|--------|----------|
| AWS | Alpha | S3, DynamoDB |
| Azure | Planned | Blob Storage |
| GCP | Planned | Cloud Storage |
| Oracle | Planned | Object Storage |

---

## Future Roadmap

### 0.2.0 (Planned)
- Complete AWS SQS implementation
- Complete AWS SNS implementation
- AWS Lambda invocation
- Azure Cosmos DB support
- Improved error messages

### 0.3.0 (Planned)
- GCP Pub/Sub implementation
- GCP Firestore support
- Oracle Object Storage
- Connection pooling
- Request batching

### 0.4.0 (Planned)
- Multi-region support
- Cross-account access
- Enhanced observability
- Performance optimizations

### 1.0.0 (Planned)
- Stable API
- Complete documentation
- Full test coverage
- Production-ready

---

## Migration Guides

### From 0.0.x to 0.1.0

No breaking changes expected as this is the initial release.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute to CloudKit.
