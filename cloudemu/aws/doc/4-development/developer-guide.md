# AWS Developer Guide

## WHAT
Guide for contributing to the AWS emulation module.

## WHY
Ensures consistency and quality in service implementation.

## HOW

### Adding a New Service
1. **Define Service**: Create `src/services/<service_name>/mod.rs`.
2. **Implement Handlers**: Create request handlers in `handlers.rs`.
3. **Update Router**: Register the new service in `lib.rs`.
4. **Update Storage**: Add tables in `aws-data-core`.

### Testing
- Run unit tests: `cargo test -p aws-control-core`
- Run integration tests: `cargo test --test integration`

### Tools
- **AWS CLI**: Essential for verification.
- **Rusqlite**: For database interactions.
