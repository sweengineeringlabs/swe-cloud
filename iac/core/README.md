# Core Layer - Provider-Grouped Modules
# Following CloudKit's provider-first organization

## Structure

Matching CloudKit's organization where each provider contains ALL its resources:

```
core/
├── aws/              ← All AWS resources together
│   ├── main.tf       (compute, storage, database, network,...)
│   └── variables.tf
├── azure/            ← All Azure resources together
│   ├── main.tf       (compute, storage, database, network,...)
│   └── variables.tf
└── gcp/              ← All GCP resources together
    ├── main.tf       (compute, storage, database, network,...)
    └── variables.tf
```

## CloudKit Parallel

**CloudKit Structure:**
```
cloudkit_core/
├── aws/
│   ├── s3.rs
│   ├── dynamodb.rs
│   ├── lambda.rs
│   ├── sqs.rs
│   └── ...          ← All AWS services together
├── azure/
│   ├── blob.rs
│   ├── cosmos.rs
│   └── ...          ← All Azure services together
└── gcp/
    ├── gcs.rs
    ├── firestore.rs
    └── ...          ← All GCP services together
```

**IAC Structure (Now Matching!):**
```
core/
├── aws/
│   └── main.tf      ← EC2, S3, RDS, VPC, etc.
├── azure/
│   └── main.tf      ← VM, Blob, Cosmos, VNet, etc.
└── gcp/
    └── main.tf      ← Compute, GCS, Firestore, VPC, etc.
```

## Benefits

1. **Provider Cohesion** - All AWS logic in one place
2. **Easy Navigation** - Want AWS? Go to `core/aws/`
3. **Clear Ownership** - AWS team owns `core/aws/`
4. **Shared State** - Provider resources can share local state
5. **Matches CloudKit** - Same mental model across SDK and IaC

## Usage

### From Facade/Orchestration Layer:

```hcl
# Route to entire AWS provider module
module "aws" {
  source = "../../core/aws"
  
  compute_config = {
    ami           = "ami-xxxxx"
    instance_type = "t3.medium"
    tags          = local.tags
  }
  
  storage_config = {
    bucket_name = "my-bucket"
    tags        = local.tags
  }
}

# Access outputs
output "instance" {
  value = module.aws.compute
}

output "bucket" {
  value = module.aws.storage
}
```

### Provider Module Pattern:

Each provider module contains:
- **All resource types** for that provider
- **Conditional creation** via `count` based on config
- **Unified outputs** structured by resource type

```hcl
# core/aws/main.tf
resource "aws_instance" "compute" {
  count = var.compute_config != null ? 1 : 0
  # ...
}

resource "aws_s3_bucket" "storage" {
  count = var.storage_config != null ? 1 : 0
  # ...
}

output "compute" {
  value = var.compute_config != null ? { ... } : null
}
```

## Migration from Resource-Grouped

**Before (Resource-Grouped):**
```
core/
├── compute/     ← All providers' compute together
│   └── main.tf  (routes to aws/compute, azure/compute, etc.)
└── storage/     ← All providers' storage together
    └── main.tf  (routes to aws/storage, azure/storage, etc.)
```

**After (Provider-Grouped):** ✅
```
core/
├── aws/         ← All AWS resources together
│   └── main.tf  (compute, storage, database, etc.)
└── azure/       ← All Azure resources together
    └── main.tf  (compute, storage, database, etc.)
```

## Key Differences

| Aspect | Resource-Grouped | Provider-Grouped (CloudKit) |
|--------|------------------|------------------------------|
| **Organization** | By resource type | By cloud provider |
| **File Count** | Many small files | Fewer larger files |
| **Navigation** | "Where's compute?" → `core/compute/` | "Where's AWS?" → `core/aws/` |
| **Shared Logic** | Hard (providers separate) | Easy (same file) |
| **Team Ownership** | By resource type | By cloud provider |
| **Matches CloudKit** | ❌ No | ✅ Yes |

## Example: Adding Database

**Resource-Grouped Approach:**
```
core/database/main.tf  ← Create new orchestration file
providers/aws/database/main.tf  ← Create AWS impl
providers/azure/database/main.tf  ← Create Azure impl
```

**Provider-Grouped Approach:** ✅
```
# Just add to existing core/aws/main.tf:
resource "aws_db_instance" "database" {
  count = var.database_config != null ? 1 : 0
  # ...
}

output "database" {
  value = var.database_config != null ? { ... } : null
}
```

## Advantages of Provider-Grouping

1. **Less File Navigation** - Everything for AWS in one place
2. **Shared Provider Config** - Single provider block
3. **Inter-Resource References** - Easy to reference within provider
4. **Clearer Dependencies** - See all AWS resources together
5. **Team Boundaries** - AWS team, Azure team, GCP team
6. **Matches CloudKit** - Consistent mental model

---

**Status:** Restructured to match CloudKit ✅  
**Organization:** Provider-first (like CloudKit SDK)  
**Benefit:** Same mental model across SDK and IaC
