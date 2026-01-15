# Events Facade

locals {
  common_tags = merge(
    var.tags,
    {
      ManagedBy   = "Terraform"
      Environment = var.environment
      Project     = var.project_name
      Module      = "Events-Facade"
    }
  )
}

# AWS: EventBridge
module "aws_events" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../iac_core/aws/src/events"

  name = var.name
  tags = local.common_tags
}

# Azure: Event Grid
module "azure_events" {
  count  = var.provider_name == "azure" ? 1 : 0
  source = "../../iac_core/azure/src/events"

  name                = var.name
  resource_group_name = "${var.project_name}-${var.environment}-rg"
  location            = "East US"
  tags                = local.common_tags
}

# GCP: PubSub
module "gcp_events" {
  count  = var.provider_name == "gcp" ? 1 : 0
  source = "../../iac_core/gcp/src/events"

  project_id = var.project_name
  topic_name = var.name
  labels     = local.common_tags
}

output "event_resource_id" {
  value = (
    var.provider_name == "aws" ? (length(module.aws_events) > 0 ? module.aws_events[0].event_resource_id : null) :
    var.provider_name == "azure" ? (length(module.azure_events) > 0 ? module.azure_events[0].topic_id : null) :
    var.provider_name == "gcp" ? (length(module.gcp_events) > 0 ? module.gcp_events[0].topic_id : null) :
    null
  )
}

output "event_resource_arn" {
  value = (
    var.provider_name == "aws" ? (length(module.aws_events) > 0 ? module.aws_events[0].event_resource_arn : null) :
    var.provider_name == "azure" ? (length(module.azure_events) > 0 ? module.azure_events[0].endpoint : null) :
    var.provider_name == "gcp" ? (length(module.gcp_events) > 0 ? module.gcp_events[0].topic_name : null) :
    null
  )
}
