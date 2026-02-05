# Encryption Facade

locals {
  common_tags = merge(
    var.tags,
    {
      ManagedBy   = "Terraform"
      Environment = var.environment
      Project     = var.project_name
      Module      = "Encryption-Facade"
    }
  )
}

# AWS: KMS
module "aws_kms" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../aws/core/encryption"

  name        = var.name
  description = var.description
  
  tags = local.common_tags
}

# Azure: Key Vault Key
module "azure_kms" {
  count  = var.provider_name == "azure" ? 1 : 0
  source = "../../azure/core/encryption"

  name         = var.name
  key_vault_id = "/subscriptions/sub/resourceGroups/rg/providers/Microsoft.KeyVault/vaults/vault" # Placeholder
  tags         = local.common_tags
}

# GCP: KMS
module "gcp_kms" {
  count  = var.provider_name == "gcp" ? 1 : 0
  source = "../../gcp/core/encryption"

  project_id    = var.project_name
  key_ring_name = "${var.project_name}-${var.environment}-keyring"
  key_name      = var.name
  location      = "global"
  labels        = local.common_tags
}

output "key_id" {
  value = (
    var.provider_name == "aws" ? (length(module.aws_kms) > 0 ? module.aws_kms[0].key_id : null) :
    var.provider_name == "azure" ? (length(module.azure_kms) > 0 ? module.azure_kms[0].key_id : null) :
    var.provider_name == "gcp" ? (length(module.gcp_kms) > 0 ? module.gcp_kms[0].key_id : null) :
    null
  )
}

output "key_arn" {
  value = (
    var.provider_name == "aws" ? (length(module.aws_kms) > 0 ? module.aws_kms[0].key_arn : null) :
    var.provider_name == "azure" ? (length(module.azure_kms) > 0 ? module.azure_kms[0].key_id : null) :
    var.provider_name == "gcp" ? (length(module.gcp_kms) > 0 ? module.gcp_kms[0].key_name : null) :
    null
  )
}
