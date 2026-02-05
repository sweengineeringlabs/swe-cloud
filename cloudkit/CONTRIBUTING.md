# Contributing to CloudKit

Thank you for your interest in contributing to CloudKit! This project aims to provide a high-quality, unified Rust SDK for multi-cloud operations.

## How to Contribute

### 1. Reporting Issues
- Use GitHub Issues to report bugs or request features.
- Provide a clear description and reproduction steps.
- Include crate versions and Rust version.

### 2. Adding a New Service
1. **Define API**: Add a new trait in `cloudkit_api`.
2. **Implement Core**: Implement the trait for relevant providers (AWS, Azure, GCP) in `cloudkit_core`.
3. **Update Facade**: Re-export the new service in `cloudkit_facade`.
4. **Documentation**: Create/Update documentation in `docs/` and crate-specific `overview.md`.

### 3. Improving Provider Support
- Modify the provider-specific crates under `cloudkit/crates/cloudkit_core/[provider]`.
- Ensure all tests pass with `--all-features`.

## Development Guidelines

### Coding Standards
- Follow standard Rust naming conventions.
- Run `cargo fmt` before committing.
- Ensure `cargo clippy` is clean.
- Document all public items.

### Testing Requirements
- Every new feature must have unit tests.
- Integration tests in `examples/` or `tests/` are highly encouraged.
- Mock external dependencies where possible.

## Pull Request Process
1. Fork and branch from `master`.
2. Make changes following the guidelines.
3. Verify tests pass locally.
4. Submit PR with a clear description of WHAT and WHY.

## License
By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for your contributions! 🙏**
