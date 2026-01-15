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

output "event_resource_id" {
  value = var.provider_name == "aws" ? (length(module.aws_events) > 0 ? module.aws_events[0].event_resource_id : null) : null
}

output "event_resource_arn" {
  value = var.provider_name == "aws" ? (length(module.aws_events) > 0 ? module.aws_events[0].event_resource_arn : null) : null
}
