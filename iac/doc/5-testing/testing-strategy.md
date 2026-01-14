# IAC Testing Strategy

**Audience**: Quality Assurance (QA) Engineers, Developers, and DevOps Architects.

## WHAT: Multi-Layered Infrastructure Validation

This document outlines the systematic approach used to verify the correctness, performance, and security of the Multi-Cloud IAC platform. It utilizes a combination of static analysis, unit planning tests, and integration application tests.

**Scope**:
- Static validation of Terraform syntax.
- Unit testing of architectural routing and logic.
- Integration testing for end-to-end functionality.
- CI/CD pipeline integration.

## WHY: Ensuring Reliable Infrastructure

### Problems Addressed

1. **Syntax & Reference Errors**
   - Impact: Malformed Terraform code or missing variables.
   - Consequence: Deployment failures during the execution phase.

2. **Routing Regression**
   - Impact: Logic errors in the facade layer sending AWS configurations to GCP.
   - Consequence: Invalid resource creation and cloud-native errors.

3. **Input Validation Gaps**
   - Impact: Allowing weak passwords or invalid CIDR blocks to bypass the API layer.
   - Consequence: Security vulnerabilities and network conflicts.

### Benefits
- **Shift-Left Quality**: Catching 100% of syntax errors before a Pull Request is even opened.
- **Provider Accuracy**: Programmatically verifying that normalized sizes and regions map correctly.
- **Confidence**: Ensuring that complex multi-cloud compositions work as intended.

## HOW: The Testing Hierarchy

### Testing Layers Overview

We employ a three-tier testing approach:

### Static Validation (Validation)

Static analysis ensures that the code structure, syntax, and internal references are valid. We use the Go test suite to perform this analysis across the entire repository.

**How it works:**
1.  **Dynamic Discovery**: The `validation_test.go` file recursively scans the project for any folder containing `.tf` files.
2.  **Initialization**: It runs `terraform init -backend=false` for each module via Terratest.
3.  **Validation**: It executes `terraform validate` to check for syntax errors.

**Usage:**
```bash
go test -v ./validation_test.go
```

## CI/CD Pipeline Integration

Detailed below is the recommended pipeline workflow:

1.  **Commit**: Run `go test -v ./validation_test.go`. Fail if any module is invalid.
2.  **Pull Request**: Run `terraform plan` on changed modules. Post the plan to the PR.
3.  **Merge to Main**: Run `terraform apply` in a staging environment.

## 4. Test Coverage Report

We measure coverage in terms of **Functional Coverage** across our service catalog and cloud providers.

### Provider-Service Matrix

The following matrix shows the current state of Terratest coverage (Unit Testing layer) across providers.

| Service Facade | AWS Coverage | Azure Coverage | GCP Coverage | Status |
| :--- | :---: | :---: | :---: | :--- |
| **Compute** | ✅ | ✅ | ✅ | **Full Coverage** |
| **Storage** | ✅ | ✅ | ✅ | **Full Coverage** |
| **Database** | ✅ | ✅ | ✅ | **Full Coverage** |
| **Networking** | ✅ | ✅ | ✅ | **Full Coverage** |
| **IAM** | ✅ | ✅ | ✅ | **Full Coverage** |
| **Monitoring** | ✅ | ✅ | ✅ | **Full Coverage** |

### Service Specific Coverage

For services that are provider-specific or have partial core module support:

| Service Facade | AWS Coverage | Azure Coverage | GCP Coverage | Notes |
| :--- | :---: | :---: | :---: | :--- |
| **Lambda** | ✅ | 🟡 | 🟡 | Tested for AWS; Azure/GCP core modules pending. |
| **Messaging** | ✅ | 🟡 | 🟡 | Tested for AWS (SQS/SNS); Azure/GCP core modules pending. |

### Recommendations for Increasing Coverage

To further harden the infrastructure code, the following improvements are recommended:

1.  **Attribute Assertions**: Expand Terratests to verify specific resource attributes (e.g., verifying that a bucket name matches the input variable after plan normalization).
2.  **Negative Variable Testing**: Implement tests that pass invalid CIDR ranges or instance sizes to ensure `validation` blocks trigger as expected.
3.  **Cross-Region Matrix**: Parameterize tests to run across multiple regions (e.g., `us-east-1` vs `eu-west-1`) to verify regional resource mappings.
4.  **Backend State Tests**: Add tests that verify SPI backend configurations (S3/GCS/Blob) to ensure remote state locks work correctly across providers.

## Summary

The Multi-Cloud IAC framework integrates testing directly into the development lifecycle. By utilizing a Go-based toolchain, we ensure that every infrastructure change is validated for consistency and security before it reaches production.

**Key Takeaways**:
1. **Validation First**: Always run `go test -v ./validation_test.go` locally.
2. **Contract Verification**: Use Terratest to verify the `terraform plan` against API requirements.
3. **Multi-Provider Check**: Ensure your tests cover AWS, Azure, and GCP where supported.

---

**Related Documentation**:
- [Architecture Hub](../3-design/architecture.md)
- [Toolchain Specification](../3-design/toolchain.md)
- [Developer Guide](../4-development/developer-guide.md)

**Last Updated**: 2026-01-14  
**Version**: 1.0  
