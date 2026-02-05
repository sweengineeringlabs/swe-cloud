# ADR: Package by Provider (Vertical Slicing)

**Status:** Accepted  
**Date:** 2026-01-14  
**Deciders:** Infrastructure Team  
**Context:** IAC Multi-Cloud Terraform Organization

---

## Context and Problem Statement

When organizing multi-cloud infrastructure code with multiple providers (AWS, Azure, GCP) and multiple resource types (compute, storage, database, networking), we must decide:

**Should we organize Terraform modules by provider (vertical slicing) or by resource type (horizontal slicing)?**

### Two Approaches

**Option A: Package by Resource Type (Horizontal Slicing)**
```
iac/core/
├── compute/
│   └── main.tf    # Routes to aws/compute, azure/compute, gcp/compute
├── storage/
│   └── main.tf    # Routes to aws/storage, azure/storage, gcp/storage
└── database/
    └── main.tf    # Routes to aws/database, azure/database, gcp/database

iac/providers/
├── aws/
│   ├── compute/
│   ├── storage/
│   └── database/
├── azure/
│   ├── compute/
│   ├── storage/
│   └── database/
└── gcp/
    ├── compute/
    ├── storage/
    └── database/
```

**Option B: Package by Provider (Vertical Slicing)** ✅
```
iac/core/
├── aws/
│   └── main.tf    # EC2, S3, RDS, VPC, etc.
├── azure/
│   └── main.tf    # VM, Blob, SQL, VNet, etc.
└── gcp/
    └── main.tf    # Compute, GCS, CloudSQL, VPC, etc.
```

---

## Decision Drivers

1. **CloudKit Alignment** - Match SDK organization for consistency
2. **Team Organization** - Infrastructure teams by cloud provider
3. **Code Cohesion** - Related resources should be together
4. **Provider State** - Terraform provider configuration
5. **Navigation** - Ease of finding infrastructure code
6. **Change Patterns** - How infrastructure changes happen
7. **Dependency Management** - Provider-specific dependencies
8. **Mental Model** - Consistent thinking across SDK and IaC

---

## Considered Options

### Option A: Package by Resource Type (Horizontal Slicing)

**Pros:**
- ✅ Easy to see all compute configurations across providers
- ✅ Resource-specific logic centralized
- ✅ Facilitates resource-type abstractions

**Cons:**
- ❌ Provider-specific code scattered across directories
- ❌ Each provider change touches multiple directories
- ❌ Difficult to manage provider state
- ❌ Team boundaries unclear
- ❌ Adding new provider touches many files
- ❌ Different pattern than CloudKit SDK
- ❌ Multiple provider blocks per resource type

### Option B: Package by Provider (Vertical Slicing) ✅

**Pros:**
- ✅ Matches CloudKit SDK organization
- ✅ High cohesion: all AWS resources together
- ✅ Clear team ownership: AWS team owns `core/aws/`
- ✅ Single provider block per provider module
- ✅ Easy navigation: "Need AWS?" → `core/aws/`
- ✅ Adding provider_name = one new directory
- ✅ Provider-specific optimizations isolated
- ✅ Shared locals/data sources within provider
- ✅ Consistent mental model with SDK

**Cons:**
- ❌ Harder to see all compute implementations at once
- ❌ Cross-provider abstractions require facade layer

---

## Decision Outcome

**Chosen option: Package by Provider (Option B)** ✅

### Rationale

1. **Cloud SDK Consistency**
   - CloudKit SDK uses provider grouping
   - Same mental model across SDK and IaC
   - Developers switch seamlessly between code and infrastructure

2. **Team Boundaries**
   - Real-world teams: AWS team, Azure team, GCP team
   - Each team owns their provider's infrastructure
   - Reduces coordination overhead

3. **Provider Cohesion**
   - AWS resources often reference each other
   - VPC → Subnets → Instances → Security Groups
   - Easier when all in same file/module

4. **Terraform Provider Management**
   ```hcl
   # core/aws/main.tf - Single provider block
   provider "aws" {
     region = var.aws_region
     default_tags { tags = local.aws_tags }
   }
   
   # All AWS resources use this provider
   resource "aws_instance" "compute" { ... }
   resource "aws_s3_bucket" "storage" { ... }
   resource "aws_db_instance" "database" { ... }
   ```

5. **Change Patterns**
   - 90% of infrastructure changes affect single provider
   - AWS API changes affect one directory
   - Adding new AWS service only touches `core/aws/`

6. **Mental Model Alignment**
   ```
   CloudKit Code:  cloudkit_core/aws/s3.rs
   IAC Code:       iac/core/aws/main.tf (S3 bucket)
   ↑ Same navigation pattern!
   ```

---

## Implementation

### Directory Structure
```
iac_core/              # Renamed from iac/core to match cloudkit_core
├── aws/
│   └── src/           # Matches cloudkit_core/aws/src/
│       ├── compute/   # Like cloudkit_core/aws/src/ec2.rs
│       ├── storage/   # Like cloudkit_core/aws/src/s3.rs
│       ├── database/  # Like cloudkit_core/aws/src/dynamodb.rs
│       ├── networking/# Like cloudkit_core/aws/src/vpc.rs
│       └── iam/       # Like cloudkit_core/aws/src/iam.rs
├── azure/
│   └── src/
│       ├── compute/
│       ├── storage/
│       └── database/
└── gcp/
    └── src/
        ├── compute/
        ├── storage/
        └── database/
```

### Provider Module Pattern
Each resource type is a separate module:

