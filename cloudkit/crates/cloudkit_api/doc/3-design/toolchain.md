# CloudKit API Toolchain

## Overview
This crate is the reference for all service contracts. Tooling focuses on trait documentation and type safety.

## Tools

### Rustdoc
| | |
|---|---|
| **What** | Documentation generator |
| **Version** | Native Cargo |

**Why we use it**: To generate the API reference that serves as the blueprint for all provider implementations.

### Async-Trait
| | |
|---|---|
| **What** | Proc-macro for async traits |
| **Version** | `0.1` |

**Why we use it**: Required to support async functions in traits until the feature is fully stabilized in the Rust language.

## Version Matrix
| Component | Version |
|-----------|---------|
| cloudkit_spi | 0.1 |
| serde | 1.0 |

## Verification
Check documentation and trait validity:
```bash
cargo check
cargo doc --no-deps
```
