variable "provider_name" {
  description = "Cloud provider (aws, azure, gcp)"
  type        = string
}

variable "project_name" {
  description = "Project name"
  type        = string
}

variable "environment" {
  description = "Environment name"
  type        = string
}

variable "identity_name" {
  description = "Name of the identity"
  type        = string
}

variable "identity_type" {
  description = "Type of identity (role, user, service_agent)"
  type        = string
  default     = "service_agent"
}

variable "principals" {
  description = "List of trusted principals (for roles)"
  type        = list(string)
  default     = []
}

variable "provider_config" {
  description = "Provider specific configuration"
  type        = map(string)
  default     = {}
}

variable "roles" {
  description = "List of high-level roles/capabilities to attach (e.g. storage_read, admin)"
  type        = list(string)
  default     = []
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
