# IAC SEA Implementation Plan

## Phase 1: Foundation (Common Layer)

### 1.1 Create Shared Definitions

**File:** `iac/common/variables.tf`
```hcl
# Standard variable schemas used across all modules

variable "provider" {
  description = "Cloud provider: aws, azure, gcp, oracle"
  type        = string
  validation {
    condition     = contains(["aws", "azure", "gcp", "oracle"], var.provider)
    error_message = "Valid providers: aws, azure, gcp, oracle"
  }
}

variable "environment" {
  description = "Environment: dev, staging, prod"
  type        = string
  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "Valid environments: dev, staging, prod"
  }
}

variable "resource_size" {
  description = "Resource size: small, medium, large, xlarge"
  type        = string
  default     = "small"
  validation {
    condition     = contains(["small", "medium", "large", "xlarge"], var.resource_size)
    error_message = "Valid sizes: small, medium, large, xlarge"
  }
}

variable "tags" {
  description = "Common tags to apply to all resources"
  type        = map(string)
  default     = {}
}
```

**File:** `iac/common/locals.tf`
```hcl
# Size mappings and normalization

locals {
  # Compute instance type mappings
  compute_instance_types = {
    aws = {
      small  = "t3.micro"
      medium = "t3.medium"
      large  = "m5.large"
      xlarge = "m5.xlarge"
    }
    azure = {
      small  = "Standard_B1s"
      medium = "Standard_B2s"
      large  = "Standard_DS2_v2"
      xlarge = "Standard_DS3_v2"
    }
    gcp = {
      small  = "e2-micro"
      medium = "e2-medium"
      large  = "n2-standard-2"
      xlarge = "n2-standard-4"
    }
    oracle = {
      small  = "VM.Standard.E4.Flex"
      medium = "VM.Standard.E4.Flex"
      large  = "VM.Standard3.Flex"
      xlarge = "VM.Standard3.Flex"
    }
  }

  # Storage size mappings (GB)
  storage_sizes = {
    small  = 20
    medium = 100
    large  = 500
    xlarge = 1000
  }

  # Database instance type mappings
  database_instance_types = {
    aws = {
      small  = "db.t3.micro"
      medium = "db.t3.medium"
      large  = "db.r5.large"
      xlarge = "db.r5.xlarge"
    }
    azure = {
      small  = "GP_Gen5_2"
      medium = "GP_Gen5_4"
      large  = "GP_Gen5_8"
      xlarge = "GP_Gen5_16"
    }
    gcp = {
      small  = "db-n1-standard-1"
      medium = "db-n1-standard-2"
      large  = "db-n1-standard-4"
      xlarge = "db-n1-standard-8"
    }
  }

  # Network CIDR blocks
  network_cidrs = {
    small  = "10.0.0.0/24"   # 256 addresses
    medium = "10.0.0.0/20"   # 4096 addresses
    large  = "10.0.0.0/16"   # 65536 addresses
    xlarge = "10.0.0.0/12"   # 1M addresses
  }
}
```

**File:** `iac/common/tags.tf`
```hcl
# Standard tagging schema

locals {
  common_tags = merge(
    var.tags,
    {
      ManagedBy   = "Terraform"
      Environment = var.environment
      provider_name = var.provider
      Project     = var.project_name
      CostCenter  = var.cost_center
      Owner       = var.owner
      CreatedAt   = timestamp()
    }
  )

  # Provider-specific tag formats
  aws_tags = {
    for k, v in local.common_tags :
    k => v
  }

  azure_tags = {
    for k, v in local.common_tags :
    lower(k) => v
  }

  gcp_labels = {
    for k, v in local.common_tags :
    lower(replace(k, " ", "_")) => lower(v)
  }
}
```

## Phase 2: SPI Layer (Provider Integration)

### 2.1 AWS Provider Configuration

**File:** `iac/spi/aws/provider.tf`
```hcl
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = var.aws_region

  default_tags {
    tags = local.common_tags
  }

  # Assume role if provided
  dynamic "assume_role" {
    for_each = var.aws_assume_role != null ? [1] : []
    content {
      role_arn     = var.aws_assume_role.role_arn
      session_name = var.aws_assume_role.session_name
    }
  }
}
```

