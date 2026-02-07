variable "provider_name" {
  description = "Cloud provider (aws, azure, gcp)"
  type        = string
}

variable "name" {
  description = "Name of the messaging resource"
  type        = string
}

variable "type" {
  description = "Type of messaging resource (topic, queue)"
  type        = string
  default     = "queue"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "dev"
}

variable "project_name" {
  description = "Project name"
  type        = string
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
