variable "name" {
  description = "Name of the messaging resource (topic or queue)"
  type        = string
}

variable "type" {
  description = "Type of messaging resource (topic, queue)"
  type        = string
  validation {
    condition     = contains(["topic", "queue"], var.type)
    error_message = "Type must be 'topic' or 'queue'."
  }
}

variable "fifo" {
  description = "Whether to use First-In-First-Out"
  type        = bool
  default     = false
}

variable "subscription_protocol" {
  description = "Protocol for subscription (e.g. sqs, lambda, email, http)"
  type        = string
  default     = "sqs"
}

variable "endpoint" {
  description = "Endpoint for subscription"
  type        = string
  default     = null
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
