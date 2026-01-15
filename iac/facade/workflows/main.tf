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

# Azure: Logic App
module "azure_workflows" {
  count  = var.provider_name == "azure" ? 1 : 0
  source = "../../iac_core/azure/src/workflows"

  name                = var.name
  resource_group_name = "${var.project_name}-${var.environment}-rg"
  location            = "East US"
  workflow_definition = var.definition # CAUTION: Azure expects JSON, not ASL
  
  tags = local.common_tags
}

# GCP: Workflows
module "gcp_workflows" {
  count  = var.provider_name == "gcp" ? 1 : 0
  source = "../../iac_core/gcp/src/workflows"

  project_id      = var.project_name
  name            = var.name
  region          = "us-central1"
  source_contents = var.definition # CAUTION: GCP expects YAML
  
  labels = local.common_tags
}

output "workflow_id" {
  value = (
    var.provider_name == "aws" ? (length(module.aws_workflows) > 0 ? module.aws_workflows[0].workflow_id : null) :
    var.provider_name == "azure" ? (length(module.azure_workflows) > 0 ? module.azure_workflows[0].workflow_id : null) :
    var.provider_name == "gcp" ? (length(module.gcp_workflows) > 0 ? module.gcp_workflows[0].workflow_id : null) :
    null
  )
}

output "workflow_arn" {
  value = (
    var.provider_name == "aws" ? (length(module.aws_workflows) > 0 ? module.aws_workflows[0].workflow_arn : null) :
    var.provider_name == "azure" ? (length(module.azure_workflows) > 0 ? module.azure_workflows[0].workflow_id : null) :
    var.provider_name == "gcp" ? (length(module.gcp_workflows) > 0 ? module.gcp_workflows[0].workflow_name : null) :
    null
  )
}
