# Multi-Cloud Example

Demonstrates a true multi-cloud deployment controlled by a single Terraform configuration using Facades.

## Topology

1.  **AWS**: Hosts the API layer (EC2 + VPC).
2.  **Azure**: Hosts corporate data (Azure SQL + VNet).
3.  **GCP**: Hosts analytics workload (Cloud Storage + Compute Engine).

## Usage

```bash
terraform init
terraform apply
```

Note: You must have credentials configured for **all three providers** in your environment for this to work.
