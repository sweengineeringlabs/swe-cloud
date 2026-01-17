# Secrets Facade

locals {
  common_tags = merge(
    var.tags,
    {
      ManagedBy   = "Terraform"
      Environment = var.environment
      Project     = var.project_name
      Module      = "Secrets-Facade"
    }
  )
}

# AWS: Secrets Manager
module "aws_secrets" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../aws/core/secrets"

  name          = var.name
  description   = var.description
  secret_string = var.secret_string
  
  tags = local.common_tags
}

# Azure: Key Vault Secret
module "azure_secrets" {
  count  = var.provider_name == "azure" ? 1 : 0
  source = "../../azure/core/secrets"

  name         = var.name
  secret_value = var.secret_string
  key_vault_id = "/subscriptions/sub/resourceGroups/rg/providers/Microsoft.KeyVault/vaults/vault" # Placeholder
  tags         = local.common_tags
}

# GCP: Secret Manager
module "gcp_secrets" {
  count  = var.provider_name == "gcp" ? 1 : 0
  source = "../../gcp/core/secrets"

  project_id  = var.project_name
  secret_id   = var.name
  secret_data = var.secret_string
  labels      = local.common_tags
}

output "secret_id" {
  value = (
    var.provider_name == "aws" ? (length(module.aws_secrets) > 0 ? module.aws_secrets[0].secret_id : null) :
    var.provider_name == "azure" ? (length(module.azure_secrets) > 0 ? module.azure_secrets[0].secret_id : null) :
    var.provider_name == "gcp" ? (length(module.gcp_secrets) > 0 ? module.gcp_secrets[0].secret_id : null) :
    null
  )
}

output "secret_arn" {
  value = (
    var.provider_name == "aws" ? (length(module.aws_secrets) > 0 ? module.aws_secrets[0].secret_arn : null) :
    var.provider_name == "azure" ? (length(module.azure_secrets) > 0 ? module.azure_secrets[0].secret_id : null) :
    var.provider_name == "gcp" ? (length(module.gcp_secrets) > 0 ? module.gcp_secrets[0].secret_name : null) :
    null
  )
}
