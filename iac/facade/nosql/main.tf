# NoSQL Facade (Unified Interface)

locals {
  common_tags = merge(
    var.tags,
    {
      ManagedBy   = "Terraform"
      Environment = var.environment
      Project     = var.project_name
      Module      = "NoSQL-Facade"
    }
  )
}

# AWS: DynamoDB
module "aws_nosql" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../iac_core/aws/src/nosql"

  table_name    = var.table_name
  hash_key      = var.hash_key
  hash_key_type = var.hash_key_type
  range_key     = var.range_key
  range_key_type = var.range_key_type
  
  billing_mode  = "PAY_PER_REQUEST"
  read_capacity = 0
  write_capacity = 0

  tags = local.common_tags
}

# Azure: CosmosDB (Stub)
# module "azure_nosql" { ... }

# GCP: Firestore (Stub)
# module "gcp_nosql" { ... }

output "table_id" {
  value = var.provider_name == "aws" ? (length(module.aws_nosql) > 0 ? module.aws_nosql[0].table_id : null) : null
}

output "table_arn" {
  value = var.provider_name == "aws" ? (length(module.aws_nosql) > 0 ? module.aws_nosql[0].table_arn : null) : null
}