**File:** `iac/spi/aws/backend.tf`
```hcl
terraform {
  backend "s3" {
    bucket         = var.state_bucket
    key            = "${var.environment}/${var.project_name}/terraform.tfstate"
    region         = var.aws_region
    encrypt        = true
    dynamodb_table = var.state_lock_table
    
    # Enable versioning
    versioning = true
  }
}
```

### 2.2 Azure Provider Configuration

**File:** `iac/spi/azure/provider.tf`
```hcl
terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 4.0"
    }
  }
}

provider "azurerm" {
  features {
    resource_group {
      prevent_deletion_if_contains_resources = true
    }
    
    key_vault {
      purge_soft_delete_on_destroy = false
    }
    
    virtual_machine {
      delete_os_disk_on_deletion = true
    }
  }

  # Subscription configuration
  subscription_id = var.azure_subscription_id
  tenant_id       = var.azure_tenant_id
}
```

## Phase 3: API Layer (Resource Contracts)

### 3.1 Compute API Contract

**File:** `iac/api/compute/schema.tf`
```hcl
# Input Schema
variable "instance_name" {
  description = "Name of the compute instance"
  type        = string
  validation {
    condition     = can(regex("^[a-z0-9-]+$", var.instance_name))
    error_message = "Instance name must contain only lowercase letters, numbers, and hyphens"
  }
}

variable "instance_size" {
  description = "Normalized instance size"
  type        = string
}

variable "ssh_key" {
  description = "SSH public key for instance access"
  type        = string
  sensitive   = true
}

variable "allow_public_access" {
  description = "Whether to allow public internet access"
  type        = bool
  default     = false
}

# Output Schema
output "instance_id" {
  description = "Unique identifier of the compute instance"
  value       = local.instance_id
}

output "instance_type" {
  description = "Provider-specific instance type used"
  value       = local.instance_type
}

output "public_ip" {
  description = "Public IP address (if enabled)"
  value       = local.public_ip
}

output "private_ip" {
  description = "Private IP address"
  value       = local.private_ip
}

output "ssh_connection" {
  description = "SSH connection string"
  value       = local.ssh_connection
  sensitive   = true
}

output "metadata" {
  description = "Additional instance metadata"
  value = {
    provider_name = var.provider
    size          = var.instance_size
    created_at    = timestamp()
    tags          = local.common_tags
  }
}
```

### 3.2 Storage API Contract

**File:** `iac/api/storage/schema.tf`
```hcl
# Input Schema
variable "bucket_name" {
  description = "Name of the storage bucket"
  type        = string
  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]*[a-z0-9]$", var.bucket_name))
    error_message = "Bucket name must be lowercase alphanumeric with hyphens"
  }
}

variable "storage_class" {
  description = "Storage class/tier"
  type        = string
  default     = "standard"
  validation {
    condition     = contains(["standard", "infrequent", "archive", "cold"], var.storage_class)
    error_message = "Valid storage classes: standard, infrequent, archive, cold"
  }
}

variable "versioning_enabled" {
  description = "Enable object versioning"
  type        = bool
  default     = false
}

variable "encryption_enabled" {
  description = "Enable encryption at rest"
  type        = bool
  default     = true
}

variable "public_access_block" {
  description = "Block all public access"
  type        = bool
  default     = true
}

# Output Schema
output "bucket_id" {
  description = "Unique identifier of the storage bucket"
  value       = local.bucket_id
}

output "bucket_arn" {
  description = "ARN/resource ID of the bucket"
  value       = local.bucket_arn
}

output "bucket_url" {
  description = "Access URL for the bucket"
  value       = local.bucket_url
}

output "bucket_region" {
  description = "Region where bucket is located"
  value       = local.bucket_region
}
```

## Phase 4: Core Layer (Orchestration)

### 4.1 Compute Core Module

