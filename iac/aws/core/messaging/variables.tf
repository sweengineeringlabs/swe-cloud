# SQS Configuration
variable "create_queue" {
  description = "Create SQS queue"
  type        = bool
  default     = false
}

variable "queue_name" {
  description = "Name of the queue"
  type        = string
  default     = null
}

variable "fifo_queue" {
  description = "Boolean designating a FIFO queue"
  type        = bool
  default     = false
}

variable "content_based_deduplication" {
  description = "Enable content-based deduplication for FIFO queues"
  type        = bool
  default     = false
}

variable "visibility_timeout_seconds" {
  description = "Visibility timeout for the queue"
  type        = number
  default     = 30
}

variable "message_retention_seconds" {
  description = "Message retention in seconds"
  type        = number
  default     = 345600 # 4 days
}

variable "max_message_size" {
  description = "Max message size in bytes"
  type        = number
  default     = 262144 # 256 KB
}

variable "delay_seconds" {
  description = "Delay seconds"
  type        = number
  default     = 0
}

variable "receive_wait_time_seconds" {
  description = "Receive wait time seconds"
  type        = number
  default     = 0
}

variable "sqs_managed_sse_enabled" {
  description = "Enable server-side encryption with SQS-owned key"
  type        = bool
  default     = true
}

# Dead Letter Queue
variable "create_dlq" {
  description = "Create a managed Dead Letter Queue"
  type        = bool
  default     = false
}

variable "dead_letter_queue_arn" {
  description = "ARN of an existing DLQ (if create_dlq is false)"
  type        = string
  default     = null
}

variable "max_receive_count" {
  description = "Max receive count before moving to DLQ"
  type        = number
  default     = 3
}

variable "dlq_message_retention_seconds" {
  description = "Message retention for DLQ"
  type        = number
  default     = 1209600 # 14 days
}

# SNS Configuration
variable "create_topic" {
  description = "Create SNS topic"
  type        = bool
  default     = false
}

variable "topic_name" {
  description = "Name of the SNS topic"
  type        = string
  default     = null
}

variable "fifo_topic" {
  description = "Boolean designating a FIFO topic"
  type        = bool
  default     = false
}

variable "kms_master_key_id" {
  description = "ID of an AWS-managed customer master key (CMK) for SNS"
  type        = string
  default     = "alias/aws/sns"
}

variable "subscriptions" {
  description = "List of subscriptions for the topic"
  type = list(object({
    protocol             = string
    endpoint             = string
    raw_message_delivery = optional(bool)
  }))
  default = []
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
