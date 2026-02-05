# AWS EventBridge Core Module

resource "aws_cloudwatch_event_bus" "this" {
  name = var.name
  tags = var.tags
}

output "event_resource_id" {
  value = aws_cloudwatch_event_bus.this.name
}

output "event_resource_arn" {
  value = aws_cloudwatch_event_bus.this.arn
}
