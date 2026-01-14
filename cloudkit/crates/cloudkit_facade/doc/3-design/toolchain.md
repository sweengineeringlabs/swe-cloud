# CloudKit Facade Toolchain

## Overview
The facade crate utilizes standard Rust tooling for building, testing, and verifying the public API.

## Tools

### Rust (Cargo)
| | |
|---|---|
| **What** | Build system and package manager |
| **Version** | `1.85+` |
| **Install** | `rustup update` |

**Why we use it**: Standard Rust toolchain for reliability and performance.

### LLVM-COV
| | |
|---|---|
| **What** | Code coverage reporting tool |
| **Version** | `latest` |
| **Install** | `cargo install cargo-llvm-cov` |

**Why we use it**: Ensures high test coverage for the public API surface.

## Version Matrix
| Tool/Crate | Minimum | Recommended |
|------------|---------|-------------|
| Rust | 1.85 | Latest Stable |
| Tokio | 1.42 | 1.42+ |
| CloudKit API| 0.1 | 0.1 |

## Verification
Run all tests including provider-specific units:
```bash
cargo test --all-features
```
