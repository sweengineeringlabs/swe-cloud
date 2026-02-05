# Compute Facade Module

## WHAT: Unified Virtual Machine Provisioning

The Compute facade provides a single interface for deploying virtual machines across AWS (EC2), Azure (Virtual Machines), and GCP (Compute Engine). It handles OS image mapping and instance size normalization.

**Prerequisites**:
- Terraform `1.0.0+`
- Configured Cloud CLI for the target provider (AWS, Azure, or GCP).
- Initialized SPI layer for backend state management.

## WHY: Consistency Across Clouds

### Problems Solved
- **Inconsistent Sizing**: Mapping generic sizes (`small`, `medium`, `large`) to provider-specific instance types (e.g., `t3.micro` vs `Standard_B1s`).
- **Provider-Specific Logic**: Hiding the complexity of different resource names and configuration blocks between providers.

## HOW: Usage Example

```hcl
module "web_server" {
  source        = "../../facade/compute"
  provider_name = "aws"
  instance_name = "prod-web-01"
  instance_size = "medium"
  project_name  = "ProjectX"
  environment   = "prod"
}
```

## Examples and Tests

- **Basic Example**: See `examples/web-app/` for a production-like compute deployment.
- **Unit Tests**: See `facade/compute/compute_test.go` for Terratest plan assertions.

---

**Last Updated**: 2026-01-14