```hcl
# iac_core/aws/src/compute/main.tf
terraform {
  required_providers {
    aws = { source = "hashicorp/aws", version = "~> 5.0" }
  }
}

resource "aws_instance" "this" {
  ami           = var.ami
  instance_type = var.instance_type
  # ...
}

output "instance_id" {
  value = aws_instance.this.id
}
```

```hcl
# iac_core/aws/src/storage/main.tf
resource "aws_s3_bucket" "this" {
  bucket = var.bucket_name
  # ...
}

output "bucket_id" {
  value = aws_s3_bucket.this.id
}
```

### Usage from Facade
```hcl
# facade/main.tf
module "aws_compute" {
  source = "../../iac_core/aws/src/compute"
  
  ami           = "ami-xxxxx"
  instance_type = "t3.medium"
  tags          = local.tags
}

module "aws_storage" {
  source = "../../iac_core/aws/src/storage"
  
  bucket_name = "my-bucket"
  tags        = local.tags
}

output "instance_ip" {
  value = module.aws_compute.public_ip
}
```

### Team Ownership
```
CODEOWNERS:
/iac_core/aws/     @aws-infrastructure-team
/iac_core/azure/   @azure-infrastructure-team
/iac_core/gcp/     @gcp-infrastructure-team
```

---

## Consequences

### Positive

1. **CloudKit Alignment**
   - Developers learn one pattern, use everywhere
   - Code reviews easier (same structure)
   - Onboarding simplified

2. **Provider Isolation**
   - AWS state separate from Azure state
   - Provider failures isolated
   - Easier to troubleshoot

3. **Team Efficiency**
   - AWS team doesn't conflict with Azure team
   - Clear ownership boundaries
   - Parallel development

4. **Infrastructure Cohesion**
   ```hcl
   # Easy to reference within provider
   resource "aws_instance" "compute" {
     vpc_id    = aws_vpc.network[0].id
     subnet_id = aws_subnet.subnet[0].id
   }
   ```

5. **Conditional Resources**
   ```hcl
   # Enable only what's needed
   module "aws" {
     compute_config = { ... }  # Create compute
     # storage_config not set   # Skip storage
   }
   ```

### Negative

1. **Cross-Provider Abstractions Harder**
   - **Solution:** Use facade layer for abstractions
   - Facade provides unified interface, routes to providers

2. **Resource Type Overview**
   - **Solution:** Documentation shows all compute, storage, etc.
   - Facade layer provides resource-type views

### Mitigation Strategies

**For Cross-Provider Abstractions:**
```hcl
# facade/compute/main.tf
module "compute_core" {
  source = "../../core/${var.provider}"
  
  compute_config = {
    # Normalized configuration
    instance_type = local.instance_types[var.provider][var.size]
    # ...
  }
}
```

**For Shared Patterns:**
```hcl
# common/locals.tf
locals {
  # Shared across all providers
  compute_instance_types = {
    aws   = { medium = "t3.medium" }
    azure = { medium = "Standard_B2s" }
    gcp   = { medium = "e2-medium" }
  }
}
```

---

## Comparison with CloudKit

| Aspect | CloudKit (SDK) | IAC (Terraform) | Consistency |
|--------|----------------|-----------------|-------------|
| **Organization** | By Provider | By Provider | ✅ Match |
| **AWS Location** | `cloudkit_core/aws/src/` | `iac_core/aws/src/` | ✅ Match |
| **All Services** | `s3.rs`, `dynamodb.rs`, etc. | `compute/`, `storage/`, etc. | ✅ Match |
| **Navigation** | "Want AWS?" → `aws/src/` | "Want AWS?" → `iac_core/aws/src/` | ✅ Match |
| **Team Ownership** | AWS team owns `aws/` | AWS team owns `iac_core/aws/` | ✅ Match |
| **Mental Model** | Provider-first | Provider-first | ✅ Match |
| **Structure Depth** | `aws/src/s3.rs` (3 levels) | `aws/src/storage/` (3 levels) | ✅ Match |

**Result:** Perfect alignment! 🎯

---

## Related Decisions

- [ADR: SEA Architecture](../ARCHITECTURE.md)
- [ADR: Facade Layer Pattern](./facade-pattern.md)
- [ADR: Size Normalization](./size-normalization.md)

---

## References

- **CloudKit Package Strategy** - `cloudkit/crates/doc/3-design/15-package-strategy.md`
- **Domain-Driven Design** - Eric Evans
- **Terraform Best Practices** - HashiCorp
- **Conway's Law** - System design mirrors org structure
- **Vertical Slice Architecture** - Jimmy Bogard

---

## Migration Path

**From:** Resource-grouped (Horizontal)
```
core/compute/main.tf  → Routes to providers
core/storage/main.tf  → Routes to providers
providers/aws/...
providers/azure/...
```

**To:** Provider-grouped (Vertical) ✅
```  
iac_core/aws/src/compute/    → AWS compute module
iac_core/aws/src/storage/    → AWS storage module
iac_core/azure/src/compute/  → Azure compute module
iac_core/gcp/src/compute/    → GCP compute module
```

**Steps:**
1. ✅ Rename `iac/core` to `iac_core` (match cloudkit_core)
2. ✅ Create `iac_core/aws/src/` structure
3. ✅ Create separate modules per resource type
4. ✅ Update facade to use new module paths
5. ✅ Remove old resource-grouped orchestration
6. ✅ Update documentation

---

**Decision:** Package by Provider (Vertical Slicing) ✅  
**Aligns with:** CloudKit SDK, DDD, Conway's Law, Team Boundaries  
**Result:** `iac_core/aws/src/compute/`, `iac_core/aws/src/storage/`, etc.  
**Benefit:** Same mental model across codebase and infrastructure
