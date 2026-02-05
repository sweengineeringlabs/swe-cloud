# Multi-Cloud Data Pipeline Example

This example demonstrates a complete data pipeline infrastructure using the IAC SEA architecture facades.

## Architecture

The pipeline consists of:
1.  **Networking Layer**: VPC/VNet with public and private subnets.
2.  **Ingestion Layer**: Object storage (S3/Blob/GCS) for raw data ingestion.
3.  **Processing Layer**: Compute instances (EC2/VM) for data processing.
4.  **Metadata Layer**: Relational database (RDS/SQL/CloudSQL) for storing metadata.

## Usage

### Prerequisites
- Terraform >= 1.0
- Configured credentials for AWS, Azure, or GCP.

### Deploy (AWS)
```bash
terraform init
terraform apply -var="environment=dev"
```

### Deploy (Simulated Multi-Cloud)
Uncomment the Azure or GCP sections in `main.tf` to deploy across multiple providers simultaneously.

## Modules Used
- `../../facade/networking`: Unified network management.
- `../../facade/compute`: Unified compute management.
- `../../facade/storage`: Unified object storage.
- `../../facade/database`: Unified relational database.
