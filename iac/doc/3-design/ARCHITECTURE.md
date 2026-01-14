# IAC (Infrastructure as Code) SEA Architecture

## Overview

This IAC follows the **Stratified Encapsulation Architecture (SEA)** pattern used in CloudKit, providing a unified interface for multi-cloud infrastructure provisioning across AWS, Azure, GCP, and Oracle Cloud.

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

```
iac/
├── common/                    # Layer 1: COMMON
│   ├── variables.tf          # Shared variable definitions
│   ├── locals.tf             # Size mappings, conventions
│   ├── tags.tf               # Tagging standards
│   └── README.md             # Layer documentation
│
├── iac_spi/                   # Layer 2: SPI (Service Provider Interface)
│   ├── aws/
│   │   ├── provider.tf       # AWS provider configuration
│   │   ├── backend.tf        # State backend
│   │   └── variables.tf      # AWS-specific variables
│   ├── azure/
│   └── gcp/
│
├── iac_api/                   # Layer 3: API (Resource Contracts)
│   ├── compute/
│   │   ├── outputs.tf        # Output schema
│   │   └── variables.tf      # Input schema
│   ├── storage/
│   ├── database/
│   ├── networking/
│   └── iam/
│
├── iac_core/                  # Layer 4: CORE (Orchestration)
│   ├── aws/src/              # AWS internal implementations
│   ├── azure/src/            # Azure internal implementations
│   └── gcp/src/              # GCP internal implementations
│
├── facade/                   # Layer 5: FACADE (Public Interface)
│   ├── compute/
│   ├── storage/
│   ├── database/
│   ├── networking/
│   └── iam/
│
└── examples/                 # Usage examples
    ├── web-app/
    ├── data-pipeline/
    └── multi-cloud/
```

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
  
  provider      = "aws"        # or "azure", "gcp", "oracle"
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
  count  = var.provider == "aws" ? 1 : 0
  source = "../providers/aws/compute"
  # ...
}

module "azure_compute" {
  count  = var.provider == "azure" ? 1 : 0
  source = "../providers/azure/compute"
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
Current:
iac/
├── compute/
│   ├── main.tf (contains facade logic)
│   ├── aws/main.tf
│   ├── gcp/main.tf
│   └── azure/main.tf

Target:
iac/
├── common/              # NEW: Shared definitions
├── spi/                 # NEW: Provider configs
├── api/                 # NEW: Resource contracts
├── core/                # NEW: Orchestration
├── facade/
│   └── compute/
│       └── main.tf     # MOVE: From iac/compute/main.tf
└── providers/
    ├── aws/
    │   └── compute/    # MOVE: From iac/compute/aws/
    ├── azure/
    └── gcp/
```

## Benefits of SEA for IAC

1. **Modularity** - Clear boundaries between layers
2. **Testability** - Test each layer independently
3. **Extensibility** - Add providers without changing core
4. **Maintainability** - Changes isolated to specific layers
5. **Consistency** - Standardized patterns across resources
6. **Provider Agnostic** - Switch providers with minimal changes

## Next Steps

1. Create `common/` layer with shared definitions
2. Implement `spi/` layer for provider setup
3. Define `api/` contracts for each resource type
4. Build `core/` orchestration modules
5. Refactor `facade/` for unified interface
6. Migrate existing modules to new structure
7. Add comprehensive examples
8. Implement testing framework
