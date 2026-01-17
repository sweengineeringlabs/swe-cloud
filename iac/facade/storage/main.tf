# Storage Facade
# Facade Layer - Public interface for storage resources

terraform {
  required_version = ">= 1.0"
}

# ============================================================================
# IMPORT COMMON LAYER
# ============================================================================

locals {
  # Import storage class mappings
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
    oracle = {
      standard   = "Standard"
      infrequent = "InfrequentAccess"
      archive    = "Archive"
      cold       = "Archive"
    }
  }

  # Build common tags
  common_tags = merge(
    var.tags,
    {
      ManagedBy    = "Terraform"
      Environment  = var.environment
      Provider     = var.provider_name
      Project      = var.project_name
      Architecture = "SEA"
    }
  )
}

# ============================================================================
# PROVIDER-SPECIFIC MODULE ROUTING
# ============================================================================

# Route to AWS storage module
module "aws_storage" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../aws/core/storage"
  
  bucket_name         = var.bucket_name
  versioning_enabled  = var.versioning_enabled
  encryption_enabled  = var.encryption_enabled
  encryption_key_id   = var.encryption_key_id
  public_access_block = var.public_access_block
  tags                = local.common_tags
}

# Route to Azure storage module  
module "azure_storage" {
  count  = var.provider_name == "azure" ? 1 : 0
  source = "../../azure/core/storage"
  
  storage_account_name = replace(lower(var.bucket_name), "-", "") # Azure requires alphanumeric
  resource_group_name  = "${var.project_name}-${var.environment}-rg"
  location             = "East US"
  versioning_enabled   = var.versioning_enabled
  block_public_access  = var.public_access_block
  create_container     = true
  container_name       = var.bucket_name
  tags                 = local.common_tags
}

# Route to GCP storage module
module "gcp_storage" {
  count  = var.provider_name == "gcp" ? 1 : 0
  source = "../../gcp/core/storage"
  
  bucket_name         = var.bucket_name
  versioning_enabled  = var.versioning_enabled
  project_id          = var.project_name
  location            = "US"
  tags                = local.common_tags
}

# Route to ZeroCloud storage module  
module "zero_storage" {
  count  = var.provider_name == "zero" ? 1 : 0
  source = "../../zero/core/storage"
  
  bucket_name         = var.bucket_name
  versioning_enabled  = var.versioning_enabled
  tags                = local.common_tags
}

# Aggregated outputs (select based on provider)
locals {
  bucket_id = (
    var.provider_name == "aws" ? (length(module.aws_storage) > 0 ? module.aws_storage[0].bucket_id : null) :
    var.provider_name == "azure" ? (length(module.azure_storage) > 0 ? module.azure_storage[0].storage_account_id : null) :
    var.provider_name == "gcp" ? (length(module.gcp_storage) > 0 ? module.gcp_storage[0].bucket_id : null) :
    var.provider_name == "zero" ? (length(module.zero_storage) > 0 ? module.zero_storage[0].bucket_id : null) :
    null
  )
  
  bucket_arn = (
    var.provider_name == "aws" ? (length(module.aws_storage) > 0 ? module.aws_storage[0].bucket_arn : null) :
    var.provider_name == "azure" ? (length(module.azure_storage) > 0 ? module.azure_storage[0].storage_account_name : null) :
    var.provider_name == "gcp" ? (length(module.gcp_storage) > 0 ? module.gcp_storage[0].bucket_url : null) :
    var.provider_name == "zero" ? (length(module.zero_storage) > 0 ? module.zero_storage[0].bucket_arn : null) :
    null
  )
  
  bucket_url = (
    var.provider_name == "aws" ? (length(module.aws_storage) > 0 ? module.aws_storage[0].bucket_domain_name : null) :
    var.provider_name == "azure" ? (length(module.azure_storage) > 0 ? module.azure_storage[0].primary_blob_endpoint : null) :
    var.provider_name == "gcp" ? (length(module.gcp_storage) > 0 ? module.gcp_storage[0].bucket_url : null) :
    var.provider_name == "zero" ? (length(module.zero_storage) > 0 ? module.zero_storage[0].bucket_url : null) :
    null
  )
  
  bucket_region = (
    var.provider_name == "aws" ? (length(module.aws_storage) > 0 ? module.aws_storage[0].region : null) :
    var.provider_name == "azure" ? "East US" :
    var.provider_name == "gcp" ? "US" :
    var.provider_name == "zero" ? "local" :
    null
  )
}

# ============================================================================
# OUTPUTS (User-facing, simplified)
# ============================================================================

output "bucket" {
  description = "Complete bucket details"
  value = {
    # Identification
    id   = local.bucket_id
    arn  = local.bucket_arn
    name = var.bucket_name
    
    # Access
    url    = local.bucket_url
    region = local.bucket_region
    
    # Configuration
    storage_class      = var.storage_class
    versioning_enabled = var.versioning_enabled
    encryption_enabled = var.encryption_enabled
    
    # Provider
    provider = var.provider_name
    
    # Metadata
    tags = local.common_tags
  }
}

# Convenience outputs
output "bucket_id" {
  description = "Bucket ID for reference in other resources"
  value       = local.bucket_id
}

output "bucket_url" {
  description = "Bucket access URL"
  value       = local.bucket_url
}

output "bucket_arn" {
  description = "Bucket ARN/Resource ID"
  value       = local.bucket_arn
}

# ============================================================================
# USAGE EXAMPLE (in comments for reference)
# ============================================================================

/*
Example usage:

module "data_bucket" {
  source = "./facade/storage"
  
  # Required
  provider    = "aws"
  bucket_name = "my-data-bucket-prod"
  
  # Project info
  project_name = "my-project"
  environment  = "prod"
  
  # Optional
  storage_class       = "standard"
  versioning_enabled  = true
  encryption_enabled  = true
  public_access_block = true
  
  # Lifecycle
  lifecycle_rules = [{
    id      = "archive-old-data"
    enabled = true
    transition = [{
      days          = 90
      storage_class = "archive"
    }]
    expiration = {
      days = 365
    }
  }]
}

# Access outputs
output "bucket_url" {
  value = module.data_bucket.bucket_url
}
*/
