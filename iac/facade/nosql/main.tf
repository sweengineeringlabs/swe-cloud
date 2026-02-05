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
  source = "../../aws/core/nosql"

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

# Azure: CosmosDB
module "azure_nosql" {
  count  = var.provider_name == "azure" ? 1 : 0
  source = "../../azure/core/nosql"

  account_name        = replace(lower(var.table_name), "-", "")
  resource_group_name = "${var.project_name}-${var.environment}-rg"
  location            = "East US"
  container_name      = var.table_name
  partition_key_path  = "/${var.hash_key}"

  tags = local.common_tags
}

# GCP: Firestore
module "gcp_nosql" {
  count  = var.provider_name == "gcp" ? 1 : 0
  source = "../../gcp/core/nosql"

  project_id  = var.project_name
  database_id = var.table_name
  location_id = "us-east1"
}

# ZeroCloud: ZeroDB
module "zero_nosql" {
  count  = var.provider_name == "zero" ? 1 : 0
  source = "../../zero/core/nosql"

  table_name    = var.table_name
  hash_key      = var.hash_key
  hash_key_type = var.hash_key_type
  range_key     = var.range_key
  range_key_type = var.range_key_type

  tags = local.common_tags
}

output "table_id" {
  value = (
    var.provider_name == "aws" ? (length(module.aws_nosql) > 0 ? module.aws_nosql[0].table_id : null) :
    var.provider_name == "azure" ? (length(module.azure_nosql) > 0 ? module.azure_nosql[0].account_id : null) :
    var.provider_name == "gcp" ? (length(module.gcp_nosql) > 0 ? module.gcp_nosql[0].database_id : null) :
    var.provider_name == "zero" ? (length(module.zero_nosql) > 0 ? module.zero_nosql[0].table_id : null) :
    null
  )
}

output "table_arn" {
  value = (
    var.provider_name == "aws" ? (length(module.aws_nosql) > 0 ? module.aws_nosql[0].table_arn : null) :
    var.provider_name == "azure" ? (length(module.azure_nosql) > 0 ? module.azure_nosql[0].endpoint : null) :
    var.provider_name == "gcp" ? (length(module.gcp_nosql) > 0 ? module.gcp_nosql[0].database_id : null) :
    var.provider_name == "zero" ? (length(module.zero_nosql) > 0 ? module.zero_nosql[0].table_arn : null) :
    null
  )
}
