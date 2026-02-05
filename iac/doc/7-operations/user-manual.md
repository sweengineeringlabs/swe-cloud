# Multi-Cloud IAC User Manual

This manual provides a comprehensive guide for **consuming** the IAC framework to build infrastructure.

## 1. Core Concepts

The framework abstracts cloud-specific details into three main concepts:

1.  **Provider Name**: A string (`"aws"`, `"azure"`, `"gcp"`, `"zero"`) that completely switches the backend implementation.
2.  **Normalized Siting**: T-shirt sizes (`"small"`, `"medium"`) that map to optimal instance types/capacities per cloud.
3.  **Roles (Policies)**: High-level capabilities (`"storage_read"`) rather than complex specific JSON/IAM rules.

## 2. Supported Modules

| Name | Source Path | Supported Providers |
| :--- | :--- | :--- |
| **Compute** | `facade/compute` | AWS, Azure, GCP, Zero |
| **Storage** | `facade/storage` | AWS, Azure, GCP, Zero |
| **Networking** | `facade/networking` | AWS, Azure, GCP, Zero |
| **Identity** | `facade/iam` | AWS, Azure, GCP, Zero |
| **Messaging** | `facade/messaging` | AWS, Azure, GCP, Zero |
| **Lambda** | `facade/lambda` | AWS, Azure, GCP, Zero |
| **NoSQL** | `facade/nosql` | AWS, Azure, GCP, Zero |

## 3. Basic Usage Patterns

### Creating a Secure Microservice

This example creates a compute instance with a bound IAM identity.

```hcl
variable "target_cloud" { default = "zero" }

module "app_identity" {
  source        = "./facade/iam"
  provider_name = var.target_cloud
  identity_name = "myapp-role"
  roles         = ["storage_read", "nosql_write"]
}

module "app_server" {
  source        = "./facade/compute"
  provider_name = var.target_cloud
  instance_name = "myapp-server"
  instance_size = "medium"
}
```

### Switching Clouds

To deploy the EXACT same architecture to AWS, simply change the variable:
```bash
terraform apply -var="target_cloud=aws"
```

## 4. Configuring ZeroCloud (Local Dev)

To test locally with ZeroCloud, you must configure the AWS provider shim in your root module:

```hcl
provider "aws" {
  alias  = "zero"
  region = "local"
  endpoints {
    ec2 = "http://localhost:8080"
    s3  = "http://localhost:8080"
    # ... (see ZeroCloud Provider Guide)
  }
}
```

## 5. Troubleshooting

- **`Provider not found`**: Ensure you have defined the provider block (especially for ZeroCloud).
- **`Invalid Role`**: Check `facade/iam/variables.tf` for the list of supported role capabilities.
- **`CloudEmu Connection Refused`**: Ensure `cargo run -p cloudemu-server` is running.
