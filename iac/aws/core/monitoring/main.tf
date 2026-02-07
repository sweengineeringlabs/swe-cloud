# AWS Monitoring (CloudWatch)
# Mirrors CloudKit's cloudkit_core/aws/src/cloudwatch.rs pattern

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

# Metric Alarm
resource "aws_cloudwatch_metric_alarm" "this" {
  count = var.create_alarm ? 1 : 0
  
  alarm_name          = var.alarm_name
  comparison_operator = var.comparison_operator
  evaluation_periods  = var.evaluation_periods
  metric_name         = var.metric_name
  namespace           = var.namespace
  period              = var.period
  statistic           = var.statistic
  threshold           = var.threshold
  
  alarm_description = var.alarm_description
  alarm_actions     = var.alarm_actions
  ok_actions        = var.ok_actions
  
  dimensions = var.dimensions
  
  tags = var.tags
}

# Log Group
resource "aws_cloudwatch_log_group" "this" {
  count = var.create_log_group ? 1 : 0
  
  name              = var.log_group_name
  retention_in_days = var.retention_in_days
  kms_key_id        = var.kms_key_id
  
  tags = var.tags
}

# Dashboard
resource "aws_cloudwatch_dashboard" "this" {
  count = var.create_dashboard ? 1 : 0
  
  dashboard_name = var.dashboard_name
  dashboard_body = var.dashboard_body
}

# Outputs
output "alarm_arn" {
  description = "ARN of the CloudWatch alarm"
  value       = var.create_alarm ? aws_cloudwatch_metric_alarm.this[0].arn : null
}

output "log_group_arn" {
  description = "ARN of the Log Group"
  value       = var.create_log_group ? aws_cloudwatch_log_group.this[0].arn : null
}
