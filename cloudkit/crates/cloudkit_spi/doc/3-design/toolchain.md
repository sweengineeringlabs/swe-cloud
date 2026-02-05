# Toolchain

## Overview

CloudKit SPI requires minimal dependencies for maximum portability including WASM compatibility.

## Tools

### Rust Compiler (no_std compatible)

| | |
|---|---|
| **What** | Rust compiler with no_std support |
| **Version** | 1.70+ (recommended: 1.75+) |
| **Install** | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |

**Why we use it**: SPI is the foundation layer, must work in embedded and WASM environments.

**How we use it**:
```bash
# Native build
cargo build -p cloudkit_spi

# WASM build
cargo build -p cloudkit_spi --target wasm32-unknown-unknown
```

### WASM Target

| | |
|---|---|
| **What** | WebAssembly compilation target |
| **Version** | wasm32-unknown-unknown |
| **Install** | `rustup target add wasm32-unknown-unknown` |

**Why we use it**: Enables CloudKit deployment to browsers and edge computing platforms.

**How we use it**:
```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Build for WASM
cargo build -p cloudkit_spi --target wasm32-unknown-unknown

# Verify no_std compatibility
cargo check -p cloudkit_spi --target wasm32-unknown-unknown --no-default-features
```

### Serde (optional feature)

| | |
|---|---|
| **What** | Serialization framework |
| **Version** | 1.0+ (no_std compatible) |
| **Install** | Automatic via Cargo (feature-gated) |

**Why we use it**: Serialize error types and configuration when std is available.

**How we use it**:
```rust
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CloudError {
    pub kind: ErrorKind,
    pub message: String,
}
```

## Version Matrix

| Tool | Minimum | Recommended | Purpose |
|------|---------|-------------|---------|
| Rust | 1.70 | 1.75+ | Core language |
| rustc (no_std) | 1.70 | 1.75+ | Embedded support |
| wasm32 target | - | Latest | WASM compilation |
| serde | 1.0 | Latest | Optional serialization |

## Verification

### Standard Build

```bash
# Build with std
cargo build -p cloudkit_spi
# Expected: Finished dev

# Run tests (requires std)
cargo test -p cloudkit_spi
# Expected: test result: ok
```

### WASM Build

```bash
# Verify WASM target installed
rustup target list | grep wasm32-unknown-unknown
# Expected: wasm32-unknown-unknown (installed)

# Build for WASM
cargo build -p cloudkit_spi --target wasm32-unknown-unknown
# Expected: Finished dev

# Check no_std compatibility
cargo check -p cloudkit_spi --target wasm32-unknown-unknown --no-default-features
# Expected: Finished dev
```

### Feature Verification

```bash
# Build with all features
cargo build -p cloudkit_spi --all-features
# Expected: Includes serde support

# Build without features
cargo build -p cloudkit_spi --no-default-features
# Expected: Minimal no_std build
```

### Integration Test

```bash
# Create WASM test
cat > crates/cloudkit_spi/examples/wasm_compat.rs << 'EOF'
#![no_std]

use cloudkit_spi::{CloudError, ErrorKind};

#[no_mangle]
pub extern "C" fn test_error() -> i32 {
    let err = CloudError::new(ErrorKind::NotFound, "test");
    if err.kind() == ErrorKind::NotFound {
        0 // Success
    } else {
        1 // Failure
    }
}
EOF

# Build for WASM
cargo build --example wasm_compat --target wasm32-unknown-unknown -p cloudkit_spi
# Expected: Finished dev
```

## Platform-Specific Notes

### Native (std)
- Full error messages
- Heap allocation available
- Standard collections (HashMap, Vec)

### WASM (no_std)
- Limited error messages
- No heap allocation (or custom allocator)
- Must use core::* types only

### Embedded (no_std)
- Same constraints as WASM
- May have platform-specific atomics
- Custom memory management

## Dependencies

CloudKit SPI intentionally minimizes dependencies:
- **Zero required dependencies** for no_std builds
- **Serde** only when "serde" feature enabled
- **No async runtime** - traits defined in consuming crates

---

**Last Updated**: 2026-01-14
