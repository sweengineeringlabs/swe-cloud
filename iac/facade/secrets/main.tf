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
  source = "../../iac_core/aws/src/secrets"

  name          = var.name
  description   = var.description
  secret_string = var.secret_string
  
  tags = local.common_tags
}

# Azure: Key Vault Secret (Stub)
# module "azure_secrets" { ... }

output "secret_id" {
  value = var.provider_name == "aws" ? (length(module.aws_secrets) > 0 ? module.aws_secrets[0].secret_id : null) : null
}

output "secret_arn" {
  value = var.provider_name == "aws" ? (length(module.aws_secrets) > 0 ? module.aws_secrets[0].secret_arn : null) : null
}
