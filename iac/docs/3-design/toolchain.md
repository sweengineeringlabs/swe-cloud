# IAC Toolchain & Logic Specifications

This document defines the technical toolchain and logic implementation for the Infrastructure as Code (IAC) project, specifically focusing on the integration of **Go** and **Terratest** for multi-cloud validation.

## 1. Toolchain Overview

The IAC project utilizes a unified Go-based toolchain to ensure consistency across AWS, Azure, and GCP deployments.

| Tool | Role | Implementation |
| :--- | :--- | :--- |
| **Go (1.19+)** | Orchestration | Test runner and automation logic. |
| **Terratest** | Framework | Wrapper for Terraform CLI commands and assertions. |
| **Testify** | Assertion | Rich assertion library for plan and attribute verification. |
| **Terraform (1.0+)** | Engine | The underlying infrastructure provisioner. |

## 2. Testing Logic: The Validation Suite

The `validation_test.go` file implements a global static validation logic that replaces legacy scripting.

### Discovery Algorithm
The suite uses a recursive filesystem walk to identify infrastructure modules:
1.  **Search**: Starts at the root `.` or specified `iac` directory.
2.  **Filter**: Looks for any directory containing `.tf` files.
3.  **Exclusion**: Explicitly ignores `.terraform`, `.git`, and environment-specific credential folders.

### Execution Plan
For every discovered module, a parallel Go routine is spawned:
- **`terraform.InitAndValidateE`**: Performs a local init (backend-less) followed by a `terraform validate` check.
- **Isolations**: Uses an empty `BackendConfig` to prevent state locking issues during validation.

## 3. Testing Logic: The Unit Test Suite (Planning)

Unit tests (e.g., `compute_test.go`) focus on the **Facade-to-Core mapping logic**.

### Planning Strategy
We use `terraform.InitAndPlan` to capture the Terraform Plan output as a string. This allows us to verify logic without incurring cloud costs.

### Assertion Layers
1.  **Routing Assertion**: Verifies that the correct module (AWS vs Azure vs GCP) is selected based on the `provider` variable.
    *   *Logic*: `assert.True(t, strings.Contains(planString, "module.aws_compute"))`
2.  **Attribute Assertion**: Verifies that standardized inputs (e.g., `size = "medium"`) map to the correct provider-specific attributes.
    *   *Logic*: `assert.True(t, strings.Contains(planString, "instance_type = \"t3.medium\""))`
3.  **Negative Assertion**: Verifies that invalid inputs are caught by the API layer validation blocks.
    *   *Logic*: `assert.Error(t, err)` when passing invalid resource names or CIDRs.

## 4. Why Go/Terratest?

The transition from shells/scripts to a Go-based toolchain provides several architectural advantages:

*   **Type Safety**: Infrastructure tests benefit from Go's strong typing, preventing common scripting errors.
*   **Concurrency**: Built-in Go concurrency allows validating 30+ modules in seconds.
*   **Consistency**: The same language used for the CloudKit SDK is used for IAC testing, reducing cognitive load for developers.
*   **Extensibility**: Complex post-deployment checks (e.g., HTTP probing, SSH verification) are easily added using standard Go libraries.

## 5. Standard Commands

### Standard Execution
```bash
go test -v ./...
```

### Static Validation Only
```bash
go test -v ./validation_test.go
```

### Service Specific Test
```bash
go test -v ./facade/storage/...
```
