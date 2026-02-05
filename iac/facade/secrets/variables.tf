variable "provider_name" {
  description = "Cloud provider (aws, azure, gcp)"
  type        = string
}

variable "name" {
  description = "Secret name"
  type        = string
}

variable "description" {
  description = "Secret description"
  type        = string
  default     = null
}

variable "secret_string" {
  description = "Secret value"
  type        = string
  default     = null
  sensitive   = true
}

variable "environment" {
  description = "Deployment environment"
  type        = string
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
