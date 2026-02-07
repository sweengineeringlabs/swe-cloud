# Common Layer README
# SEA Architecture - Shared Definitions Foundation

## Overview

The `common/` layer is the foundation of the IAC SEA architecture, providing shared definitions, normalization mappings, and validation rules used across all other layers.

## Layer Position

```
Layer 1: COMMON ← You are here
    ↓
Layer 2: SPI
    ↓
Layer 3: API
    ↓
Layer 4: CORE
    ↓
Layer 5: FACADE
```

## Files

### `variables.tf`
Defines standard variable schemas with validation rules:
- Provider selection (`aws`, `azure`, `gcp`, `oracle`)
- Environment configuration (`dev`, `staging`, `prod`)
- Resource sizing (`small`, `medium`, `large`, `xlarge`)
- Project metadata and tagging
- Security and feature flags

### `locals.tf`
Provides normalization mappings:
- **Compute:** Instance type translations across providers
- **Database:** Database instance type mappings
- **Storage:** Size and class mappings
- **Network:** CIDR block allocations
- **Environment:** Environment-specific settings

### `tags.tf`
Standardizes resource tagging:
- Standard tag schema (ManagedBy, Environment, Provider, etc.)
- Provider-specific tag formatting (AWS, Azure, GCP, Oracle)
- Resource-type-specific tags
- Cost allocation tags

## Usage

### From Other Modules

```hcl
# In a facade/core/api module
module "common" {
  source = "../../common"
}

# Access size mappings
locals {
  instance_type = module.common.compute_instance_types[var.provider][var.instance_size]
}

# Apply standard tags
resource "aws_instance" "example" {
  tags = module.common.compute_tags
}
```

### Size Normalization Example

```hcl
# User specifies: instance_size = "medium"

# Common layer translates:
aws:   t3.medium
azure: Standard_B2s
gcp:   e2-medium
oracle: VM.Standard.E4.Flex
```

## Design Principles

1. **Provider Agnostic** - No provider-specific logic
2. **Validation First** - All inputs validated
3. **Single Source of Truth** - Mappings defined once
4. **Extensible** - Easy to add new sizes/providers
5. **Type Safe** - Terraform validation ensures correctness

## Mapping Tables

### Compute Instance Types

| Size   | AWS         | Azure           | GCP            |
|--------|-------------|-----------------|----------------|
| small  | t3.micro    | Standard_B1s    | e2-micro       |
| medium | t3.medium   | Standard_B2s    | e2-medium      |
| large  | m5.large    | Standard_DS2_v2 | n2-standard-2  |
| xlarge | m5.xlarge   | Standard_DS3_v2 | n2-standard-4  |

### Storage Sizes

| Size   | Capacity |
|--------|----------|
| small  | 20 GB    |
| medium | 100 GB   |
| large  | 500 GB   |
| xlarge | 1000 GB  |

### Network CIDRs

| Size   | CIDR        | Addresses |
|--------|-------------|-----------|
| small  | /24         | 256       |
| medium | /20         | 4,096     |
| large  | /16         | 65,536    |
| xlarge | /12         | 1,048,576 |

## Adding New Resources

To add a new resource type:

1. **Add size mapping** in `locals.tf`:
   ```hcl
   new_resource_types = {
     aws   = { small = "...", medium = "..." }
     azure = { small = "...", medium = "..." }
     gcp   = { small = "...", medium = "..." }
   }
   ```

2. **Add resource tags** in `tags.tf`:
   ```hcl
   new_resource_tags = merge(
     local.common_tags,
     { ResourceType = "NewResource" }
   )
   ```

3. **Document** usage in this README

## Validation Rules

All common variables include validation:

```hcl
variable "provider" {
  validation {
    condition     = contains(["aws", "azure", "gcp", "oracle"], var.provider)
    error_message = "..."
  }
}
```

This ensures **type safety** and **early error detection**.

## Related Documentation

- [IAC SEA Architecture](../ARCHITECTURE.md)
- [Implementation Plan](../IMPLEMENTATION_PLAN.md)
- [CloudKit Comparison](../CLOUDKIT_IAC_COMPARISON.md)
