# Contributing To CloudEmu

Welcome! We are excited that you want to contribute to CloudEmu, our production-grade local cloud emulator.

## How to Contribute

### 1. Adding a New AWS Service
CloudEmu follows a modular architecture. To add a new service:
1. **Dispatcher**: Update `gateway/dispatch.rs` to route the service requests.
2. **Service Handler**: Create a new module in `services/` (e.g., `services/sqs/`).
3. **Storage Engine**: Implement the persistence logic in the `data-plane` crate.
4. **Implementation**:
    - Implement the Axum handlers.
    - Generate AWS-compatible XML/JSON responses.
    - Add integration tests.

### 2. Reporting Bugs
- Use the GitHub Issue Tracker.
- Provide a minimal reproduction case (e.g., a Terraform snippet or AWS SDK code).
- Include the emulator logs.

### 3. Pull Requests
- Follow Rust coding standards (`cargo fmt`, `cargo clippy`).
- Ensure all tests pass (`cargo test`).
- Update relevant documentation in the `doc/` directory.

## Development Setup

```bash
# Clone the repository
git clone git@github.com:sweengineeringlabs/swe-cloud.git
cd cloudemu

# Run tests
cargo test

# Start the emulator for manual testing
cargo run
```

---

**Thank you for helping make CloudEmu better! 🚀**
