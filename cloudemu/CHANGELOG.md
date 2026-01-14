# Changelog

All notable changes to **CloudEmu** will be documented in this file.

## [0.1.0] - 2026-01-14

### Added
- Initial architectural refactor into Control Plane and Data Plane.
- Advanced S3 support: full versioning, bucket policies, and object metadata.
- Persistent storage engine using SQLite (metadata) and Filesystem (blobs).
- Basic implementations for DynamoDB, SQS, SNS, and Lambda.
- Unified configuration and error handling.
- Comprehensive integration with Terraform and AWS SDKs.

### Fixed
- Improved thread safety for concurrent bucket operations.
- Resolved ID mapping issues in SQS message retrieval.

---

*Format based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).*
