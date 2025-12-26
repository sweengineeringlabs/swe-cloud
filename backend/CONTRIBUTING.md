# Contributing to CloudKit

Thank you for your interest in contributing to CloudKit! This document provides guidelines and information for contributors.

## Code of Conduct

Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- Rust 1.85 or later
- Cargo
- Git
- (Optional) Cloud provider credentials for integration tests

### Setup

1. Fork and clone the repository:
   ```bash
   git clone https://github.com/YOUR_USERNAME/cloudkit.git
   cd cloudkit
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Run lints:
   ```bash
   cargo clippy --all-targets --all-features
   cargo fmt --check
   ```

## Development Workflow

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation
- `refactor/description` - Code refactoring

### Commits

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation
- `refactor:` - Code refactoring
- `test:` - Adding tests
- `chore:` - Maintenance

Example:
```
feat(aws): implement S3 multipart upload

- Add MultipartUpload struct
- Implement create, upload_part, complete methods
- Add tests for large file uploads
```

### Pull Requests

1. Create a feature branch from `main`
2. Make your changes
3. Write/update tests
4. Update documentation
5. Run `cargo fmt` and `cargo clippy`
6. Create a PR with a clear description

## Architecture Guidelines

### SEA Layers

When adding code, place it in the correct layer:

| Layer | Purpose | Dependencies |
|-------|---------|--------------|
| Common | Shared types | None |
| SPI | Extension traits | Common |
| API | Service traits | Common |
| Core | Implementations | Common, SPI, API |
| Facade | Public API | All |

### Adding a New Service Trait

1. Create the trait in `src/api/`:
   ```rust
   // src/api/new_service.rs
   #[async_trait]
   pub trait NewService: Send + Sync {
       async fn operation(&self, param: &str) -> CloudResult<()>;
   }
   ```

2. Export from `src/api/mod.rs`
3. Add to prelude in `src/prelude.rs`
4. Update documentation

### Adding a Provider Implementation

1. Create implementation in provider crate:
   ```rust
   // cloudkit-aws/src/new_service.rs
   pub struct AwsNewService {
       context: Arc<CloudContext>,
   }

   #[async_trait]
   impl NewService for AwsNewService {
       async fn operation(&self, param: &str) -> CloudResult<()> {
           // Implementation
       }
   }
   ```

2. Add to builder in `builder.rs`
3. Export from `lib.rs`
4. Add tests
5. Update documentation

## Testing

### Unit Tests

Place tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // ...
    }

    #[tokio::test]
    async fn test_async_something() {
        // ...
    }
}
```

### Integration Tests

Place in `tests/` directory:

```rust
// tests/integration_test.rs
use cloudkit::prelude::*;

#[tokio::test]
#[ignore] // Requires credentials
async fn test_real_s3() {
    // ...
}
```

### Mock Testing

Use the `MockStorage` pattern from `examples/testing.rs`:

```rust
let mock = MockStorage::with_data("bucket", "key", b"data");
```

## Documentation

### Code Documentation

- All public items must have doc comments
- Include examples in doc comments
- Use `#![warn(missing_docs)]`

```rust
/// Uploads data to cloud storage.
///
/// # Arguments
///
/// * `bucket` - The bucket name
/// * `key` - The object key
/// * `data` - The data to upload
///
/// # Example
///
/// ```rust,ignore
/// storage.put_object("bucket", "key", b"data").await?;
/// ```
///
/// # Errors
///
/// Returns `CloudError::Auth` if credentials are invalid.
pub async fn put_object(&self, bucket: &str, key: &str, data: &[u8]) -> CloudResult<()>;
```

### Markdown Documentation

- Update relevant `.md` files in `docs/`
- Keep examples up to date
- Fix broken links

## Style Guide

### Formatting

Run `cargo fmt` before committing.

### Clippy

Fix all Clippy warnings:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Naming

- Use snake_case for functions and variables
- Use PascalCase for types and traits
- Use SCREAMING_SNAKE_CASE for constants
- Prefer descriptive names over abbreviations

### Error Handling

- Return `CloudResult<T>` from all public async functions
- Use `?` operator for error propagation
- Provide context in error messages
- Map provider errors to `CloudError`

## Release Process

1. Update version in all `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Create release PR
4. Tag release after merge
5. Publish to crates.io

## Questions?

- Open an issue for questions
- Check existing issues first
- Be specific about your problem

Thank you for contributing! 🚀
