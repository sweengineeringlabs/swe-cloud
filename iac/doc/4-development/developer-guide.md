# Multi-Cloud IAC Developer Guide

**Audience**: Internal and community developers contributing to the Multi-Cloud IAC framework.

## WHAT: Development Workflow & Standards

This guide outlines the standard procedures for developing, testing, and contributing new modules or provider support to the framework. It ensures that all contributions maintain the high standards required for multi-cloud production infrastructure.

**Scope**:
- Local environment setup.
- Layer-by-layer development workflow.
- Naming and coding standards.
- Testing requirements.

## WHY: Consistency at Scale

### Problems Addressed

1. **Architectural Drift**
   - Impact: New modules being created without adhering to the 5-layer SEA pattern.
   - Consequence: Inconsistent API contracts and hard-to-maintain code.

2. **Quality Variance**
   - Impact: Modules merged without proper static or unit tests.
   - Consequence: Regressions and production deployment failures.

### Benefits
- **Predictable Development**: Clear steps for adding new features.
- **High Quality**: Standardized tests ensure logic correctness.
- **Easy Onboarding**: New developers can quickly understand where logic belongs.

## HOW: Development & Contribution Flow

### 1. Environment Setup

Standard tools required:
- **Terraform** (1.0.0+)
- **Go** (1.19+)
- **Cloud CLIs** (AWS, Azure, GCP)

See [Prerequisites Guide](../6-deployment/prerequisites.md) for detailed setup.

### 2. Adding a New Service (Steps)

**Step 1: Define the API (`api/`)**
Create `variables.tf` and `outputs.tf` for the new service in the `api/` directory. Ensure they are provider-agnostic.

**Step 2: Implement Core Logic (`[provider]/core/`)**
Add implementations for each supported cloud provider in the respective `[provider]/core/` directories.

**Step 3: Create the Facade (`facade/`)**
Create the public entry point that uses `count` or `for_each` and the `provider` variable to route to the correct core modules in each provider directory.

**Step 4: Add Documentation**
Every facade must have:
- `doc/overview.md` (WHAT-WHY-HOW)
- `doc/3-design/toolchain.md` (Version matrix)

**Step 5: Write Terratests**
Create a `[service]_test.go` file in the facade directory. At minimum, it must assert:
- Resource creation in `terraform plan`.
- Correct mapping of standardized inputs.
- Failure on invalid inputs (Negative Testing).

### 3. Standards & Conventions

- **Naming**: Use `snake_case` for Terraform resources and variables.
- **Tags**: Always include the `common_tags` local variable in resource definitions.
- **Validation**: Every public variable MUST have a `validation` block with a clear `error_message`.

### 4. Local Testing Workflow

**Before deploying to cloud**:

1. **Start CloudEmu**:
   ```bash
   cd ../cloudemu
   cargo run --release -p cloudemu-server
   ```

2. **Test Your Module Locally**:
   ```bash
   cd iac/examples/local-cloudemu
   # Modify main.tf to use your new module
   terraform init
   terraform apply -auto-approve
   ```

3. **Run Integration Tests**:
   ```bash
   cd test/integration
   go test -v -timeout 10m ./...
   ```

4. **Verify Resources**:
   ```bash
   ../scripts/verify-cloudemu.sh
   ```

**Benefits**:
- **Fast iteration**: <1 minute cycles vs 5-10 minutes
- **Zero cost**: Unlimited testing without AWS charges
- **Offline development**: No internet required

**See**: [CloudEmu Integration Guide](cloudemu-integration.md) for complete documentation

---

## Summary

Contributing to the IAC framework requires a strict adherence to the SEA architectural layers. By following this guide, you ensure that the multi-cloud abstractions remain robust and reliable.

**Key Takeaways**:
1. Check the [Architecture Hub](../3-design/architecture.md) before starting.
2. Maintain 100% test coverage for the planning phase.
3. Don't skip the `iac_api` contract definition.
4. **Test locally with CloudEmu before pushing to cloud**.

**Related Documentation**:
- [Architecture Hub](../3-design/architecture.md)
- [Testing Strategy](../5-testing/testing-strategy.md)
- [CloudEmu Integration](cloudemu-integration.md)
- [Contributing Guidelines](../../CONTRIBUTING.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0  
