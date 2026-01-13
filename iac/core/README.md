# Core Layer README
# Resource Orchestration and Composition

## Overview

The `core/` layer implements **resource orchestration**, handling composition, dependency management, provider routing, and lifecycle hooks. This layer sits between the API contracts (what users want) and provider implementations (how it's done).

## Layer Position

```
Layer 1: COMMON
    ↓
Layer 2: SPI
    ↓
Layer 3: API
    ↓
Layer 4: CORE ← You are here
    ↓
Layer 5: FACADE
```

## Purpose

The Core layer is responsible for:

1. **Size Normalization** - Translate generic sizes to provider-specific types
2. **Provider Routing** - Route requests to correct provider implementation
3. **Output Aggregation** - Normalize outputs from different providers
4. **Dependency Management** - Ensure resources are created in correct order
5. **Lifecycle Management** - Post-creation validation and hooks
6. **Tag Application** - Apply standard and resource-specific tags

## Resource Types

### Compute (`core/compute/`)

Orchestrates virtual machine instances across providers.

**Key Responsibilities:**
- Maps `instance_size` → provider-specific instance type
- Routes to AWS/Azure/GCP provider module
- Aggregates outputs (instance_id, IPs, etc.)
- Manages network dependencies
- Applies compute-specific tags

**Example Flow:**
```
User: instance_size = "medium"
  ↓
Core: Looks up common.compute_instance_types["aws"]["medium"]
  ↓
Core: Gets "t3.medium"
  ↓
Core: Routes to providers/aws/compute with instance_type = "t3.medium"
  ↓
Provider: Creates AWS EC2 instance
  ↓
Core: Aggregates outputs into standard format
```

### Storage (`core/storage/`)

Orchestrates object storage buckets across providers.

**Key Responsibilities:**
- Maps `storage_class` → provider-specific class
- Routes to AWS S3/Azure Blob/GCP Storage
- Aggregates bucket outputs
- Validates encryption and access controls
- Applies storage-specific tags

**Example Flow:**
```
User: storage_class = "archive"
  ↓
Core: Looks up common.storage_class_mapping["aws"]["archive"]
  ↓
Core: Gets "GLACIER"
  ↓
Core: Routes to providers/aws/storage with storage_class = "GLACIER"
  ↓
Provider: Creates S3 bucket with Glacier storage class
  ↓
Core: Validates encryption and public access settings
```

## Architecture Pattern

### Provider Routing

```hcl
# Route based on provider variable
module "aws_instance" {
  count  = var.provider == "aws" ? 1 : 0
  source = "../../providers/aws/compute"
  # AWS-specific configuration
}

module "azure_instance" {
  count  = var.provider == "azure" ? 1 : 0
  source = "../../providers/azure/compute"
  # Azure-specific configuration
}

module "gcp_instance" {
  count  = var.provider == "gcp" ? 1 : 0
  source = "../../providers/gcp/compute"
  # GCP-specific configuration
}
```

### Output Aggregation

```hcl
locals {
  # Aggregate instance ID from whichever provider was used
  instance_id = try(
    module.aws_instance[0].instance_id,
    module.azure_instance[0].instance_id,
    module.gcp_instance[0].instance_id,
    ""
  )
  
  # Same pattern for all outputs
  public_ip = try(
    module.aws_instance[0].public_ip,
    module.azure_instance[0].public_ip_address,
    module.gcp_instance[0].external_ip,
    null
  )
}
```

### Size Normalization

```hcl
locals {
  # Get provider-specific type from common layer
  instance_type = var.compute_instance_types[var.provider][var.instance_size]
  # Example: compute_instance_types["aws"]["medium"] = "t3.medium"
}

# Pass to provider
module "aws_instance" {
  instance_type = local.instance_type  # "t3.medium"
}
```

## Dependency Management

### Network Dependencies

```hcl
# Ensure network exists before instance
resource "null_resource" "network_dependency" {
  count = var.subnet_id != null ? 1 : 0

  triggers = {
    subnet_id = var.subnet_id
  }

  lifecycle {
    precondition {
      condition     = var.subnet_id != null && var.subnet_id != ""
      error_message = "Valid subnet_id is required"
    }
  }
}
```

### Creation Order

```
1. Network resources (VPC, Subnet)
   ↓
2. Security resources (Security Groups)
   ↓
3. Compute resources (Instances)
   ↓
4. Post-creation hooks (Monitoring setup)
```

## Lifecycle Hooks

### Post-Creation Validation

```hcl
resource "null_resource" "instance_ready" {
  depends_on = [module.aws_instance, module.azure_instance, module.gcp_instance]

  lifecycle {
    postcondition {
      condition     = local.instance_id != ""
      error_message = "Instance creation failed"
    }
  }
  
  provisioner "local-exec" {
    command = "echo 'Instance ${local.instance_id} is ready'"
  }
}
```

### Feature-Specific Hooks

```hcl
# Monitoring setup (if enabled)
resource "null_resource" "monitoring_setup" {
  count = var.enable_monitoring ? 1 : 0
  
  depends_on = [null_resource.instance_ready]
  
  provisioner "local-exec" {
    command = "echo 'Enabling monitoring for ${local.instance_id}'"
  }
}

# Encryption check (if enabled)
resource "null_resource" "encryption_check" {
  count = var.encryption_enabled ? 1 : 0
  
  provisioner "local-exec" {
    command = "echo 'Encryption verified for ${var.bucket_name}'"
  }
}
```

## Tag Application

### Merging Tags

```hcl
locals {
  resource_tags = merge(
    var.common_tags,           # From common layer (ManagedBy, Environment, etc.)
    {
      ResourceType = "Compute"  # Resource-type tags
      Service      = "VirtualMachine"
      InstanceName = var.instance_name
      Size         = var.instance_size
    },
    var.instance_tags          # User-provided tags
  )
}
```

### Tag Priority

1. **Common tags** (lowest priority) - Standard tags from common layer
2. **Resource-type tags** - Core layer adds ResourceType, Service, etc.
3. **User tags** (highest priority) - User can override any tag

## Usage from Facade Layer

```hcl
# facade/compute/main.tf
module "compute_core" {
  source = "../../core/compute"
  
  # API contract inputs
  instance_name   = var.instance_name
  instance_size   = var.instance_size  # Normalized size
  provider        = var.provider
  
  # Common layer inputs
  compute_instance_types = local.compute_instance_types
  common_tags            = local.common_tags
  
  # Pass through other configs
  ssh_public_key     = var.ssh_public_key
  allow_public_access = var.allow_public_access
  provider_config    = var.provider_config
}

# Outputs
output "instance" {
  value = {
    id         = module.compute_core.instance_id
    type       = module.compute_core.instance_type
    public_ip  = module.compute_core.public_ip
  }
}
```

## Error Handling

### Pre-conditions

```hcl
lifecycle {
  precondition {
    condition     = var.subnet_id != null && var.subnet_id != ""
    error_message = "Valid subnet_id is required when specified"
  }
}
```

### Post-conditions

```hcl
lifecycle {
  postcondition {
    condition     = local.instance_id != ""
    error_message = "Instance creation failed - no instance ID returned"
  }
}
```

## Testing Core Layer

### Unit Test Example

```hcl
# Test size normalization
module "test_compute" {
  source = "./core/compute"
  
  instance_name = "test"
  instance_size = "medium"
  provider      = "aws"
  
  compute_instance_types = {
    aws = { medium = "t3.medium" }
  }
}

# Verify output
output "normalized_type" {
  value = module.test_compute.instance_type
  # Should be "t3.medium"
}
```

### Integration Test

```go
// Test provider switching
func TestProviderSwitch(t *testing.T) {
    providers := []string{"aws", "azure", "gcp"}
    
    for _, provider := range providers {
        terraformOptions := &terraform.Options{
            TerraformDir: "../core/compute",
            Vars: map[string]interface{}{
                "provider":      provider,
                "instance_size": "medium",
            },
        }
        
        terraform.InitAndApply(t, terraformOptions)
        instanceID := terraform.Output(t, terraformOptions, "instance_id")
        assert.NotEmpty(t, instanceID)
    }
}
```

## Design Principles

1. **Single Responsibility** - Each module orchestrates one resource type
2. **Provider Abstraction** - No provider-specific logic leaks to API
3. **Fail Fast** - Pre-conditions catch errors before resource creation
4. **Idempotent** - Can run multiple times safely
5. **Observable** - Lifecycle hooks provide visibility

## Related Documentation

- [Common Layer](../common/README.md) - Size mappings
- [API Layer](../api/README.md) - Resource contracts
- [SPI Layer](../spi/README.md) - Provider setup
- [Facade Layer](../facade/README.md) - Public interface (next)

---

**Status:** Phase 4 Complete ✅  
**Next:** Phase 5 - Facade Layer (Public Interface)
