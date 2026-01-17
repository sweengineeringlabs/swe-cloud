# Database API Contract - Input Variables
# Provider-agnostic database resource schema

variable "database_identifier" {
  description = "Unique identifier for the database instance"
  type        = string
  
  validation {
    condition     = can(regex("^[a-z0-9-]+$", var.database_identifier))
    error_message = "Database identifier must contain only lowercase letters, numbers, and hyphens."
  }
}

variable "engine" {
  description = "Database engine type"
  type        = string
  
  validation {
    condition     = contains(["postgres", "mysql", "mariadb", "sqlserver", "oracle"], var.engine)
    error_message = "Engine must be one of: postgres, mysql, mariadb, sqlserver, oracle."
  }
}

variable "engine_version" {
  description = "Database engine version"
  type        = string
  default     = null
}

variable "instance_class" {
  description = "Normalized instance class (small, medium, large, xlarge)"
  type        = string
  
  validation {
    condition     = contains(["small", "medium", "large", "xlarge"], var.instance_class)
    error_message = "Instance class must be one of: small, medium, large, xlarge."
  }
}

variable "allocated_storage_gb" {
  description = "Allocated storage in gigabytes"
  type        = number
  default     = 20
  
  validation {
    condition     = var.allocated_storage_gb >= 20 && var.allocated_storage_gb <= 65536
    error_message = "Storage must be between 20 and 65536 GB."
  }
}

variable "storage_encrypted" {
  description = "Enable storage encryption"
  type        = bool
  default     = true
}

variable "database_name" {
  description = "Name of the initial database"
  type        = string
  default     = null
}

variable "master_username" {
  description = "Master username for the database"
  type        = string
  default     = "admin"
}

variable "master_password" {
  description = "Master password for the database"
  type        = string
  sensitive   = true
}

variable "publicly_accessible" {
  description = "Allow public access to the database"
  type        = bool
  default     = false
}

variable "multi_az" {
  description = "Enable high availability across multiple availability zones"
  type        = bool
  default     = false
}

variable "backup_retention_days" {
  description = "Number of days to retain backups"
  type        = number
  default     = 7
  
  validation {
    condition     = var.backup_retention_days >= 0 && var.backup_retention_days <= 35
    error_message = "Backup retention must be between 0 and 35 days."
  }
}

variable "enable_monitoring" {
  description = "Enable enhanced monitoring"
  type        = bool
  default     = true
}

variable "deletion_protection" {
  description = "Enable deletion protection"
  type        = bool
  default     = true
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}

variable "provider_specific_config" {
  description = "Provider-specific configuration options"
  type        = map(any)
  default     = {}
}
