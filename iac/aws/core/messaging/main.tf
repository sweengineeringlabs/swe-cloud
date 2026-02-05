# AWS Messaging (SQS & SNS)
# Mirrors CloudKit's cloudkit_core/aws/src/messaging.rs pattern

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

# ============================================================================
# SQS QUEUES
# ============================================================================

resource "aws_sqs_queue" "this" {
  count = var.create_queue ? 1 : 0
  
  name                        = var.queue_name
  fifo_queue                  = var.fifo_queue
  content_based_deduplication = var.content_based_deduplication
  
  visibility_timeout_seconds  = var.visibility_timeout_seconds
  message_retention_seconds   = var.message_retention_seconds
  max_message_size            = var.max_message_size
  delay_seconds               = var.delay_seconds
  receive_wait_time_seconds   = var.receive_wait_time_seconds
  
  sqs_managed_sse_enabled     = var.sqs_managed_sse_enabled
  
  redrive_policy = var.dead_letter_queue_arn != null ? jsonencode({
    deadLetterTargetArn = var.dead_letter_queue_arn
    maxReceiveCount     = var.max_receive_count
  }) : null

  tags = var.tags
}

# Dead Letter Queue (optional automatic creation)
resource "aws_sqs_queue" "dlq" {
  count = var.create_queue && var.create_dlq ? 1 : 0
  
  name = "${var.queue_name}-dlq"
  
  message_retention_seconds = var.dlq_message_retention_seconds
  sqs_managed_sse_enabled   = true
  
  tags = var.tags
}

# ============================================================================
# SNS TOPICS
# ============================================================================

resource "aws_sns_topic" "this" {
  count = var.create_topic ? 1 : 0
  
  name                        = var.topic_name
  fifo_topic                  = var.fifo_topic
  content_based_deduplication = var.content_based_deduplication
  
  kms_master_key_id = var.kms_master_key_id
  
  tags = var.tags
}

# Topic Subscription
resource "aws_sns_topic_subscription" "this" {
  count = var.create_topic && length(var.subscriptions) > 0 ? length(var.subscriptions) : 0
  
  topic_arn = aws_sns_topic.this[0].arn
  protocol  = var.subscriptions[count.index].protocol
  endpoint  = var.subscriptions[count.index].endpoint
  
  # For SQS subscriptions to SNS
  raw_message_delivery = lookup(var.subscriptions[count.index], "raw_message_delivery", false)
}

# ============================================================================
# OUTPUTS
# ============================================================================

output "queue_id" {
  description = "The URL for the created Amazon SQS queue"
  value       = var.create_queue ? aws_sqs_queue.this[0].id : null
}

output "queue_arn" {
  description = "The ARN of the SQS queue"
  value       = var.create_queue ? aws_sqs_queue.this[0].arn : null
}

output "dlq_id" {
  description = "The URL for the created Dead Letter Queue"
  value       = var.create_queue && var.create_dlq ? aws_sqs_queue.dlq[0].id : null
}

output "dlq_arn" {
  description = "The ARN of the Dead Letter Queue"
  value       = var.create_queue && var.create_dlq ? aws_sqs_queue.dlq[0].arn : null
}

output "topic_arn" {
  description = "The ARN of the SNS topic"
  value       = var.create_topic ? aws_sns_topic.this[0].arn : null
}