**File:** `iac/core/compute/main.tf`
```hcl
# Compute resource orchestration with dependencies

locals {
  # Provider-specific instance type
  instance_type = lookup(
    var.compute_instance_types[var.provider],
    var.instance_size,
    var.compute_instance_types[var.provider]["medium"]
  )

  # Network configuration
  network_config = var.network_id != null ? {
    network_id = var.network_id
    subnet_id  = var.subnet_id
  } : null

  # Security configuration
  security_config = {
    ssh_enabled        = var.ssh_key != null
    public_access      = var.allow_public_access
    firewall_rules     = var.firewall_rules
  }
}

# Dependency: Ensure network exists
data "terraform_provider" "network" {
  count = local.network_config != null ? 1 : 0
  # Network validation
}

# Core resource composition
module "instance" {
  source = "../../providers/${var.provider}/compute"

  instance_name = var.instance_name
  instance_type = local.instance_type
  ssh_key       = var.ssh_key
  
  network_config  = local.network_config
  security_config = local.security_config
  
  tags = local.common_tags
}

# Post-creation lifecycle
resource "null_resource" "instance_ready" {
  depends_on = [module.instance]

  provisioner "local-exec" {
    command = "echo 'Instance ${module.instance.instance_id} is ready'"
  }

  triggers = {
    instance_id = module.instance.instance_id
  }
}
```

### 4.2 Storage Core Module

**File:** `iac/core/storage/main.tf`
```hcl
# Storage resource orchestration

locals {
  # Provider-specific storage class mapping
  storage_class_mapping = {
    aws = {
      standard   = "STANDARD"
      infrequent = "STANDARD_IA"
      archive    = "GLACIER"
      cold       = "DEEP_ARCHIVE"
    }
    azure = {
      standard   = "Hot"
      infrequent = "Cool"
      archive    = "Archive"
      cold       = "Archive"
    }
    gcp = {
      standard   = "STANDARD"
      infrequent = "NEARLINE"
      archive    = "COLDLINE"
      cold       = "ARCHIVE"
    }
  }

  provider_storage_class = lookup(
    local.storage_class_mapping[var.provider],
    var.storage_class,
    local.storage_class_mapping[var.provider]["standard"]
  )
}

# Core resource
module "bucket" {
  source = "../../providers/${var.provider}/storage"

  bucket_name           = var.bucket_name
  storage_class         = local.provider_storage_class
  versioning_enabled    = var.versioning_enabled
  encryption_enabled    = var.encryption_enabled
  public_access_block   = var.public_access_block
  
  lifecycle_rules = var.lifecycle_rules
  cors_rules      = var.cors_rules
  
  tags = local.common_tags
}

# Lifecycle management
resource "null_resource" "bucket_lifecycle" {
  depends_on = [module.bucket]

  # Enforce retention policies
  provisioner "local-exec" {
    command = <<-EOT
      echo "Bucket ${module.bucket.bucket_id} created with:"
      echo "  Storage class: ${local.provider_storage_class}"
      echo "  Versioning: ${var.versioning_enabled}"
      echo "  Encryption: ${var.encryption_enabled}"
    EOT
  }
}
```

## Phase 5: Facade Layer (Public Interface)

### 5.1 Compute Facade

**File:** `iac/facade/compute/main.tf`
```hcl
# Compute facade with provider routing

module "compute_core" {
  source = "../../core/compute"

  # Pass through normalized inputs
  provider_name = var.provider
  instance_name = var.instance_name
  instance_size = var.instance_size
  ssh_key       = var.ssh_key
  
  # Provider-specific config
  network_id  = var.provider_config.network_id
  subnet_id   = var.provider_config.subnet_id
  
  # Feature flags
  allow_public_access = var.allow_public_access
  monitoring_enabled  = var.monitoring_enabled
  backup_enabled      = var.backup_enabled
  
  tags = merge(var.tags, local.common_tags)
}

# Unified outputs
output "instance" {
  description = "Compute instance details"
  value = {
    id          = module.compute_core.instance_id
    type        = module.compute_core.instance_type
    public_ip   = module.compute_core.public_ip
    private_ip  = module.compute_core.private_ip
    ssh         = module.compute_core.ssh_connection
    provider_name = var.provider
    size        = var.instance_size
  }
}
```

