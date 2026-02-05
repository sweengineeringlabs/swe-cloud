# Changelog

All notable changes to the Multi-Cloud IAC framework will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-01-14

### Added
- **Multi-Cloud Terratest Suite**: Comprehensive unit testing for all facades (Compute, Storage, DB, Net, IAM, Monitoring, Lambda, Messaging) across AWS, Azure, and GCP.
- **Monitoring Facade**: Unified interface for AWS CloudWatch, Azure Monitor, and GCP Cloud Monitoring.
- **Go-Integrated Validation**: Replaced PowerShell scripts with `validation_test.go` for faster, parallel static analysis.
- **SEA Architecture Documentation**: Clear guides for Design, Migration, and Deployment.
- **GCP Support**: Added missing GCP resource mappings for Storage and Monitoring.

### Changed
- **Refactoring to SEA**: Completed the 5-layer Stratified Encapsulation Architecture (Common, SPI, API, Core, Facade).
- **Consolidated Testing**: Moved all quality checks (Validation + Unit Tests) to a single Go toolchain.
- **Unified Tagging**: Standardized tagging logic across all providers.

### Fixed
- **Messaging Facade**: Fixed missing routing logic for SQS and SNS.
- **Compute Facade**: Removed unused `terraform_remote_state` data source fixing isolated test failures.
- **GCP Monitoring**: Fixed missing `threshold_value` in alert policy resource.

## [0.1.0] - 2026-01-13

### Added
- Initial SEA structure for Compute and Storage.
- AWS core module implementations.
- Basic Azure support for Compute and Storage.
