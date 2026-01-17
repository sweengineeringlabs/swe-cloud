# Storage Facade Variables
# Facade Layer - User-facing inputs (simplified)

# ============================================================================
# REQUIRED INPUTS
# ============================================================================

variable "provider_name" {
  description = "Cloud provider (aws, azure, gcp, or oracle)"
  type        = string
  validation {
    condition     = contains(["aws", "azure", "gcp", "oracle", "zero"], var.provider_name)
    error_message = "Provider must be one of: aws, azure, gcp, oracle, zero"
  }
}

variable "bucket_name" {
  description = "Name of the storage bucket (3-63 lowercase alphanumeric characters with hyphens)"
  type        = string
  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]*[a-z0-9]$", var.bucket_name))
    error_message = "Bucket name must be lowercase alphanumeric with hyphens, starting and ending with alphanumeric"
  }
}

variable "project_name" {
  description = "Project name for tagging and organization"
  type        = string
}

variable "environment" {
  description = "Environment (dev, staging, or prod)"
  type        = string
  default     = "dev"
  validation {
    condition     = contains(["local", "dev", "staging", "prod"], var.environment)
    error_message = "Environment must be one of: dev, staging, prod"
  }
}

# ============================================================================
# STORAGE CONFIGURATION
# ============================================================================

variable "storage_class" {
  description = "Storage class/tier (standard, infrequent, archive, or cold)"
  type        = string
  default     = "standard"
  validation {
    condition     = contains(["standard", "infrequent", "archive", "cold"], var.storage_class)
    error_message = "Storage class must be one of: standard, infrequent, archive, cold"
  }
}

variable "versioning_enabled" {
  description = "Enable object versioning for data protection"
  type        = bool
  default     = false
}

variable "encryption_enabled" {
  description = "Enable encryption at rest (recommended)"
  type        = bool
  default     = true
}

variable "encryption_key_id" {
  description = "KMS key ID for encryption (optional, uses default if not specified)"
  type        = string
  default     = null
  sensitive   = true
}

variable "public_access_block" {
  description = "Block all public access (recommended for security)"
  type        = bool
  default     = true
}

# ============================================================================
# LOGGING & MONITORING
# ============================================================================

variable "enable_logging" {
  description = "Enable access logging"
  type        = bool
  default     = true
}

variable "log_bucket_name" {
  description = "Bucket name for storing access logs (optional)"
  type        = string
  default     = null
}

# ============================================================================
# CORS CONFIGURATION
# ============================================================================

variable "cors_rules" {
  description = <<-EOT
    CORS rules for cross-origin requests. Example:
    [{
      allowed_origins = ["https://example.com"]
      allowed_methods = ["GET", "HEAD"]
      allowed_headers = ["*"]
      max_age_seconds = 3000
    }]
  EOT
  type = list(object({
    allowed_origins = list(string)
    allowed_methods = list(string)
    allowed_headers = list(string)
    expose_headers  = optional(list(string), [])
    max_age_seconds = optional(number, 3000)
  }))
  default = []
}

# ============================================================================
# LIFECYCLE MANAGEMENT
# ============================================================================

variable "lifecycle_rules" {
  description = <<-EOT
    Lifecycle rules for automatic object management. Example:
    [{
      id      = "archive-old-data"
      enabled = true
      prefix  = "logs/"
      transition = [{
        days          = 30
        storage_class = "infrequent"
      }, {
        days          = 90
        storage_class = "archive"
      }]
      expiration = {
        days = 365
      }
    }]
  EOT
  type = list(object({
    id      = string
    enabled = bool
    prefix  = optional(string, "")
    
    transition = optional(list(object({
      days          = number
      storage_class = string
    })), [])
    
    expiration = optional(object({
      days = number
    }), null)
    
    noncurrent_version_expiration = optional(object({
      days = number
    }), null)
  }))
  default = []
}

# ============================================================================
# REPLICATION
# ============================================================================

variable "replication_enabled" {
  description = "Enable cross-region replication for disaster recovery"
  type        = bool
  default     = false
}

variable "replication_destination" {
  description = "Destination bucket for replication (required if replication_enabled is true)"
  type        = string
  default     = null
}

# ============================================================================
# TAGGING
# ============================================================================

variable "tags" {
  description = "Additional tags to apply to the bucket"
  type        = map(string)
  default     = {}
}

variable "bucket_tags" {
  description = "Bucket-specific tags (merged with common tags)"
  type        = map(string)
  default     = {}
}

# ============================================================================
# PROVIDER-SPECIFIC CONFIGURATION
# ============================================================================

variable "provider_config" {
  description = <<-EOT
    Provider-specific configuration options:
    
    AWS:
      - acl: Access control list (private, public-read, etc.)
      - force_destroy: Allow deletion of non-empty bucket
      - object_lock_enabled: Enable object lock for compliance
    
    Azure:
      - resource_group_name: Resource group (required for Azure)
      - location: Azure region (e.g., eastus)
      - account_tier: Storage account tier (Standard, Premium)
      - account_replication_type: Replication type (LRS, GRS, etc.)
    
    GCP:
      - project_id: GCP project ID (required for GCP)
      - location: GCP location (e.g., US, EU, asia-southeast1)
      - uniform_bucket_level_access: Use uniform access control
    
    Oracle:
      - compartment_id: OCI compartment ID
      - namespace: OCI object storage namespace
  EOT
  type        = any
  default     = {}
}
