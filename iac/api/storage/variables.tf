# Storage Resource API Contract
# API Layer - Input Schema

# ============================================================================
# REQUIRED INPUTS
# ============================================================================

variable "bucket_name" {
  description = "Name of the storage bucket"
  type        = string
  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]*[a-z0-9]$", var.bucket_name))
    error_message = "Bucket name must start and end with alphanumeric, contain only lowercase letters, numbers, and hyphens"
  }
  validation {
    condition     = length(var.bucket_name) >= 3 && length(var.bucket_name) <= 63
    error_message = "Bucket name must be between 3 and 63 characters"
  }
}

variable "provider" {
  description = "Cloud provider"
  type        = string
  validation {
    condition     = contains(["aws", "azure", "gcp", "oracle"], var.provider)
    error_message = "Provider must be one of: aws, azure, gcp, oracle"
  }
}

# ============================================================================
# STORAGE CONFIGURATION
# ============================================================================

variable "storage_class" {
  description = "Storage class/tier"
  type        = string
  default     = "standard"
  validation {
    condition     = contains(["standard", "infrequent", "archive", "cold"], var.storage_class)
    error_message = "Storage class must be one of: standard, infrequent, archive, cold"
  }
}

variable "versioning_enabled" {
  description = "Enable object versioning"
  type        = bool
  default     = false
}

variable "encryption_enabled" {
  description = "Enable encryption at rest"
  type        = bool
  default     = true
}

variable "encryption_key_id" {
  description = "KMS key ID for encryption (provider-specific)"
  type        = string
  default     = null
  sensitive   = true
}

# ============================================================================
# ACCESS CONTROL
# ============================================================================

variable "public_access_block" {
  description = "Block all public access"
  type        = bool
  default     = true
}

variable "enable_logging" {
  description = "Enable access logging"
  type        = bool
  default     = true
}

variable "log_bucket_name" {
  description = "Bucket for storing access logs"
  type        = string
  default     = null
}

variable "cors_rules" {
  description = "CORS rules for the bucket"
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
  description = "Lifecycle rules for object management"
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
  description = "Enable cross-region replication"
  type        = bool
  default     = false
}

variable "replication_destination" {
  description = "Destination bucket for replication"
  type        = string
  default     = null
}

# ============================================================================
# TAGGING
# ============================================================================

variable "bucket_tags" {
  description = "Bucket-specific tags"
  type        = map(string)
  default     = {}
}

# ============================================================================
# PROVIDER-SPECIFIC CONFIGURATION
# ============================================================================

variable "provider_config" {
  description = "Provider-specific configuration"
  type = object({
    # AWS-specific
    acl                    = optional(string, "private")
    force_destroy          = optional(bool, false)
    object_lock_enabled    = optional(bool, false)
    
    # Azure-specific
    resource_group_name   = optional(string)
    location              = optional(string)
    account_tier          = optional(string, "Standard")
    account_replication_type = optional(string, "LRS")
    
    # GCP-specific
    project_id            = optional(string)
    location              = optional(string, "US")
    uniform_bucket_level_access = optional(bool, true)
    
    # Oracle-specific
    compartment_id        = optional(string)
    namespace             = optional(string)
  })
  default = {}
}
