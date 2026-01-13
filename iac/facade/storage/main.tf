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
      Provider     = var.provider
      Project      = var.project_name
      Architecture = "SEA"
    }
  )
}

# ============================================================================
# CORE ORCHESTRATION
# ============================================================================

module "storage_core" {
  source = "../../core/storage"

  # API contract inputs
  bucket_name          = var.bucket_name
  provider             = var.provider
  storage_class        = var.storage_class
  versioning_enabled   = var.versioning_enabled
  encryption_enabled   = var.encryption_enabled
  encryption_key_id    = var.encryption_key_id
  public_access_block  = var.public_access_block
  
  # Logging
  enable_logging       = var.enable_logging
  log_bucket_name      = var.log_bucket_name
  
  # Rules
  cors_rules           = var.cors_rules
  lifecycle_rules      = var.lifecycle_rules
  
  # Replication
  replication_enabled     = var.replication_enabled
  replication_destination = var.replication_destination
  
  # Tags
  bucket_tags          = var.bucket_tags
  provider_config      = var.provider_config
  
  # From common layer
  storage_class_mapping = local.storage_class_mapping
  common_tags           = local.common_tags
}

# ============================================================================
# OUTPUTS (User-facing, simplified)
# ============================================================================

output "bucket" {
  description = "Complete bucket details"
  value = {
    # Identification
    id     = module.storage_core.bucket_id
    arn    = module.storage_core.bucket_arn
    name   = var.bucket_name
    
    # Access
    url    = module.storage_core.bucket_url
    region = module.storage_core.bucket_region
    
    # Configuration
    storage_class       = module.storage_core.storage_class
    versioning_enabled  = module.storage_core.versioning_enabled
    encryption_enabled  = module.storage_core.encryption_enabled
    
    # Provider
    provider = var.provider
    
    # Metadata
    tags = module.storage_core.tags
  }
}

# Convenience outputs
output "bucket_id" {
  description = "Bucket ID for reference in other resources"
  value       = module.storage_core.bucket_id
}

output "bucket_url" {
  description = "Bucket access URL"
  value       = module.storage_core.bucket_url
}

output "bucket_arn" {
  description = "Bucket ARN/Resource ID"
  value       = module.storage_core.bucket_arn
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
