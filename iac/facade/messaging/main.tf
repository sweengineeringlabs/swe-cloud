# Messaging Facade
# Unified interface for Queue and Topic resources across providers

terraform {
  required_version = ">= 1.0"
}

locals {
  common_tags = merge(
    var.tags,
    {
      ManagedBy    = "Terraform"
      Environment  = var.environment
      Provider     = var.provider_name
      Project      = var.project_name
      Module       = "Messaging-Facade"
    }
  )
}

# AWS: SQS or SNS
module "aws_messaging" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../iac_core/aws/src/messaging"
  
  create_queue = var.type == "queue"
  queue_name   = var.name
  
  create_topic = var.type == "topic"
  topic_name   = var.name
  
  tags = local.common_tags
}

# Azure and GCP would be similar if core modules existed.
# For now we route to AWS or provide placeholders.

output "resource_arn" {
  value = var.provider_name == "aws" ? (var.type == "queue" ? module.aws_messaging[0].queue_arn : module.aws_messaging[0].topic_arn) : "placeholder-arn"
}

output "resource_name" {
  value = var.name
}

output "resource_url" {
  value = var.provider_name == "aws" && var.type == "queue" ? module.aws_messaging[0].queue_id : null
}

