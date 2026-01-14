# CloudKit SPI Toolchain

## Overview
This foundational crate focus on primitive types and standardized error definitions.

## Tools

### Serde
| | |
|---|---|
| **What** | Serialization framework |
| **Version** | `1.0` |

**Why we use it**: To allow foundational types like `Region` and `Credentials` to be easily serialized/deserialized from configuration files.

### Thiserror
| | |
|---|---|
| **What** | Error derive macro |
| **Version** | `2.0` |

**Why we use it**: Provides a clean way to define the `CloudError` enum while capturing internal provider details.

## Version Matrix
| Component | Version |
|-----------|---------|
| chrono | 0.4 |
| uuid | 1.11 |

## Verification
```bash
cargo test
```
