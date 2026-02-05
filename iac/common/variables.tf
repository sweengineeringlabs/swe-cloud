# Common variable definitions for all IAC modules
# Following CloudKit SEA architecture pattern

# ============================================================================
# PROVIDER CONFIGURATION
# ============================================================================

variable "provider" {
  description = "Cloud provider to use: aws, azure, gcp, or oracle"
  type        = string
  validation {
    condition     = contains(["aws", "azure", "gcp", "oracle"], var.provider)
    error_message = "Provider must be one of: aws, azure, gcp, oracle"
  }
}

variable "environment" {
  description = "Environment: dev, staging, or prod"
  type        = string
  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "Environment must be one of: dev, staging, prod"
  }
}

# ============================================================================
# RESOURCE SIZING
# ============================================================================

variable "resource_size" {
  description = "Normalized resource size: small, medium, large, or xlarge"
  type        = string
  default     = "medium"
  validation {
    condition     = contains(["small", "medium", "large", "xlarge"], var.resource_size)
    error_message = "Size must be one of: small, medium, large, xlarge"
  }
}

# ============================================================================
# PROJECT METADATA
# ============================================================================

variable "project_name" {
  description = "Name of the project"
  type        = string
  validation {
    condition     = can(regex("^[a-z0-9-]+$", var.project_name))
    error_message = "Project name must contain only lowercase letters, numbers, and hyphens"
  }
}

variable "cost_center" {
  description = "Cost center for billing"
  type        = string
  default     = "engineering"
}

variable "owner" {
  description = "Owner email or team name"
  type        = string
  validation {
    condition     = can(regex("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$|^[a-z-]+$", var.owner))
    error_message = "Owner must be a valid email or team name"
  }
}

# ============================================================================
# TAGGING
# ============================================================================

variable "tags" {
  description = "Custom tags to apply to all resources"
  type        = map(string)
  default     = {}
}

variable "enable_auto_tagging" {
  description = "Automatically apply standard tags to all resources"
  type        = bool
  default     = true
}

# ============================================================================
# NETWORKING
# ============================================================================

variable "network_size" {
  description = "Network CIDR size: small, medium, large, xlarge"
  type        = string
  default     = "medium"
  validation {
    condition     = contains(["small", "medium", "large", "xlarge"], var.network_size)
    error_message = "Network size must be one of: small, medium, large, xlarge"
  }
}

# ============================================================================
# SECURITY
# ============================================================================

variable "enable_encryption" {
  description = "Enable encryption at rest for all applicable resources"
  type        = bool
  default     = true
}

variable "enable_public_access" {
  description = "Allow public internet access to resources"
  type        = bool
  default     = false
}

variable "enable_monitoring" {
  description = "Enable monitoring and alerting"
  type        = bool
  default     = true
}

variable "enable_backup" {
  description = "Enable automated backups"
  type        = bool
  default     = true
}
