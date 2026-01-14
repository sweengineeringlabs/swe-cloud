# Changelog

All notable changes to the CloudKit framework will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-13

### Added
- **Initial SEA Refactoring**: Completed the 5-layer Stratified Encapsulation Architecture.
- **Unified Provider Structure**: AWS, Azure, GCP, and Oracle providers consolidated under `cloudkit_core`.
- **Expanded Service Support**: Storage, Message Queues, PubSub, and Key-Value stores implemented for AWS and GCP.
- **Provider-Agnostic Facade**: Public API moved to `cloudkit_facade` for better ergonomics.

### Changed
- Moved provider-specific logic out of the main `cloudkit` crate into dedicated crates.
- Standardized error handling using `CloudError` in `cloudkit_spi`.
- Unified configuration handling via `CloudConfig`.

### Fixed
- GCP PubSub implementation issues.
- AWS Lambda runtime mapping.
- Memory leaks in internal executors.
