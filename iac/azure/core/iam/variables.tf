variable "create_identity" {
  description = "Create User Assigned Managed Identity"
  type        = bool
  default     = false
}

variable "identity_name" {
  description = "Name of the User Assigned Identity"
  type        = string
  default     = null
}

variable "resource_group_name" {
  description = "Resource Group Name"
  type        = string
  default     = null
}

variable "location" {
  description = "Azure Region"
  type        = string
  default     = null
}

# Role Assignments
variable "create_assignment" {
  description = "Create IAM Role Assignment"
  type        = bool
  default     = false
}

variable "scope_type" {
  description = "Scope of assignment (resource_group, subscription)"
  type        = string
  default     = "resource_group"
}

variable "scope_id" {
  description = "The Scope ID (RG ID or Subscription ID)"
  type        = string
  default     = null
}

variable "role_definition_name" {
  description = "Name of the built-in role (e.g. Contributor)"
  type        = string
  default     = null
}

variable "principal_id" {
  description = "Principal ID to assign role to (if not creating identity)"
  type        = string
  default     = null
}

# Custom Role Definitions
variable "create_role_definition" {
  description = "Create custom role definition"
  type        = bool
  default     = false
}

variable "role_name" {
  description = "Name of the custom role"
  type        = string
  default     = null
}

variable "role_description" {
  description = "Description of the custom role"
  type        = string
  default     = null
}

variable "role_actions" {
  description = "List of actions for custom role"
  type        = list(string)
  default     = []
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