**File:** `iac/facade/compute/variables.tf`
```hcl
# Facade variables (provider-agnostic)

variable "provider" {
  description = "Cloud provider"
  type        = string
}

variable "instance_name" {
  description = "Instance name"
  type        = string
}

variable "instance_size" {
  description = "Normalized size: small, medium, large, xlarge"
  type        = string
  default     = "medium"
}

variable "ssh_key" {
  description = "SSH public key"
  type        = string
  sensitive   = true
}

variable "allow_public_access" {
  description = "Allow public internet access"
  type        = bool
  default     = false
}

variable "provider_config" {
  description = "Provider-specific configuration"
  type = object({
    network_id = optional(string)
    subnet_id  = optional(string)
    # AWS-specific
    ami            = optional(string)
    security_group = optional(string)
    # Azure-specific
    resource_group = optional(string)
    location       = optional(string)
    # GCP-specific
    project_id = optional(string)
    zone       = optional(string)
  })
  default = {}
}

variable "tags" {
  description = "Custom tags"
  type        = map(string)
  default     = {}
}
```

## Phase 6: Examples

### 6.1 Multi-Cloud Web Application

**File:** `iac/examples/web-app/main.tf`
```hcl
# Example: Multi-cloud web application

module "web_server_aws" {
  source = "../../facade/compute"

  provider_name = "aws"
  instance_name = "web-server-aws"
  instance_size = "medium"
  ssh_key       = file("~/.ssh/id_rsa.pub")
  
  allow_public_access = true
  
  provider_config = {
    ami            = "ami-0c55b159cbfafe1f0"
    security_group = aws_security_group.web.id
  }
  
  tags = {
    Application = "WebApp"
    Tier        = "Frontend"
  }
}

module "web_server_azure" {
  source = "../../facade/compute"

  provider_name = "azure"
  instance_name = "web-server-azure"
  instance_size = "medium"
  ssh_key       = file("~/.ssh/id_rsa.pub")
  
  allow_public_access = true
  
  provider_config = {
    resource_group = azurerm_resource_group.main.name
    location       = "eastus"
  }
  
  tags = {
    Application = "WebApp"
    Tier        = "Frontend"
  }
}

output "servers" {
  value = {
    aws   = module.web_server_aws.instance
    azure = module.web_server_azure.instance
  }
}
```

## Implementation Timeline

### Week 1: Foundation
- [ ] Create `common/` layer
- [ ] Define size mappings
- [ ] Establish tagging standards
- [ ] Document variable schemas

### Week 2: SPI Layer
- [ ] Implement AWS provider config
- [ ] Implement Azure provider config
- [ ] Implement GCP provider config
- [ ] Setup backend configurations

### Week 3: API Layer
- [ ] Define compute API contract
- [ ] Define storage API contract
- [ ] Define database API contract
- [ ] Define networking API contract

### Week 4: Core Layer
- [ ] Build compute orchestration
- [ ] Build storage orchestration
- [ ] Build database orchestration
- [ ] Build networking orchestration

### Week 5: Facade Layer
- [ ] Create compute facade
- [ ] Create storage facade
- [ ] Create database facade
- [ ] Create networking facade

### Week 6: Migration & Testing
- [ ] Migrate existing modules
- [ ] Create examples
- [ ] Write Terratest tests
- [ ] Documentation

## Testing Strategy

### 1. Validation Tests
```bash
terraform fmt -check -recursive
terraform validate
tflint --recursive
```

### 2. Unit Tests (Terratest)
```go
// Test compute facade
func TestComputeFacadeAWS(t *testing.T) {
    terraformOptions := &terraform.Options{
        TerraformDir: "../facade/compute",
        Vars: map[string]interface{}{
            "provider":      "aws",
            "instance_size": "medium",
        },
    }
    
    defer terraform.Destroy(t, terraformOptions)
    terraform.InitAndApply(t, terraformOptions)
    
    instanceID := terraform.Output(t, terraformOptions, "instance_id")
    assert.NotEmpty(t, instanceID)
}
```

### 3. Integration Tests
Test cross-layer interactions and provider switching.

### 4. Contract Tests
Validate API schemas and output formats.

## Benefits

1. **Modularity** - Clear layer boundaries
2. **Testability** - Each layer independently testable
3. **Portability** - Easy provider switching
4. **Consistency** - Standardized patterns
5. **Maintainability** - Changes isolated to layers
6. **Extensibility** - Add providers without core changes
