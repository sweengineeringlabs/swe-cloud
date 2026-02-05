# Service Account Variables
variable "create_service_account" {
  description = "Create a Service Account"
  type        = bool
  default     = false
}

variable "account_id" {
  description = "The service account ID"
  type        = string
  default     = null
}

variable "display_name" {
  description = "The display name for the service account"
  type        = string
  default     = null
}

variable "description" {
  description = "Description of the service account"
  type        = string
  default     = null
}

# Role Binding Variables
variable "project_id" {
  description = "The project ID"
  type        = string
  default     = null
}

variable "project_roles" {
  description = "List of IAM roles to assign to the service account"
  type        = list(string)
  default     = []
}

variable "member" {
  description = "IAM member to assign roles to (if not creating SA)"
  type        = string
  default     = null
}

# Key Variables
variable "create_key" {
  description = "Create a service account key"
  type        = bool
  default     = false
}

# Custom Role Variables
variable "create_custom_role" {
  description = "Create a custom IAM role"
  type        = bool
  default     = false
}

variable "role_id" {
  description = "The ID of the custom role"
  type        = string
  default     = null
}

variable "role_title" {
  description = "The title of the custom role"
  type        = string
  default     = null
}

variable "role_description" {
  description = "The description of the custom role"
  type        = string
  default     = null
}

variable "permissions" {
  description = "The names of the permissions this role grants"
  type        = list(string)
  default     = []
}
