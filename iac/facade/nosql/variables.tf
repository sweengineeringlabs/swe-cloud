variable "provider_name" {
  description = "Cloud provider (aws, azure, gcp)"
  type        = string
}

variable "table_name" {
  description = "NoSQL table name"
  type        = string
}

variable "hash_key" {
  description = "Partition key name"
  type        = string
}

variable "hash_key_type" {
  description = "Partition key type (S, N, B)"
  type        = string
  default     = "S"
}

variable "range_key" {
  description = "Sort key name"
  type        = string
  default     = null
}

variable "range_key_type" {
  description = "Sort key type (S, N, B)"
  type        = string
  default     = "S"
}

variable "environment" {
  description = "Deployment environment"
  type        = string
  default     = "dev"
}

variable "project_name" {
  description = "Project name"
  type        = string
}

variable "tags" {
  description = "Additional tags"
  type        = map(string)
  default     = {}
}
