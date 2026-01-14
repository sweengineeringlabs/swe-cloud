# IAC Testing Strategy

This document outlines the testing strategy for the Multi-Cloud Infrastructure as Code (IAC) project.

## Testing Layers

We employ a multi-layered testing approach to ensure the reliability, security, and correctness of our infrastructure code.

| Test Type | Code Equivalent | Terraform Command | Cost Impact | Frequency |
|:---|:---|:---|:---|:---|
| **Static Analysis** | Compiler / Linter | `terraform validate` | None | On every commit (Local/CI) |
| **Unit Test** | Logic Assertion (`assert(2+2==4)`) | `terraform plan` | None | On Pull Requests |
| **Integration Test** | Running the App | `terraform apply` | **Yes** | Nightly / Release Candidate |

## 1. Static Analysis (Validation)

Static analysis ensures that the code structure, syntax, and internal references are valid. It is the cheapest and fastest way to catch errors.

### The `validate_all.ps1` Approach

We utilize a single, dynamic PowerShell script located at `iac/scripts/validate_all.ps1` to perform static analysis across the entire repository.

**How it works:**
1.  **Dynamic Discovery**: The script recursively scans the project directory for any folder containing a `main.tf` file. This means new modules are automatically detected without updating the script.
2.  **Initialization**: It runs `terraform init -backend=false` for each module. This downloads necessary provider plugins but **does not** configure remote state, keeping operation strictly local.
3.  **Validation**: It executes `terraform validate` to check for syntax errors and undefined variables.
4.  **Reporting**: A summary report of Passed vs. Failed modules is generated.

**Usage:**
```powershell
./iac/scripts/validate_all.ps1
```

## 2. Unit Testing (Planning)

Unit testing in Terraform largely revolves around the `terraform plan` command. This simulates the execution of the code against the current state of the cloud provider to verify *what would happen*.

- **Goal**: Verify that the calculated changes match expectations (e.g., "I expect 1 resource to be added").
- **Tools**: `terraform plan`, Open Policy Agent (OPA) for policy checks (optional).

## 3. Integration Testing (Applying)

Integration testing involves actually provisioning resources in a sandbox environment to verify they work as intended.

- **Goal**: Verify end-to-end functionality (e.g., "Can the EC2 instance actually talk to the RDS database?").
- **Tools**: `terratest` (Go), `kitchen-terraform`.
- **Cost**: Real resources are created and billed. Tests should include a teardown step (`terraform destroy`).

## CI/CD Pipeline Integration

Detailed below is the recommended pipeline workflow:

1.  **Commit**: Run `validate_all.ps1`. Fail if any module is invalid.
2.  **Pull Request**: Run `terraform plan` on changed modules. Post the plan to the PR.
3.  **Merge to Main**: Run `terraform apply` in a staging environment.
