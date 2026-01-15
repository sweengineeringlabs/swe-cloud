variable "provider_name" {
  description = "Cloud provider (aws, azure, gcp)"
  type        = string
}

variable "function_name" {
  description = "Name of the function"
  type        = string
}

variable "handler" {
  description = "Function entrypoint"
  type        = string
  default     = "index.handler"
}

variable "runtime" {
  description = "Function runtime"
  type        = string
  default     = "python3.9"
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

variable "provider_config" {
  description = "Provider-specific configuration"
  type        = any
  default     = {}
}

variable "source_code" {
  description = "Inline source code"
  type        = string
  default     = null
}

variable "environment_variables" {
  description = "Environment variables"
  type        = map(string)
  default     = {}
}

