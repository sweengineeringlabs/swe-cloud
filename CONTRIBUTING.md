# Contributing to SWE Cloud

## Prerequisites

- Rust 1.85+ (`rustup update stable`)
- Terraform 1.x (for IAC)
- Node.js 18+ (for swe-cloud-ui)

## Development Workflow

1. Fork and clone the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make changes following the [SEA architecture](docs/3-design/architecture.md)
4. Run validation: `cargo check --workspace && cargo test --workspace`
5. Submit a pull request

## SEA Compliance

All Rust crates must follow the SEA (SPI-API-Core-Facade) layering pattern:
- **SPI**: Types, errors, and trait definitions
- **API**: Public trait contracts
- **Core**: Business logic implementations
- **Facade**: Re-exports via SAF module

## PR Checklist

- [ ] Code compiles: `cargo check --workspace`
- [ ] Tests pass: `cargo test --workspace`
- [ ] No clippy warnings: `cargo clippy --workspace`
- [ ] Documentation updated in `docs/`
- [ ] Follows SEA layering conventions

## Reporting Issues

Use GitHub Issues with the appropriate template (Bug Report or Feature Request).
