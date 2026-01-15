# Workflows Facade

locals {
  common_tags = merge(
    var.tags,
    {
      ManagedBy   = "Terraform"
      Environment = var.environment
      Project     = var.project_name
      Module      = "Workflows-Facade"
    }
  )
}

# AWS: Step Functions
module "aws_workflows" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../iac_core/aws/src/workflows"

  name       = var.name
  definition = var.definition
  role_arn   = var.role_arn
  
  tags = local.common_tags
}

output "workflow_id" {
  value = var.provider_name == "aws" ? (length(module.aws_workflows) > 0 ? module.aws_workflows[0].workflow_id : null) : null
}

output "workflow_arn" {
  value = var.provider_name == "aws" ? (length(module.aws_workflows) > 0 ? module.aws_workflows[0].workflow_arn : null) : null
}
