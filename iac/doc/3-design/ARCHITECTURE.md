# Multi-Cloud SEA Architecture

**Audience**: Architects, Developers, DevOps Engineers, and Security Reviewers.

## WHAT: Stratified Encapsulation Architecture (SEA)

This document defines the 5-layer architectural pattern used to provide a unified, provider-agnostic interface for infrastructure provisioning across AWS, Azure, and GCP. It strictly mirrors the patterns found in the CloudKit Rust SDK.

**Scope**:
- Layer definitions and responsibilities.
- Resource orchestration and dependency flow.
- Size and storage class normalization.
- Provider selection logic.

## WHY: Solving Multi-Cloud Complexity

### Problems Addressed

1. **Provider Lock-in**
   - Impact: Code is tightly coupled to specific cloud provider APIs (e.g., direct use of `aws_instance`).
   - Consequence: High migration costs and inability to use best-of-breed services from different clouds.

2. **Logic Inconsistency**
   - Impact: Different teams implementing the same resource (e.g., S3 bucket) with different tagging or security standards.
   - Consequence: Security gaps and increased maintenance overhead.

3. **Testing Fragility**
   - Impact: Hard-to-test monolithic modules.
   - Consequence: Untested infrastructure logic leading to production failures.

### Benefits
- **Unified Interface**: Deploy a "Compute" resource regardless of the underlying cloud.
- **Strict Layering**: Clear separation between API contracts and provider implementations.
- **High Testability**: Isolated layers enable granular Terratests and static validation.

## HOW: The 5-Layer Implementation

### SEA Layer Overview

## SEA Layers

```
┌─────────────────────────────────────────────────────────────────┐
│              FACADE (Multi-Cloud Orchestration)                  │
│  Root modules with provider selection and normalization         │
├─────────────────────────────────────────────────────────────────┤
│              CORE (Resource Orchestration)                       │
│  Resource composition, dependency management, lifecycle          │
├─────────────────────────────────────────────────────────────────┤
│              API (Resource Contracts)                            │
│  Standardized resource interfaces and outputs                   │
├─────────────────────────────────────────────────────────────────┤
│              SPI (Provider Integration)                          │
│  Provider-specific implementations and configurations            │
├─────────────────────────────────────────────────────────────────┤
│              COMMON (Shared Definitions)                         │
│  Variables, locals, validation, tagging standards               │
└─────────────────────────────────────────────────────────────────┘
```

## Proposed Structure

iac/
├── common/             # Layer 1: COMMON (Shared variables)
├── api/                # Layer 2: API (Resource Contracts)
├── facade/             # Layer 3: FACADE (Unified Entry Points)
├── aws/                # Layer 4 & 5: AWS Provider
│   ├── core/           # AWS internal implementations
│   └── spi/            # AWS credentials/auth
├── azure/              # Azure Provider
│   ├── core/
│   └── spi/
├── gcp/                # GCP Provider
│   ├── core/
│   └── spi/
├── zero/               # ZeroCloud Provider
│   ├── core/
│   └── spi/
├── examples/           # Blueprints & Examples
└── test/               # Terratests

## Layer Details

### Layer 1: COMMON

Foundation layer with shared definitions and standards.

**Contents:**
- Variable schemas with validation
- Size normalization mappings (small → provider-specific)
- Tagging conventions
- Naming standards
- Region mappings

**Principles:**
- No provider dependencies
- Pure data definitions
- Reusable across all modules

### Layer 2: SPI (Service Provider Interface)

Provider-specific configurations and extension points.

**Contents:**
- Provider authentication configuration
- Backend configuration (state storage)
- Provider versions and constraints
- Custom provider settings

**Principles:**
- Isolated provider setup
- Pluggable authentication
- Environment-specific configuration

### Layer 3: API (Resource Contracts)

Standardized resource interfaces defining inputs and outputs.

**Service Contracts:**
- `compute` - Virtual machines, containers, serverless
- `storage` - Object storage, block storage, file storage
- `database` - SQL, NoSQL, data warehouses
- `networking` - VPCs, subnets, load balancers, firewalls
- `iam` - Roles, policies, service accounts, groups

**Principles:**
- Provider-agnostic schemas
- Consistent variable naming
- Standardized output formats

### Layer 4: CORE (Orchestration)

Resource composition and dependency management.

**Contents:**
- Resource creation logic
- Dependency graphs
- Lifecycle management
- Resource tagging
- Conditional logic

**Principles:**
- Reusable composition patterns
- Clear dependency chains
- Idempotent operations

