# Monitoring Facade
# Unified interface for Monitoring resources across providers

terraform {
  required_version = ">= 1.0"
}

locals {
  common_tags = merge(
    var.tags,
    {
      ManagedBy    = "Terraform"
      Environment  = var.environment
      Provider     = var.provider
      Project      = var.project_name
      Module       = "Monitoring-Facade"
    }
  )
}

# AWS: CloudWatch
module "aws_monitoring" {
  count  = var.provider == "aws" ? 1 : 0
  source = "../../iac_core/aws/src/monitoring"
  
  create_alarm        = true
  alarm_name          = var.alarm_name
  metric_name         = var.metric_name
  threshold           = var.threshold
  comparison_operator = var.comparison_operator
  evaluation_periods  = var.evaluation_periods
  period              = var.period
  namespace           = lookup(var.provider_config, "namespace", "AWS/EC2")
  statistic           = lookup(var.provider_config, "statistic", "Average")
  
  tags = local.common_tags
}

# Azure: Azure Monitor
module "azure_monitoring" {
  count  = var.provider == "azure" ? 1 : 0
  source = "../../iac_core/azure/src/monitoring"
  
  alarm_name          = var.alarm_name
  resource_group_name = lookup(var.provider_config, "resource_group_name", "monitoring-rg")
  scopes              = lookup(var.provider_config, "scopes", [])
  metric_name         = var.metric_name
  metric_namespace    = lookup(var.provider_config, "metric_namespace", "Microsoft.Compute/virtualMachines")
  aggregation         = lookup(var.provider_config, "aggregation", "Average")
  operator            = var.comparison_operator == "GreaterThanThreshold" ? "GreaterThan" : "LessThan"
  threshold           = var.threshold
  
  tags = local.common_tags
}

# GCP: Cloud Monitoring
module "gcp_monitoring" {
  count  = var.provider == "gcp" ? 1 : 0
  source = "../../iac_core/gcp/src/monitoring"
  
  create_alert_policy = true
  display_name        = var.alarm_name
  # project_id        = lookup(var.provider_config, "project_id", null) # Not in core variables
  
  # GCP uses MQL or filter strings, this is simplified for the facade
  filter          = "metric.type=\"compute.googleapis.com/instance/cpu/utilization\" AND resource.type=\"gce_instance\""
  threshold_value = var.threshold
  comparison      = var.comparison_operator == "GreaterThanThreshold" ? "COMPARISON_GT" : "COMPARISON_LT"
}

output "alarm_id" {
  value = (
    var.provider == "aws" ? (length(module.aws_monitoring) > 0 ? module.aws_monitoring[0].alarm_arn : null) :
    var.provider == "azure" ? (length(module.azure_monitoring) > 0 ? module.azure_monitoring[0].metric_alert_id : null) :
    var.provider == "gcp" ? (length(module.gcp_monitoring) > 0 ? module.gcp_monitoring[0].alert_policy_id : null) :
    null
  )
}
