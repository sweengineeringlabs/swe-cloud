# IAM API Contract - Input Variables
# Provider-agnostic Identity schema

variable "identity_name" {
  description = "Name of the identity (Role, User, Service Account)"
  type        = string
  
  validation {
    condition     = can(regex("^[a-z0-9-]+$", var.identity_name))
    error_message = "Identity name must contain only lowercase letters, numbers, and hyphens."
  }
}

variable "identity_type" {
  description = "Type of identity to create"
  type        = string
  # role: AWS Pattern (IAM Role), Azure Pattern (Custom Role Definition), GCP Pattern (Custom Role)
  # user: AWS Pattern (IAM User), Azure Pattern (Managed Identity), GCP Pattern (Service Account)
  validation {
    condition     = contains(["role", "user", "service_agent"], var.identity_type)
    error_message = "Identity type must be one of: role, user, service_agent."
  }
}

variable "permissions" {
  description = "List of abstract permissions/policies to attach"
  type        = list(string)
  default     = []
  # Examples: ["read-only", "admin", "custom-policy-json"]
}

variable "principals" {
  description = "List of principals that can assume this identity (Trust Policy)"
  type        = list(string)
  default     = []
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}

variable "provider_specific_config" {
  description = "Provider-specific configuration"
  type        = map(any)
  default     = {}
}