### Layer 5: FACADE (Public Interface)

User-facing modules with provider routing.

**Contents:**
- Provider selection logic
- Size normalization
- Unified interface
- Error handling

**Principles:**
- Stable public API
- Provider abstraction
- Ergonomic usage

## Design Principles

### 1. Provider Abstraction

Similar to CloudKit's provider pattern:

```hcl
# Facade module
module "compute" {
  source = "../facade/compute"
  
  provider_name = "aws"        # or "azure", "gcp", "oracle"
  instance_size = "medium"     # Normalized size
  instance_name = "web-server"
  
  provider_config = {
    # Provider-specific config
  }
}
```

### 2. Size Normalization

Following CloudKit's approach:

```hcl
# common/locals.tf
locals {
  compute_sizes = {
    aws = {
      small  = "t3.micro"
      medium = "t3.medium"
      large  = "m5.large"
    }
    azure = {
      small  = "Standard_B1s"
      medium = "Standard_B2s"
      large  = "Standard_DS2_v2"
    }
    gcp = {
      small  = "e2-micro"
      medium = "e2-medium"
      large  = "n2-standard-2"
    }
  }
}
```

### 3. Dependency Flow

```
User Application
       │
       ▼
┌─────────────┐
│   Facade    │  (Provider routing)
└─────────────┘
       │
       ▼
┌─────────────┐
│    Core     │  (Resource composition)
└─────────────┘
       │
       ▼
┌─────────────┐
│  Providers  │  (AWS/Azure/GCP/Oracle)
└─────────────┘
```

### 4. Feature Flags

Using Terraform's conditional logic:

```hcl
# Enable/disable features
module "aws_compute" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../aws/core/compute"
  # ...
}

module "azure_compute" {
  count  = var.provider_name == "azure" ? 1 : 0
  source = "../../azure/core/compute"
  # ...
}
```

### 5. Testing Strategy

1. **Unit Tests** - Using `terraform validate`
2. **Integration Tests** - Using Terratest
3. **Mock Support** - Using `test` provider
4. **Contract Tests** - Validating API schemas

## Error Handling

Unified validation:

```hcl
variable "provider" {
  type = string
  validation {
    condition     = contains(["aws", "azure", "gcp", "oracle"], var.provider)
    error_message = "Provider must be one of: aws, azure, gcp, oracle."
  }
}

variable "instance_size" {
  type = string
  validation {
    condition     = contains(["small", "medium", "large"], var.instance_size)
    error_message = "Size must be one of: small, medium, large."
  }
}
```

## Observability

Built-in support for:
- **Tagging** - Standard tags on all resources
- **Monitoring** - CloudWatch/Azure Monitor/Stackdriver integration
- **Logging** - Resource change tracking
- **Cost Tracking** - Cost allocation tags

## Comparison with CloudKit SEA

| Aspect | CloudKit (Rust) | IAC (Terraform) |
|--------|-----------------|-----------------|
| **Facade** | `cloudkit` crate | `facade/` modules |
| **Core** | `cloudkit_core` | `core/` modules |
| **API** | `cloudkit_api` traits | `api/` schemas |
| **SPI** | `cloudkit_spi` | `spi/` provider configs |
| **Common** | `cloudkit_spi` types | `common/` definitions |
| **Providers** | `cloudkit-aws`, etc. | `providers/` modules |
| **Testing** | Unit + Integration | Terratest + validation |
| **Extension** | Trait implementation | Module composition |

## Migration Path

Current → Target structure:

```
Target:
iac/
├── common/              # Shared definitions
├── api/                 # Resource contracts
├── facade/              # Unified Orchestration
│   └── compute/
├── aws/
│   ├── core/            # AWS Implementations
│   └── spi/             # AWS Credentials
├── azure/
└── gcp/
```

## Summary

The Multi-Cloud IAC Framework provides a robust, layer-separated approach to infrastructure management. By strictly adhering to the SEA pattern, it ensures that infrastructure logic remains portable, testable, and consistent across multiple cloud providers.

**Key Takeaways**:
1. **Contract First**: Always define `iac_api` before implementing provider logic.
2. **Normalized Inputs**: Use `common` layer sizes and classes for consistency.
3. **Decoupled Providers**: Keep `iac_core` implementations provider-specific within their own source directories.

---

**Related Documentation**:
- [Testing Strategy](../5-testing/testing-strategy.md) - How each layer is verified.
- [Toolchain Specification](./toolchain.md) - Tools used in the development and testing process.

**Last Updated**: 2026-01-14  
**Version**: 1.0  
