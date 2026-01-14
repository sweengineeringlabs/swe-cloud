# Unit Testing with Terratest

We use **Terratest** (Go) to perform "Unit Tests" on our Terraform modules. These tests run `terraform plan` to verify logic without incurring cloud costs.

## Prerequisites
- Go >= 1.20
- Terraform >= 1.0

## Directory Structure
Tests are located in `iac/test/`.
```
iac/test/
├── go.mod                # Go dependencies
├── go.sum                # Checksums
└── storage_facade_test.go # Test logic
```

## Running Tests

To run all unit tests:
```bash
cd iac/test
go test -v
```

This will:
1.  Initialize Terraform for the targeted modules.
2.  Run `terraform plan` with specific variables.
3.  Assert that the Plan output contains expected resources (e.g., "1 to add").

## Writing New Tests
Create a new `*_test.go` file. Use the pattern:

```go
func TestNewModule(t *testing.T) {
    opts := &terraform.Options{
        TerraformDir: "../path/to/module",
        Vars: map[string]interface{}{...},
    }
    plan := terraform.InitAndPlan(t, opts)
    assert.Contains(t, plan, "resource_type.name")
}
```
