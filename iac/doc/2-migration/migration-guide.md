# Migration Guide: IAC SEA Architecture

This guide explains how to migrate from the old monolithic IAC structure to the new **Service-Oriented Architecture (SEA)** pattern.

## Architecture Changes

### Old Structure
- `iac/core/compute`: Mixed logic, hard to scale.
- `iac/core/storage`: Mixed logic.

### New Structure
The new architecture strictly separates **Core Provider Logic** from **Unified Facades**.

#### Core Layer (`iac_core/`)
Direct, provider-specific implementations.
- `iac_core/aws/src/compute` -> EC2
- `iac_core/azure/src/compute` -> VM
- `iac_core/gcp/src/compute` -> Compute Engine

#### Facade Layer (`iac/facade/`)
Unified interfaces that abstract the provider.
- `iac/facade/compute` -> Routes to AWS/Azure/GCP based on `var.provider`.

## Migration Steps

### 1. Update Module Sources
Change your module sources in `main.tf` to point to the new Facade or Core paths.

**Before:**
```hcl
module "app_server" {
  source = "../../core/compute"
  # ...
}
```

**After (Using Facade - Recommended):**
```hcl
module "app_server" {
  source   = "../../facade/compute"
  provider = "aws" # or "azure", "gcp"
  # ...
}
```

**After (Using Core - Advanced):**
```hcl
module "app_server" {
  source = "../../iac_core/aws/src/compute"
  # ...
}
```

### 2. Update Variable Names
Some variable names have been standardized in the Facade layer.

- `ami` -> `image_id` (in some contexts, though Facade handles defaults)
- `instance_type` -> `instance_size` (small/medium/large) for generic scaling.

### 3. State Migration
If migrating existing resources, use `terraform state mv`:

```bash
terraform state mv module.old_module module.new_module
```

## Available Modules

| Resource | Facade Path | AWS Core | Azure Core | GCP Core |
|:---|:---|:---|:---|:---|
| **Compute** | `facade/compute` | `aws/src/compute` | `azure/src/compute` | `gcp/src/compute` |
| **Storage** | `facade/storage` | `aws/src/storage` | `azure/src/storage` | `gcp/src/storage` |
| **Database** | `facade/database` | `aws/src/database` | `azure/src/database` | `gcp/src/database` |
| **Networking**| `facade/networking`| `aws/src/networking`| `azure/src/networking`| `gcp/src/networking`|
| **IAM** | `facade/iam` | `aws/src/iam` | `azure/src/iam` | `gcp/src/iam` |

## Support
For issues, refer to the `iac/doc/4-development/backlog.md` or contact the platform engineering team.
