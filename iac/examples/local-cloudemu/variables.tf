# Variables for CloudEmu testing

variable "aws_region" {
  description = "AWS region (used by CloudEmu for naming)"
  type        = string
  default     = "us-east-1"
}

variable "gcp_region" {
  description = "GCP region"
  type        = string
  default     = "us-central1"
}

variable "environment" {
  description = "Environment name (dev, test, local)"
  type        = string
  default     = "local"
}

variable "bucket_name" {
  description = "Name for the test S3 bucket"
  type        = string
  default     = "cloudemu-test-bucket"
}

variable "database_name" {
  description = "Name for the test DynamoDB table"
  type        = string
  default     = "cloudemu-test-table"
}

variable "queue_name" {
  description = "Name for the test SQS queue"
  type        = string
  default     = "cloudemu-test-queue"
}

variable "topic_name" {
  description = "Name for the test SNS topic"
  type        = string
  default     = "cloudemu-test-topic"
}

variable "function_name" {
  description = "Name for the test Lambda function"
  type        = string
  default     = "cloudemu-test-function"
}
