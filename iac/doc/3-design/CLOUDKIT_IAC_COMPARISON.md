# CloudKit SDK ↔️ IAC SEA Parallel Patterns

This document provides a side-by-side comparison of the design patterns used in the CloudKit Rust SDK and the IAC Infrastructure framework.

## 1. Architectural Mapping

| SEA Layer | CloudKit (Rust) | IAC (Terraform) | Purpose |
|:---|:---|:---|:---|
| **Facade** | `cloudkit` crate | `facade/` modules | Public entry point; unified interface |
| **Core** | `cloudkit_core` | `iac_core/` modules | Orchestration and dependency management |
| **API** | `cloudkit_api` traits | `iac_api/` schemas | Standardized resource contracts |
| **SPI** | `cloudkit_spi` | `iac_spi/` configs | Provider-specific integration points |
| **Common** | `cloudkit_spi` types | `common/` definitions | Shared types and constants |

## 2. Resource Abstraction Patterns

### CloudKit (Rust)
Resources are defined as traits in `cloudkit_api`. Providers implement these traits.

```rust
pub trait StorageProvider: Send + Sync {
    fn create_bucket(&self, name: &str) -> Result<Bucket>;
}
```

### IAC (Terraform)
Resources are defined as variable/output contracts in `iac_api`. Core modules route to provider implementations that satisfy these contracts.

```hcl
# iac_api/storage/variables.tf
variable "bucket_name" { type = string }
variable "storage_class" { type = string }

# iac_api/storage/outputs.tf
output "bucket_url" { value = local.bucket_url }
```

## 3. Provider Switching

### CloudKit (Rust)
Switching providers is done at initialization using the `Cloud::new()` builder.

```rust
let cloud = Cloud::new(Provider::Aws)
    .with_region("us-east-1")
    .build();
```

### IAC (Terraform)
Switching providers is done by changing a single variable in the Facade module.

```hcl
module "storage" {
  source   = "../../facade/storage"
  provider_name = "aws" # Change to "azure" or "gcp"
  # ...
}
```

## 4. Size Normalization

Both systems use a normalization layer to map generic sizes (small, medium, large) to provider-specific SKU names.

| Generic | AWS | Azure | GCP |
|:---|:---|:---|:---|
| `small` | `t3.micro` | `Standard_B1s` | `e2-micro` |
| `medium` | `t3.medium` | `Standard_B2s` | `e2-medium` |

## 5. Security and Defaults

Both systems implement "Secure by Default" principles:
- **CloudKit**: Implements encrypted-at-rest data by default in SDK methods.
- **IAC**: Facade modules enable S3/Blob encryption and block public access by default.

## 6. Testing Strategy

| Level | CloudKit | IAC |
|:---|:---|:---|
| **Unit** | Mocking traits | `terraform plan` assertions |
| **Integr.** | Real API calls | `terraform apply` (Terratest) |
| **Contract** | Rust type system | HCL variable validation |

## 7. Conclusions

By mirroring the structural patterns of the CloudKit SDK, the IAC project achieves:
1. **Mental Model Parity**: A developer who understands the SDK's architecture immediately understands the infrastructure's architecture.
2. **Standardized Operations**: Consistent tagging, monitoring, and security across all cloud deployments.
3. **Provider Agnosticism**: Reduced vendor lock-in through well-defined abstraction layers.
