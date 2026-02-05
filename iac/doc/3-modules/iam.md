# IAM Module (Facade)

The IAM module provides a unified interface for managing Identity and Access Management across AWS, Azure, GCP, and ZeroCloud.

## Usage

```hcl
module "identity" {
  source        = "../../facade/iam"
  provider_name = "aws" # or "zero", "azure", "gcp"
  
  identity_type = "role"
  identity_name = "app-role"
  
  # Capability-Based Policies
  roles = ["storage_read", "nosql_write"]
}
```

## Inputs

| Name | Description | Type | Default |
| :--- | :--- | :--- | :--- |
| `provider_name` | Target provider | `string` | - |
| `identity_name` | Name of Role/User/SA | `string` | - |
| `identity_type` | `role`, `user`, or `service_agent` | `string` | `service_agent` |
| `roles` | List of capabilities to attach | `list(string)` | `[]` |

## Capabilities (`roles`)

This module supports a unified **Capability-Based Policy** system. You provide abstract capability names, and the module maps them to the correct provider-specific policies.

| Capability | AWS / ZeroCloud (Policy) | Azure (Role Definition) | GCP (Role) |
| :--- | :--- | :--- | :--- |
| `storage_read` | `AmazonS3ReadOnlyAccess` | `Storage Blob Data Reader` | `roles/storage.objectViewer` |
| `storage_write` | `AmazonS3FullAccess` | `Storage Blob Data Contributor` | `roles/storage.objectAdmin` |
| `nosql_read` | `AmazonDynamoDBReadOnlyAccess` | `Cosmos DB Account Reader` | `roles/datastore.viewer` |
| `nosql_write` | `AmazonDynamoDBFullAccess` | `Cosmos DB Account Contributor` | `roles/datastore.user` |
| `compute_admin` | `AmazonEC2FullAccess` | `Virtual Machine Contributor` | `roles/compute.admin` |
| `admin` | `AdministratorAccess` | `Owner` | `roles/owner` |

## Provider Specifics

- **AWS / ZeroCloud**: Creates IAM Roles or Users. Attaches Managed Policies.
- **Azure**: Creates User Assigned Managed Identities. (Policy attachment pending implementation).
- **GCP**: Creates Service Accounts. (Policy attachment pending implementation).
