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
  source = "../../iac_core/aws/src/encryption"

  name        = var.name
  description = var.description
  
  tags = local.common_tags
}

output "key_id" {
  value = var.provider_name == "aws" ? (length(module.aws_kms) > 0 ? module.aws_kms[0].key_id : null) : null
}

output "key_arn" {
  value = var.provider_name == "aws" ? (length(module.aws_kms) > 0 ? module.aws_kms[0].key_arn : null) : null
}
