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
  source = "../../aws/core/messaging"
  
  create_queue = var.type == "queue"
  queue_name   = var.name
  
  create_topic = var.type == "topic"
  topic_name   = var.name
  
  tags = local.common_tags
}

# Azure: Service Bus
module "azure_messaging" {
  count  = var.provider_name == "azure" ? 1 : 0
  source = "../../azure/core/messaging"
  
  create_queue = var.type == "queue"
  queue_name   = var.name
  
  create_topic = var.type == "topic"
  topic_name   = var.name
  
  tags = local.common_tags
}

# GCP: Pub/Sub
module "gcp_messaging" {
  count  = var.provider_name == "gcp" ? 1 : 0
  source = "../../gcp/core/messaging"
  
  create_queue = var.type == "queue"
  queue_name   = var.name
  
  create_topic = var.type == "topic"
  topic_name   = var.name
  
  tags = local.common_tags
}

# ZeroCloud: ZeroQueue
module "zero_messaging" {
  count  = var.provider_name == "zero" ? 1 : 0
  source = "../../zero/core/messaging"
  
  create_queue = var.type == "queue"
  queue_name   = var.name
  
  create_topic = var.type == "topic"
  topic_name   = var.name
  
  tags = local.common_tags
}

output "resource_arn" {
  value = (
    var.provider_name == "aws" ? (var.type == "queue" ? module.aws_messaging[0].queue_arn : module.aws_messaging[0].topic_arn) : 
    var.provider_name == "azure" ? "azure-arn-placeholder" :
    var.provider_name == "gcp" ? "gcp-id-placeholder" :
    var.provider_name == "zero" ? (var.type == "queue" ? module.zero_messaging[0].queue_arn : module.zero_messaging[0].topic_arn) :
    null
  )
}

output "resource_url" {
  value = (
    var.provider_name == "aws" && var.type == "queue" ? module.aws_messaging[0].queue_id :
    var.provider_name == "zero" && var.type == "queue" ? module.zero_messaging[0].queue_id :
    null
  )
}

