# Unit Testing with Terratest

We use **Terratest** (Go) to perform "Unit Tests" on our Terraform modules. These tests run `terraform plan` to verify logic without incurring cloud costs.

## Prerequisites
- Go >= 1.20
- Terraform >= 1.0

## Directory Structure
Tests are co-located with the modules they verify. A root `go.mod` in `iac/` manages dependencies.

```
iac/
├── go.mod                # Root Go dependencies
├── facade/
│   └── storage/
│       ├── main.tf
│       └── storage_test.go # Co-located test
```

## Running Tests

To run all unit tests from the root:
```bash
cd iac
go test -v ./...
```

This will recursively find and execute all `*_test.go` files in the repository.

## Writing New Tests
Create a new `*_test.go` file next to your module. Use `TerraformDir: "."`.

```go
package mymodule_test

import (
    "testing"
    "github.com/gruntwork-io/terratest/modules/terraform"
    "github.com/stretchr/testify/assert"
)

func TestMyModule(t *testing.T) {
    opts := &terraform.Options{
        TerraformDir: ".",
        Vars: map[string]interface{}{...},
    }
    plan := terraform.InitAndPlan(t, opts)
    assert.Contains(t, plan, "resource_type.name")
}
```
