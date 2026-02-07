# IAM Facade
# Unified interface for Identity resources across providers

terraform {
  required_version = ">= 1.0"
}

# ============================================================================
# IMPORT COMMON LAYER
# ============================================================================

  common_tags = merge(
    var.tags,
    {
      ManagedBy    = "Terraform"
      Environment  = var.environment
      Provider     = var.provider_name
      Project      = var.project_name
      Module       = "IAM-Facade"
    }
  )

  # Unified Capability Mapping
  # Maps abstract roles (e.g. "storage_read") to provider-specific policies/ARNs
  capability_map = {
    aws = {
      storage_read   = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess"
      storage_write  = "arn:aws:iam::aws:policy/AmazonS3FullAccess"
      nosql_read     = "arn:aws:iam::aws:policy/AmazonDynamoDBReadOnlyAccess"
      nosql_write    = "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess"
      compute_admin  = "arn:aws:iam::aws:policy/AmazonEC2FullAccess"
      admin          = "arn:aws:iam::aws:policy/AdministratorAccess"
    }
    zero = {
      # ZeroCloud reuses AWS policies (mocked in control plane)
      storage_read   = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess"
      storage_write  = "arn:aws:iam::aws:policy/AmazonS3FullAccess"
      nosql_read     = "arn:aws:iam::aws:policy/AmazonDynamoDBReadOnlyAccess"
      nosql_write    = "arn:aws:iam::aws:policy/AmazonDynamoDBFullAccess"
      compute_admin  = "arn:aws:iam::aws:policy/AmazonEC2FullAccess"
      admin          = "arn:aws:iam::aws:policy/AdministratorAccess"
    }
    azure = {
      storage_read   = "Storage Blob Data Reader"
      storage_write  = "Storage Blob Data Contributor"
      nosql_read     = "Cosmos DB Account Reader Role"
      nosql_write    = "Cosmos DB Account Contributor" # Approximate
      compute_admin  = "Virtual Machine Contributor"
      admin          = "Owner"
    }
    gcp = {
      storage_read   = "roles/storage.objectViewer"
      storage_write  = "roles/storage.objectAdmin"
      nosql_read     = "roles/datastore.viewer"
      nosql_write    = "roles/datastore.user"
      compute_admin  = "roles/compute.admin"
      admin          = "roles/owner"
    }
  }

  # Resolution Logic
  # We gracefully handle missing capabilities by ignoring them or using a fallback if needed
  selected_roles = [
    for r in var.roles :
    lookup(local.capability_map[var.provider_name], r, null)
  ]
  
  # Remove nulls (unsupported roles for a provider)
  final_roles = [for r in local.selected_roles : r if r != null]
}

# ============================================================================
# PROVIDER-SPECIFIC MODULE ROUTING
# ============================================================================

# AWS: IAM Role or User
module "aws_iam" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../aws/core/iam"
  
  # Map 'role' -> IAM Role, 'user'/'service_agent' -> IAM User
  create_role = var.identity_type == "role"
  role_name   = var.identity_name
  
  create_user = contains(["user", "service_agent"], var.identity_type)
  user_name   = var.identity_name
  
  # Trust Policy (Principals)
  trusted_services = var.principals
  
  # Policy Attachment
  managed_policy_arns = local.final_roles
  
  tags = local.common_tags
}

# Azure: User Assigned Managed Identity
module "azure_iam" {
  count  = var.provider_name == "azure" ? 1 : 0
  source = "../../azure/core/iam"
  
  # For Azure, we map 'service_agent'/'user' to Managed Identity
  create_identity     = contains(["user", "service_agent"], var.identity_type)
  identity_name       = var.identity_name
  resource_group_name = try(var.provider_config.resource_group_name, "default-rg")
  location            = try(var.provider_config.location, "eastus")
  
  tags = local.common_tags
}

# GCP: Service Account
module "gcp_iam" {
  count  = var.provider_name == "gcp" ? 1 : 0
  source = "../../gcp/core/iam"
  
  # For GCP, we map 'service_agent'/'user' to Service Account
  create_service_account = contains(["user", "service_agent"], var.identity_type)
  account_id             = var.identity_name
  display_name           = var.identity_name
  project_id             = try(var.provider_config.project_id, null)
}

# ZeroCloud: ZeroID
module "zero_iam" {
  count  = var.provider_name == "zero" ? 1 : 0
  source = "../../zero/core/iam"
  
  create_role = var.identity_type == "role"
  role_name   = var.identity_name
  
  create_user = contains(["user", "service_agent"], var.identity_type)
  user_name   = var.identity_name
  
  trusted_services = var.principals
  
  managed_policy_arns = local.final_roles
  
  tags = local.common_tags
}

# ============================================================================
# AGGREGATED OUTPUTS
# ============================================================================

locals {
  identity_id = (
    var.provider_name == "aws"   ? (length(module.aws_iam) > 0 ? (var.identity_type == "role" ? module.aws_iam[0].role_id : module.aws_iam[0].user_name) : null) :
    var.provider_name == "azure" ? (length(module.azure_iam) > 0 ? module.azure_iam[0].identity_id : null) :
    var.provider_name == "gcp"   ? (length(module.gcp_iam) > 0 ? module.gcp_iam[0].service_account_email : null) :
    var.provider_name == "zero"  ? (length(module.zero_iam) > 0 ? (var.identity_type == "role" ? module.zero_iam[0].role_id : module.zero_iam[0].user_name) : null) :
    null
  )
  
  principal_id = (
    var.provider_name == "aws"   ? (length(module.aws_iam) > 0 ? (var.identity_type == "role" ? module.aws_iam[0].role_arn : module.aws_iam[0].user_arn) : null) :
    var.provider_name == "azure" ? (length(module.azure_iam) > 0 ? module.azure_iam[0].identity_principal_id : null) :
    var.provider_name == "gcp"   ? (length(module.gcp_iam) > 0 ? module.gcp_iam[0].service_account_email : null) :
    var.provider_name == "zero"  ? (length(module.zero_iam) > 0 ? (var.identity_type == "role" ? module.zero_iam[0].role_arn : module.zero_iam[0].user_arn) : null) :
    null
  )
}
