# Developer Guide

**Audience**: Contributors, Developers

Hub document for CloudEmu development guides and practices.

## WHAT: Development Documentation

This guide provides comprehensive information for developers working on CloudEmu, including architecture understanding, contribution workflows, and best practices.

## WHY: Streamlined Development

### Problems Addressed

1. **Onboarding Complexity**: New contributors need clear entry points
2. **Consistency**: Development practices should be standardized
3. **Quality**: Testing and code review standards must be maintained

### Benefits

- **Fast Onboarding**: Clear guides reduce time-to-first-contribution
- **High Quality**: Standardized practices improve code quality
- **Maintainability**: Consistent patterns make the codebase easier to maintain

## HOW: Development Resources

### Quick Links

| Guide | Description |
|-------|-------------|
| [Architecture Overview](../3-design/architecture.md) | System design and component interaction |
| [Testing Strategy](../5-testing/testing-strategy.md) | Testing pyramid and practices |
| [Contributing](../../CONTRIBUTING.md) | Contribution workflow and guidelines |

### Development Workflow

1. **Setup Environment**
   ```bash
   # Clone repository
   git clone https://github.com/sweengineeringlabs/swe-cloud.git
   cd cloud/cloudemu
   
   # Build all crates
   cargo build --workspace
   
   # Run tests
   cargo test --workspace
   ```

2. **Run Local Server**
   ```bash
   # Start multi-cloud server
   cargo run -p cloudemu-server
   
   # Server listens on:
   # - AWS: 4566
   # - Azure: 4567
   # - GCP: 4568
   ```

3. **Development Cycle**
   - Create feature branch
   - Write tests first (TDD)
   - Implement feature
   - Run `cargo test`
   - Submit PR with tests

### Code Organization

```
cloudemu/
├── crates/
│   ├── cloudemu-core/       # Shared types and traits
│   ├── cloudemu-server/     # Multi-cloud orchestrator
│   ├── cloudemu-azure/      # Azure provider
│   ├── cloudemu-gcp/        # GCP provider
│   ├── control-plane/       # AWS control plane (gateway)
│   └── data-plane/          # Storage engine
```

### Testing Guidelines

- **Unit Tests**: Test individual functions and modules
- **Integration Tests**: Test crate public APIs
- **End-to-End Tests**: Test full request flows

See [Testing Strategy](../5-testing/testing-strategy.md) for details.

### Adding a New Provider

1. Create new crate: `cloudemu-[provider]`
2. Implement `CloudProviderTrait` from `cloudemu-core`
3. Add service handlers
4. Wire into `cloudemu-server/src/main.rs`
5. Add integration tests

### Adding a New Service

1. Create service module in provider crate
2. Implement request routing
3. Add storage operations
4. Write unit tests
5. Add integration tests
6. Update documentation

### Code Style

- Follow Rust standard style (rustfmt)
- Run `cargo clippy` before committing
- Add documentation comments for public APIs
- Keep functions focused and testable

### Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [AWS API Reference](https://docs.aws.amazon.com/index.html)
- [Azure API Reference](https://learn.microsoft.com/en-us/rest/api/azure/)

---

**Last Updated**: 2026-01-14
