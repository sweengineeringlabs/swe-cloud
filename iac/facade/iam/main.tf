# IAM Facade
# Unified interface for Identity resources across providers

terraform {
  required_version = ">= 1.0"
}

# ============================================================================
# IMPORT COMMON LAYER
# ============================================================================

locals {
  common_tags = merge(
    var.tags,
    {
      ManagedBy    = "Terraform"
      Environment  = var.environment
      Provider     = var.provider
      Project      = var.project_name
      Module       = "IAM-Facade"
    }
  )
}

# ============================================================================
# PROVIDER-SPECIFIC MODULE ROUTING
# ============================================================================

# AWS: IAM Role or User
module "aws_iam" {
  count  = var.provider == "aws" ? 1 : 0
  source = "../../iac_core/aws/src/iam"
  
  # Map 'role' -> IAM Role, 'user'/'service_agent' -> IAM User
  create_role = var.identity_type == "role"
  role_name   = var.identity_name
  
  create_user = contains(["user", "service_agent"], var.identity_type)
  user_name   = var.identity_name
  
  # Trust Policy (Principals)
  trusted_services = var.principals
  
  tags = local.common_tags
}

# Azure: User Assigned Managed Identity
module "azure_iam" {
  count  = var.provider == "azure" ? 1 : 0
  source = "../../iac_core/azure/src/iam"
  
  # For Azure, we map 'service_agent'/'user' to Managed Identity
  create_identity     = contains(["user", "service_agent"], var.identity_type)
  identity_name       = var.identity_name
  resource_group_name = try(var.provider_config.resource_group_name, "default-rg")
  location            = try(var.provider_config.location, "eastus")
  
  tags = local.common_tags
}

# GCP: Service Account
module "gcp_iam" {
  count  = var.provider == "gcp" ? 1 : 0
  source = "../../iac_core/gcp/src/iam"
  
  # For GCP, we map 'service_agent'/'user' to Service Account
  create_service_account = contains(["user", "service_agent"], var.identity_type)
  account_id             = var.identity_name
  display_name           = var.identity_name
  project_id             = try(var.provider_config.project_id, null)
}

# ============================================================================
# AGGREGATED OUTPUTS
# ============================================================================

locals {
  identity_id = (
    var.provider == "aws"   ? (length(module.aws_iam) > 0 ? (var.identity_type == "role" ? module.aws_iam[0].role_id : module.aws_iam[0].user_name) : null) :
    var.provider == "azure" ? (length(module.azure_iam) > 0 ? module.azure_iam[0].identity_id : null) :
    var.provider == "gcp"   ? (length(module.gcp_iam) > 0 ? module.gcp_iam[0].service_account_email : null) :
    null
  )
  
  principal_id = (
    var.provider == "aws"   ? (length(module.aws_iam) > 0 ? (var.identity_type == "role" ? module.aws_iam[0].role_arn : module.aws_iam[0].user_arn) : null) :
    var.provider == "azure" ? (length(module.azure_iam) > 0 ? module.azure_iam[0].identity_principal_id : null) :
    var.provider == "gcp"   ? (length(module.gcp_iam) > 0 ? module.gcp_iam[0].service_account_email : null) :
    null
  )
}
