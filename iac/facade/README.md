# Facade Layer README
# Public Interface for Infrastructure Resources

## Overview

The `facade/` layer provides the **public-facing interface** for the IAC SEA architecture. This is what users interact with directly - clean, simple, and well-documented modules that hide the complexity of the underlying layers.

## Layer Position

```
Layer 1: COMMON
    ↓
Layer 2: SPI
    ↓
Layer 3: API
    ↓
Layer 4: CORE
    ↓
Layer 5: FACADE ← You are here (USER INTERFACE)
```

## Purpose

The Facade layer serves as the **entry point** for users:

1. **Simplicity** - Hide internal complexity
2. **Documentation** - Self-documenting with examples
3. **Validation** - Catch errors early with clear messages
4. **Convenience** - Sensible defaults, minimal required inputs
5. **Consistency** - Unified interface across all resource types

## Philosophy

> **"Make the simple things simple, and the complex things possible."**

- **Simple:** Create a basic resource with 3-4 parameters
- **Complex:** Advanced configurations available via optional parameters

## Resource Modules

### Compute (`facade/compute/`)

Create virtual machine instances across clouds with a simple interface.

**Minimal Example:**
```hcl
module "web_server" {
  source = "./facade/compute"
  
  provider_name = "aws"
  instance_name = "web-01"
  instance_size = "medium"
  project_name  = "my-app"
  environment   = "prod"
}
```

**Advanced Example:**
```hcl
module "app_server" {
  source = "./facade/compute"
  
  # Required
  provider_name = "aws"
  instance_name = "app-server-01"
  instance_size = "large"
  project_name  = "my-app"
  environment   = "prod"
  
  # Network
  allow_public_access = true
  subnet_id           = "subnet-xxxxx"
  security_group_ids  = ["sg-xxxxx"]
  
  # Access
  ssh_public_key = file("~/.ssh/id_rsa.pub")
  admin_username = "admin"
  
  # Features
  enable_monitoring = true
  enable_backup     = true
  
  # Initialization
  user_data = file("./scripts/init.sh")
  
  # Provider-specific
  provider_config = {
    ami                   = "ami-0c55b159cbfafe1f0"
    instance_profile_name = "MyInstanceProfile"
    ebs_optimized         = true
  }
  
  # Tags
  tags = {
    Team = "Platform"
    Cost = "Engineering"
  }
}
```

**Outputs:**
```hcl
output "server_details" {
  value = module.app_server.instance
  # Returns: { id, arn, type, size, public_ip, private_ip, ssh_connection, state, zone, tags }
}

output "connect" {
  value     = module.app_server.ssh_connection
  sensitive = true
  # Returns: "ssh admin@54.123.456.789"
}
```

### Storage (`facade/storage/`)

Create object storage buckets across clouds with lifecycle management.

**Minimal Example:**
```hcl
module "data_bucket" {
  source = "./facade/storage"
  
  provider_name = "aws"
  bucket_name  = "my-data-prod"
  project_name = "my-app"
  environment  = "prod"
}
```

**Advanced Example:**
```hcl
module "archive_bucket" {
  source = "./facade/storage"
  
  # Required
  provider_name = "aws"
  bucket_name  = "my-archive-storage"
  project_name = "my-app"
  environment  = "prod"
  
  # Storage configuration
  storage_class       = "standard"
  versioning_enabled  = true
  encryption_enabled  = true
  public_access_block = true
  
  # Logging
  enable_logging  = true
  log_bucket_name = "my-logs-bucket"
  
  # CORS for web access
  cors_rules = [{
    allowed_origins = ["https://app.example.com"]
    allowed_methods = ["GET", "HEAD"]
    allowed_headers = ["*"]
    max_age_seconds = 3000
  }]
  
  # Lifecycle management
  lifecycle_rules = [{
    id      = "transition-to-archive"
    enabled = true
    prefix  = "old-data/"
    
    transition = [
      {
        days          = 30
        storage_class = "infrequent"
      },
      {
        days          = 90
        storage_class = "archive"
      }
    ]
    
    expiration = {
      days = 365
    }
  }]
  
  # Disaster recovery
  replication_enabled     = true
  replication_destination = "my-archive-backup"
  
  # Tags
  tags = {
    DataClassification = "Confidential"
    RetentionPeriod    = "1-year"
  }
}
```

**Outputs:**
```hcl
output "bucket_details" {
  value = module.archive_bucket.bucket
  # Returns: { id, arn, url, region, storage_class, versioning_enabled, encryption_enabled, tags }
}

output "bucket_url" {
  value = module.archive_bucket.bucket_url
  # Returns: "my-archive-storage.s3.amazonaws.com"
}
```

## Multi-Cloud Examples

### Same Resource, Different Providers

```hcl
# AWS instance
module "aws_server" {
  source = "./facade/compute"
  
  provider_name = "aws"
  instance_name = "web-aws"
  instance_size = "medium"
  project_name  = "multi-cloud-app"
  
  provider_config = {
    ami = "ami-xxxxx"
  }
}

# Azure instance
module "azure_server" {
  source = "./facade/compute"
  
  provider_name = "azure"
  instance_name = "web-azure"
  instance_size = "medium"  # Same size, different type!
  project_name  = "multi-cloud-app"
  
  provider_config = {
    resource_group_name = "rg-prod"
    location            = "eastus"
  }
}

# GCP instance
module "gcp_server" {
  source = "./facade/compute"
  
  provider_name = "gcp"
  instance_name = "web-gcp"
  instance_size = "medium"  # Same size, different type!
  project_name  = "multi-cloud-app"
  
  provider_config = {
    project_id = "my-project"
    zone       = "us-central1-a"
  }
}
```

