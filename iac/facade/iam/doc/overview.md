# IAM Facade Module

## WHAT: Unified Identity & Access Management

The IAM facade provides a unified interface for AWS IAM Roles, Azure Managed Identities, and GCP Service Accounts. It handles identity creation and principal attachment.

**Prerequisites**:
- Terraform `1.0.0+`
- Configured Cloud CLI for the target provider.

## WHY: Centralized Security Principals

### Problems Solved
- **Identity Abstraction**: Mapping the concept of a "Service Actor" to provider-specific resources.
- **Permission Normalization**: Providing a consistent entry point for attaching permissions to compute resources.

## HOW: Usage Example

```hcl
module "app_identity" {
  source        = "../../facade/iam"
  provider      = "aws"
  identity_name = "app-server-role"
  identity_type = "role"
  principals    = ["ec2.amazonaws.com"]
}
```

## Examples and Tests
- **Unit Tests**: See `facade/iam/iam_test.go` for Terratest plan assertions.

---

**Last Updated**: 2026-01-14
