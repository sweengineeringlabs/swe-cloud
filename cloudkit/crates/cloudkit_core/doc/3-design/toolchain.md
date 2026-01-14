# CloudKit Core Toolchain

## Overview
`cloudkit_core` relies on a multi-provider test suite and mock frameworks to ensure orchestration logic is sound across all targets.

## Tools

### Mockall
| | |
|---|---|
| **What** | Double library for Rust |
| **Version** | `0.13` |
| **Install** | Included in dev-dependencies |

**Why we use it**: Essential for testing the `OperationExecutor` and `CloudContext` without incurring cloud costs or latency.

### Cargo Features
| | |
|---|---|
| **What** | Conditional compilation |
| **Version** | Native Cargo |

**Why we use it**: To keep the dependency tree lean, allowing users to only compile the provider implementations they need.

## Version Matrix
| Provider SDK | Version | Notes |
|--------------|---------|-------|
| AWS SDK | 1.0+ | Official Rust SDK |
| Azure SDK | 0.21 | Unofficial (community) |
| GCP SDK | 0.29 | Community crates |

## Verification
Verify a specific provider implementation:
```bash
cargo test -p cloudkit-aws
```
