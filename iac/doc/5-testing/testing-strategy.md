# IAC Testing Strategy

This document outlines the testing strategy for the Multi-Cloud Infrastructure as Code (IAC) project.

## Testing Layers

We employ a multi-layered testing approach to ensure the reliability, security, and correctness of our infrastructure code.

| Test Type | Code Equivalent | Terraform Command | Cost Impact | Frequency |
|:---|:---|:---|:---|:---|
| **Static Analysis** | Compiler / Linter | `terraform validate` | None | On every commit (Local/CI) |
| **Unit Test** | Logic Assertion (`assert(2+2==4)`) | `terraform plan` | None | On Pull Requests |
| **Integration Test** | Running the App | `terraform apply` | **Yes** | Nightly / Release Candidate |

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
