# Storage Core Variables
# Core Layer - Orchestration inputs

# ============================================================================
# FROM API LAYER
# ============================================================================

variable "bucket_name" {
  description = "Bucket name"
  type        = string
}

variable "provider" {
  description = "Cloud provider"
  type        = string
}

variable "storage_class" {
  description = "Normalized storage class"
  type        = string
  default     = "standard"
}

variable "versioning_enabled" {
  description = "Enable versioning"
  type        = bool
  default     = false
}

variable "encryption_enabled" {
  description = "Enable encryption"
  type        = bool
  default     = true
}

variable "encryption_key_id" {
  description = "Encryption key ID"
  type        = string
  default     = null
  sensitive   = true
}

variable "public_access_block" {
  description = "Block public access"
  type        = bool
  default     = true
}

variable "enable_logging" {
  description = "Enable access logging"
  type        = bool
  default     = true
}

variable "log_bucket_name" {
  description = "Log bucket name"
  type        = string
  default     = null
}

variable "cors_rules" {
  description = "CORS rules"
  type        = any
  default     = []
}

variable "lifecycle_rules" {
  description = "Lifecycle rules"
  type        = any
  default     = []
}

variable "replication_enabled" {
  description = "Enable replication"
  type        = bool
  default     = false
}

variable "replication_destination" {
  description = "Replication destination"
  type        = string
  default     = null
}

variable "bucket_tags" {
  description = "Bucket-specific tags"
  type        = map(string)
  default     = {}
}

variable "provider_config" {
  description = "Provider-specific configuration"
  type        = any
  default     = {}
}

# ============================================================================
# FROM COMMON LAYER
# ============================================================================

variable "storage_class_mapping" {
  description = "Storage class mappings from common layer"
  type        = map(map(string))
}

variable "common_tags" {
  description = "Common tags from common layer"
  type        = map(string)
  default     = {}
}
