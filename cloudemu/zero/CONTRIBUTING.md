# Contributing to ZeroCloud

We welcome contributions! Please follow these guidelines:

1.  **Architecture First**: All major changes must be preceded by an ADR (Architecture Decision Record).
2.  **Layered Design**: Respect the separation between SPI, Core, and Data drivers.
3.  **Tests Required**: No PR will be merged without matching unit and integration tests.
4.  **Documentation**: Update the relevant `overview.md` files for any changed crates.

## Development Workflow

1.  Create a feature branch.
2.  Implement changes.
3.  Run `cargo test --workspace`.
4.  Submit a Pull Request.
