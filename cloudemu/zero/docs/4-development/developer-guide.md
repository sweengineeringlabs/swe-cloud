# ZeroCloud Developer Guide

**Audience**: Developers, Contributors, DevOps

## WHAT
This guide explains how to set up, develop, and test ZeroCloud components.

## WHY
ZeroCloud is a complex multi-crate project with platform-specific drivers. Consistency in development and testing is critical for stability across Windows and Linux.

## HOW

### Environment Setup
1.  Install Rust (Stable).
2.  (Required for tests) Install Docker.
3.  (Required for native mode) Run as Administrator (Windows) or root (Linux).

### Development Guides
-   **[Installation Guide](guide/installation.md)**: Building and installing the CLI.
-   **[User Manual](guide/user-manual.md)**: How to use the system.

### Testing Strategy
ZeroCloud uses a three-tier testing strategy:
1.  **Unit Tests**: In-crate tests with mocks.
2.  **Integration Tests**: Cross-crate tests ensuring SPI compliance.
3.  **API Integration Tests**: Testing the HTTP Facade via `reqwest`.

### Coding Standards
-   Follow the **Singularity Naming Convention**: Modules should use singular names (e.g., `driver` instead of `drivers`).
-   Use `async-trait` for all SPI implementations.
