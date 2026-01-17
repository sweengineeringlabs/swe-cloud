# Contributing to the Multi-Cloud IAC Framework

Thank you for your interest in contributing to the Multi-Cloud IAC framework! This project provides a unified, provider-agnostic interface for deploying infrastructure across AWS, Azure, and GCP using a 5-layer Stratified Encapsulation Architecture (SEA).

## How to Contribute

### 1. Reporting Issues

If you find bugs or have suggestions for new features:

- Check if the issue already exists.
- Provide a clear description and steps to reproduce.
- Include environment details (Terraform version, Go version, Cloud provider).
- Tag appropriately (bug, enhancement, provider-request).

### 2. Adding a New Service Facade

To add a new service (e.g., "Cache" or "BigData"):

1. **API Layer**: Define the provider-agnostic input and output contracts in `api/`.
2. **Core Layer**: Implement the orchestration logic in `[provider]/core/`.
3. **Facade Layer**: Create the user-facing interface in `facade/` that routes to the Core layer.
4. **Documentation**: Add `doc/overview.md` and `doc/3-design/toolchain.md` in the facade directory.
5. **Testing**: Add a Terratest suite (e.g., `cache_test.go`) in the facade directory.

### 3. Improving Provider Support

To improve support for an existing provider (e.g., adding a new database engine to the AWS core module):

1. Modify the relevant core module in `[provider]/core/`.
2. Update the API contract if necessary.
3. Add/Update tests to verify the new functionality.

## Development Guidelines

### Coding Standards

- **Terraform**: Follow standard naming conventions. Use `lowercase-hyphen` for resource names.
- **Go**: Follow standard Go practices. Use `t.Parallel()` in tests.
- **SEA Architecture**: Ensure logic is placed in the correct layer (Facade > Core > API > SPI > Common).

### Testing Requirements

- **Static Validation**: Ensure `go test -v ./validation_test.go` passes.
- **Unit Tests**: Every facade must have a Terratest verifying the `terraform plan` output for all supported providers.
- **Examples**: Major features should have a corresponding example in `examples/`.

## Pull Request Process

1. **Create an issue** for significant changes first.
2. **Fork and branch** from `master`.
3. **Make your changes** following the guidelines above.
4. **Verify tests** pass locally.
5. **Submit PR** with a clear description of WHAT was changed and WHY.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for helping us build better multi-cloud infrastructure! 🙏**
