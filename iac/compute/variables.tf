variable "provider" {
  description = "The cloud provider to use: 'aws', 'gcp', 'azure', or 'test'."
  type        = string
  validation {
    condition     = contains(["aws", "gcp", "azure", "test"], var.provider)
    error_message = "Valid values for provider are: 'aws', 'gcp', 'azure', or 'test'."
  }
}

variable "instance_name" {
  description = "The name for the compute instance. This will be used as a base for resource names."
  type        = string
}

variable "instance_size" {
  description = "A generic size for the instance (e.g., 'small', 'medium', 'large')."
  type        = string
  default     = "small"
  validation {
    condition     = contains(["small", "medium", "large"], var.instance_size)
    error_message = "Valid values for instance_size are: 'small', 'medium', or 'large'."
  }
}

variable "provider_config" {
  description = "A map of provider-specific configuration. For aws: { ami, location }. For gcp: { project_id, zone }. For azure: { location, resource_group_name, admin_password }."
  type        = any
  default     = {}
}
