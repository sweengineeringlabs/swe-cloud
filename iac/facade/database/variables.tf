# General Configuration
variable "provider_name" {
  description = "Cloud provider (aws, azure, gcp)"
  type        = string
  validation {
    condition     = contains(["aws", "azure", "gcp"], var.provider_name)
    error_message = "Provider must be one of: aws, azure, gcp."
  }
}

variable "project_name" {
  description = "Project name"
  type        = string
}

variable "environment" {
  description = "Environment (dev, staging, prod)"
  type        = string
  default     = "dev"
}

# Database Configuration
variable "identifier" {
  description = "Database identifier/name"
  type        = string
}

variable "database_name" {
  description = "Name of the database schema to create"
  type        = string
  default     = null
}

variable "engine" {
  description = "Database engine (postgres, mysql, etc.)"
  type        = string
  default     = "postgres"
}

variable "engine_version" {
  description = "Database engine version"
  type        = string
  default     = "13"
}

variable "instance_class" {
  description = "Abstract instance size (small, medium, large, xlarge)"
  type        = string
  default     = "small"
  validation {
    condition     = contains(["small", "medium", "large", "xlarge"], var.instance_class)
    error_message = "Instance class must be one of: small, medium, large, xlarge."
  }
}

variable "allocated_storage_gb" {
  description = "Storage size in GB"
  type        = number
  default     = 20
}

# Credentials
variable "master_username" {
  description = "Master username"
  type        = string
  default     = "admin"
}

variable "master_password" {
  description = "Master password"
  type        = string
  sensitive   = true
}

# Features
variable "publicly_accessible" {
  description = "Make database publicly accessible"
  type        = bool
  default     = false
}

variable "multi_az" {
  description = "Enable Multi-AZ / High Availability"
  type        = bool
  default     = false
}

variable "storage_encrypted" {
  description = "Enable storage encryption"
  type        = bool
  default     = true
}

variable "backup_retention_days" {
  description = "Backup retention days"
  type        = number
  default     = 7
}

# Provider Specifics
variable "provider_config" {
  description = "Provider-specific configuration (subnet_group, network_link, resource_group_name, etc.)"
  type        = any
  default     = {}
}

variable "tags" {
  description = "Additional tags"
  type        = map(string)
  default     = {}
}
