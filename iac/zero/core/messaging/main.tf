terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

variable "queue_name" { type = string }
variable "topic_name" { type = string }
variable "create_queue" { type = bool }
variable "create_topic" { type = bool }
variable "tags" { type = map(string) }

# Reuse AWS Provider for ZeroQueue (redirected via SPI)
resource "aws_sqs_queue" "this" {
  count = var.create_queue ? 1 : 0
  name  = var.queue_name
  tags  = var.tags
}

resource "aws_sns_topic" "this" {
  count = var.create_topic ? 1 : 0
  name  = var.topic_name
  tags  = var.tags
}
