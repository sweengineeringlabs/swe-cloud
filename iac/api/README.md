# API Layer README
# Resource Contracts and Interface Definitions

## Overview

The `api/` layer defines **standardized resource contracts** that specify the inputs and outputs for all infrastructure resources. This layer ensures consistency across providers and enables the Core and Facade layers to work with a unified interface.

## Layer Position

```
Layer 1: COMMON
    ↓
Layer 2: API ← You are here
    ↓
Layer 3: FACADE
    ↓
Layer 4: [PROVIDER]/core
    ↓
Layer 5: [PROVIDER]/spi
```

## Purpose

The API layer serves as a **contract** between:
1. **User Interface (Facade)** - What users can configure
2. **Implementation (Providers)** - What providers must deliver

This separation allows:
- **Provider independence** - Swap providers without changing contracts
- **Validation** - Type-safe inputs with Terraform validation
- **Documentation** - Self-documenting interfaces
- **Testing** - Contract tests ensure implementations comply

## Resource Types

### Compute (`api/compute/`)
Virtual machine instances across clouds.

**Input Contract** (`variables.tf`):
- `instance_name` - Instance identifier
- `instance_size` - Normalized size (small/medium/large/xlarge)
- `ssh_public_key` - SSH access key
- `allow_public_access` - Public network access
- Provider-specific configuration

**Output Contract** (`outputs.tf`):
- `instance_id` - Unique instance identifier
- `instance_type` - Provider-specific type used
- `public_ip` / `private_ip` - Network addresses
- `ssh_connection` - Connection string
- Metadata and cost information

### Storage (`api/storage/`)
Object storage buckets across clouds.

**Input Contract** (`variables.tf`):
- `bucket_name` - Bucket identifier
- `storage_class` - Normalized class (standard/infrequent/archive/cold)
- `versioning_enabled` - Object versioning
- `encryption_enabled` - At-rest encryption
- `public_access_block` - Public access control
- Lifecycle rules and CORS

**Output Contract** (`outputs.tf`):
- `bucket_id` - Unique bucket identifier
- `bucket_url` - Access URL
- `bucket_region` - Location information
- Configuration status
- Metadata and cost information

### Database (`api/database/`)
*Coming in future phase*

## Design Principles

### 1. Provider Agnostic
Contracts define **what** not **how**:
```hcl
# ✅ Good - Generic
variable "instance_size" {
  type = string
  # Translates to: t3.medium (AWS), Standard_B2s (Azure), e2-medium (GCP)
}

# ❌ Bad - Provider-specific
variable "instance_type" {
  type = string
  # Forces users to know provider-specific types
}
```

### 2. Comprehensive Validation
Every input has validation rules:
```hcl
variable "bucket_name" {
  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]*[a-z0-9]$", var.bucket_name))
    error_message = "Bucket name must be DNS-compliant"
  }
  validation {
    condition     = length(var.bucket_name) >= 3 && length(var.bucket_name) <= 63
    error_message = "Bucket name must be 3-63 characters"
  }
}
```

### 3. Standardized Outputs
All outputs follow consistent naming:
```hcl
output "resource_id"     # Unique identifier
output "resource_arn"    # Cloud API reference
output "resource_url"    # Access endpoint
output "metadata"        # Structured metadata
output "tags"            # Applied tags
```

### 4. Sensible Defaults
Optional parameters have safe defaults:
```hcl
variable "encryption_enabled" {
  default = true  # Secure by default
}

variable "public_access_block" {
  default = true  # Private by default
}
```

## Usage from Core Layer

```hcl
# core/compute/main.tf
module "compute_api" {
  source = "../../api/compute"
  
  # Inputs conform to API contract
  instance_name        = var.instance_name
  instance_size        = var.instance_size
  provider_name = var.provider
  ssh_public_key       = var.ssh_key
  allow_public_access  = var.allow_public_access
}

# Outputs conform to API contract
output "instance" {
  value = {
    id         = module.compute_api.instance_id
    type       = module.compute_api.instance_type
    public_ip  = module.compute_api.public_ip
  }
}
```

## Contract Evolution

When adding new features:

1. **Add to API contract** (this layer)
2. **Implement in providers** (provider layer)
3. **Expose in Core** (orchestration layer)
4. **Document in Facade** (user interface)

### Example: Adding Disk Size

```hcl
# 1. API Layer - Add to contract
variable "disk_size_gb" {
  description = "Root disk size in GB"
  type        = number
  default     = 20
  validation {
    condition     = var.disk_size_gb >= 10 && var.disk_size_gb <= 2000
    error_message = "Disk size must be 10-2000 GB"
  }
}

# 2. Providers - Implement
# providers/aws/compute/main.tf
resource "aws_instance" "this" {
  root_block_device {
    volume_size = var.disk_size_gb
  }
}

# 3. Core - Pass through
module "provider_instance" {
  disk_size_gb = var.disk_size_gb
}

# 4. Facade - Document
# facade/compute/README.md
- `disk_size_gb`: Root disk size (default: 20GB, range: 10-2000GB)
```

## Validation Examples

### Instance Name
```hcl
# Valid
instance_name = "web-server-01"
instance_name = "app-tier-prod"
instance_name = "db-primary"

# Invalid
instance_name = "Web_Server"     # Uppercase
instance_name = "-api-server"    # Starts with hyphen
instance_name = "db.primary"     # Contains period
instance_name = "x"              # Too short (< 3 chars)
```

### Storage Class
```hcl
# Valid
storage_class = "standard"    # Hot/frequent access
storage_class = "infrequent"  # Warm/IA
storage_class = "archive"     # Cold/Glacier
storage_class = "cold"        # Deep Archive

# Maps to:
AWS:    STANDARD, STANDARD_IA, GLACIER, DEEP_ARCHIVE
Azure:  Hot, Cool, Archive, Archive
GCP:    STANDARD, NEARLINE, COLDLINE, ARCHIVE
```

## Testing Contracts

Contract tests verify implementations:

```hcl
# Test: Compute contract compliance
resource "test_instance" "compute_contract" {
  # Must accept all required inputs
  instance_name = "test-01"
  instance_size = "medium"
  provider_name = "aws"
  
  # Must produce all required outputs
  lifecycle {
    postcondition {
      condition     = self.instance_id != null
      error_message = "Contract violation: instance_id is required"
    }
    postcondition {
      condition     = can(regex("^\\d+\\.\\d+\\.\\d+\\.\\d+$", self.private_ip))
      error_message = "Contract violation: private_ip must be valid IPv4"
    }
  }
}
```

## Related Documentation

- [Common Layer](../common/README.md) - Shared definitions
- [Core Layer](../core/README.md) - Orchestration (coming)
- [Facade Layer](../facade/README.md) - Public interface (coming)
- [Implementation Plan](../IMPLEMENTATION_PLAN.md) - Full architecture

---

**Status:** Phase 3 Complete ✅  
**Next:** Phase 4 - Core Layer (Resource Orchestration)
