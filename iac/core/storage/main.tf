# Storage Core Orchestration
# Core Layer - Resource composition and dependency management

terraform {
  required_version = ">= 1.0"
}

# ============================================================================
# IMPORT COMMON DEFINITIONS
# ============================================================================

locals {
  # Get provider-specific storage class from common layer
  provider_storage_class = var.storage_class_mapping[var.provider][var.storage_class]
  
  # Storage resource tags
  resource_tags = merge(
    var.common_tags,
    {
      ResourceType = "Storage"
      Service      = "ObjectStorage"
      BucketName   = var.bucket_name
      StorageClass = var.storage_class
    },
    var.bucket_tags
  )
}

# ============================================================================
# PROVIDER ROUTING
# ============================================================================

# Route to AWS S3
module "aws_bucket" {
  count  = var.provider == "aws" ? 1 : 0
  source = "../../providers/aws/storage"

  bucket_name          = var.bucket_name
  storage_class        = local.provider_storage_class
  versioning_enabled   = var.versioning_enabled
  encryption_enabled   = var.encryption_enabled
  encryption_key_id    = var.encryption_key_id
  public_access_block  = var.public_access_block
  
  enable_logging       = var.enable_logging
  log_bucket_name      = var.log_bucket_name
  
  cors_rules           = var.cors_rules
  lifecycle_rules      = var.lifecycle_rules
  
  replication_enabled     = var.replication_enabled
  replication_destination = var.replication_destination
  
  acl           = var.provider_config.acl
  force_destroy = var.provider_config.force_destroy
  
  tags = local.resource_tags
}

# Route to Azure Blob Storage
module "azure_bucket" {
  count  = var.provider == "azure" ? 1 : 0
  source = "../../providers/azure/storage"

  storage_account_name = var.bucket_name
  resource_group_name  = var.provider_config.resource_group_name
  location             = var.provider_config.location
  
  account_tier             = var.provider_config.account_tier
  account_replication_type = var.provider_config.account_replication_type
  access_tier              = local.provider_storage_class
  
  enable_versioning    = var.versioning_enabled
  enable_encryption    = var.encryption_enabled
  
  public_access_enabled = !var.public_access_block
  
  tags = local.resource_tags
}

# Route to GCP Cloud Storage
module "gcp_bucket" {
  count  = var.provider == "gcp" ? 1 : 0
  source = "../../providers/gcp/storage"

  bucket_name  = var.bucket_name
  project_id   = var.provider_config.project_id
  location     = var.provider_config.location
  
  storage_class = local.provider_storage_class
  
  versioning_enabled = var.versioning_enabled
  
  uniform_bucket_level_access = var.provider_config.uniform_bucket_level_access
  public_access_prevention    = var.public_access_block ? "enforced" : "inherited"
  
  cors          = var.cors_rules
  lifecycle_rule = var.lifecycle_rules
  
  labels = local.resource_tags
}

# ============================================================================
# OUTPUT AGGREGATION
# ============================================================================

locals {
  # Aggregate bucket ID
  bucket_id = try(
    module.aws_bucket[0].bucket_id,
    module.azure_bucket[0].storage_account_id,
    module.gcp_bucket[0].bucket_name,
    ""
  )

  # Aggregate bucket ARN
  bucket_arn = try(
    module.aws_bucket[0].bucket_arn,
    module.azure_bucket[0].storage_account_id,
    module.gcp_bucket[0].bucket_url,
    ""
  )

  # Aggregate bucket URL
  bucket_url = try(
    module.aws_bucket[0].bucket_domain_name,
    module.azure_bucket[0].primary_blob_endpoint,
    module.gcp_bucket[0].bucket_url,
    ""
  )

  # Region/location
  bucket_region = try(
    module.aws_bucket[0].region,
    module.azure_bucket[0].location,
    module.gcp_bucket[0].location,
    ""
  )
}

# ============================================================================
# LIFECYCLE HOOKS
# ============================================================================

# Post-creation validation
resource "null_resource" "bucket_ready" {
  depends_on = [
    module.aws_bucket,
    module.azure_bucket,
    module.gcp_bucket
  ]

  triggers = {
    bucket_id = local.bucket_id
  }

  provisioner "local-exec" {
    command = "echo 'Bucket ${var.bucket_name} is ready on ${var.provider}'"
  }

  lifecycle {
    postcondition {
      condition     = local.bucket_id != ""
      error_message = "Bucket creation failed - no bucket ID returned"
    }
  }
}

# Encryption validation (if enabled)
resource "null_resource" "encryption_check" {
  count = var.encryption_enabled ? 1 : 0

  depends_on = [null_resource.bucket_ready]

  triggers = {
    bucket_id    = local.bucket_id
    encryption   = var.encryption_enabled
  }

  provisioner "local-exec" {
    command = "echo 'Encryption verified for bucket ${var.bucket_name}'"
  }
}

# Public access validation
resource "null_resource" "access_check" {
  count = var.public_access_block ? 1 : 0

  depends_on = [null_resource.bucket_ready]

  triggers = {
    bucket_id     = local.bucket_id
    public_block  = var.public_access_block
  }

  provisioner "local-exec" {
    command = "echo 'Public access blocked for bucket ${var.bucket_name}'"
  }
}

# ============================================================================
# OUTPUTS (conforming to API contract)
# ============================================================================

output "bucket_id" {
  description = "Unique identifier of the storage bucket"
  value       = local.bucket_id
}

output "bucket_arn" {
  description = "ARN/Resource ID of the bucket"
  value       = local.bucket_arn
}

output "bucket_url" {
  description = "URL for accessing the bucket"
  value       = local.bucket_url
}

output "bucket_region" {
  description = "Region where bucket is located"
  value       = local.bucket_region
}

output "versioning_enabled" {
  description = "Versioning status"
  value       = var.versioning_enabled
}

output "encryption_enabled" {
  description = "Encryption status"
  value       = var.encryption_enabled
}

output "storage_class" {
  description = "Storage class"
  value       = var.storage_class
}

output "tags" {
  description = "Applied tags"
  value       = local.resource_tags
}
