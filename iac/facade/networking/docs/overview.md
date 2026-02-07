# Networking Facade Module

## WHAT: Virtual Private Network Provisioning

The Networking facade provides a unified interface for AWS VPC, Azure VNet, and GCP Virtual Private Cloud. It handles network address spacing (CIDR) and subnetwork layout.

**Prerequisites**:
- Terraform `1.0.0+`
- Configured Cloud CLI for the target provider.

## WHY: Standardized Secure Connectivity

### Problems Solved
- **CIDR Management**: Ensuring consistent address space allocation regardless of the cloud.
- **Service Mapping**: Abstracting different networking concepts like AWS Availability Zones vs Azure regions.

## HOW: Usage Example

```hcl
module "base_network" {
  source       = "../../facade/networking"
  provider_name = "azure"
  network_name = "corp-vnet"
  metrics = {
    cidr    = "10.0.0.0/16"
    azs     = ["eastus-1", "eastus-2"]
    subnets = ["10.0.1.0/24", "10.0.2.0/24"]
  }
}
```

## Examples and Tests
- **Unit Tests**: See `facade/networking/networking_test.go` for Terratest plan assertions.

---

**Last Updated**: 2026-01-14