**What happens behind the scenes:**
```
User: instance_size = "medium"
  ↓
Facade: Passes to Core
  ↓
Core: Looks up mapping
  ↓
AWS:   medium → t3.medium
Azure: medium → Standard_B2s
GCP:   medium → e2-medium
  ↓
Providers: Create instances with correct types
```

## Size Normalization

### Compute Sizes

| Size   | AWS       | Azure           | GCP           | Use Case          |
|--------|-----------|-----------------|---------------|-------------------|
| small  | t3.micro  | Standard_B1s    | e2-micro      | Dev/Test          |
| medium | t3.medium | Standard_B2s    | e2-medium     | Small Production  |
| large  | m5.large  | Standard_DS2_v2 | n2-standard-2 | Production        |
| xlarge | m5.xlarge | Standard_DS3_v2 | n2-standard-4 | High Performance  |

### Storage Classes

| Class      | AWS           | Azure   | GCP      | Use Case               |
|------------|---------------|---------|----------|------------------------|
| standard   | STANDARD      | Hot     | STANDARD | Frequent access        |
| infrequent | STANDARD_IA   | Cool    | NEARLINE | Occasional access      |
| archive    | GLACIER       | Archive | COLDLINE | Rare access            |
| cold       | DEEP_ARCHIVE  | Archive | ARCHIVE  | Long-term archive      |

## Feature Flags

Control optional features with boolean flags:

```hcl
# Security
encryption_enabled   = true   # Encrypt data at rest
public_access_block  = true   # Block public access

# Operations
enable_monitoring    = true   # Enable cloud monitoring
enable_backup        = true   # Enable automated backups
enable_logging       = true   # Enable access logs

# Network
allow_public_access  = false  # Assign public IP

# Data Management
versioning_enabled   = true   # Enable object versioning
replication_enabled  = true   # Enable cross-region replication
```

## Tag Hierarchy

Tags are merged in this order (later overrides earlier):

1. **Common tags** (automatic)
   ```hcl
   {
     ManagedBy    = "Terraform"
     Environment  = "prod"
     provider_name = "aws"
     Project      = "my-app"
     Architecture = "SEA"
   }
   ```

2. **Resource-type tags** (automatic)
   ```hcl
   {
     ResourceType = "Compute"
     Service      = "VirtualMachine"
     InstanceName = "web-01"
     Size         = "medium"
   }
   ```

3. **User tags** (your `tags` variable)
   ```hcl
   {
     Team = "Platform"
     Cost = "Engineering"
   }
   ```

**Final result:** All tags merged together!

## Error Messages

The facade provides clear, actionable error messages:

```hcl
# Invalid provider
Error: Provider must be one of: aws, azure, gcp, oracle

# Invalid instance name
Error: Instance name must be lowercase alphanumeric with hyphens, 
       starting and ending with alphanumeric

# Invalid size
Error: Instance size must be one of: small, medium, large, xlarge

# Missing required provider config
Error: When using AWS, provider_config.ami is required
```

## Best Practices

### 1. Use Modules for Each Environment

```hcl
# environments/prod/main.tf
module "prod_server" {
  source = "../../facade/compute"
  
  provider_name = "aws"
  environment  = "prod"
  instance_size = "large"
  # ...
}

# environments/dev/main.tf
module "dev_server" {
  source = "../../facade/compute"
  
  provider_name = "aws"
  environment  = "dev"
  instance_size = "small"  # Smaller for dev
  # ...
}
```

### 2. Extract Common Variables

```hcl
# variables.tf
variable "project_name" {
  default = "my-app"
}

variable "provider" {
  default = "aws"
}

# main.tf
module "compute" {
  source = "./facade/compute"
  
  project_name = var.project_name
  provider_name = var.provider
  # ...
}
```

### 3. Use Descriptive Names

```hcl
# ✅ Good
instance_name = "api-server-prod-01"
bucket_name   = "user-uploads-prod"

# ❌ Bad
instance_name = "server1"
bucket_name   = "bucket"
```

### 4. Enable Security Features

```hcl
# Always secure by default
encryption_enabled   = true
public_access_block  = true
enable_monitoring    = true
```

## Quick Reference

### Compute Module

```hcl
module "instance" {
  source = "./facade/compute"
  
  # Required (4 parameters minimum)
  provider_name = "aws" | "azure" | "gcp" | "oracle"
  instance_name = "name"
  project_name  = "project"
  environment   = "dev" | "staging" | "prod"
  
  # Common optional
  instance_size       = "small" | "medium" | "large" | "xlarge"
  ssh_public_key      = "ssh-rsa ..."
  allow_public_access = true | false
  
  # See variables.tf for full options
}
```

### Storage Module

```hcl
module "bucket" {
  source = "./facade/storage"
  
  # Required (4 parameters minimum)
  provider_name = "aws" | "azure" | "gcp" | "oracle"
  bucket_name  = "name"
  project_name = "project"
  environment  = "dev" | "staging" | "prod"
  
  # Common optional
  storage_class       = "standard" | "infrequent" | "archive" | "cold"
  versioning_enabled  = true | false
  encryption_enabled  = true | false
  
  # See variables.tf for full options
}
```

## Related Documentation

- [Common Layer](../common/README.md) - Size mappings and tags
- [Core Layer](../core/README.md) - Orchestration logic
- [API Layer](../api/README.md) - Resource contracts
- [Examples](../../examples/README.md) - Complete examples

---

**Status:** Phase 5 Complete ✅  
**Next:** Phase 6 - Migration & Testing
