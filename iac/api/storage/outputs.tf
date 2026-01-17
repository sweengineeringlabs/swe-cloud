# Storage Resource API Contract
# API Layer - Output Schema

# ============================================================================
# BUCKET IDENTIFICATION
# ============================================================================

output "bucket_id" {
  description = "Unique identifier of the storage bucket"
  value       = var.bucket_id
}

output "bucket_arn" {
  description = "ARN/Resource ID of the bucket"
  value       = var.bucket_arn
}

output "bucket_name" {
  description = "Name of the bucket"
  value       = var.bucket_name
}

# ============================================================================
# ACCESS INFORMATION
# ============================================================================

output "bucket_url" {
  description = "URL for accessing the bucket"
  value       = var.bucket_url
}

output "bucket_domain_name" {
  description = "Domain name of the bucket"
  value       = var.bucket_domain_name
}

output "bucket_regional_domain_name" {
  description = "Regional domain name (AWS-specific)"
  value       = var.bucket_regional_domain_name
}

# ============================================================================
# CONFIGURATION
# ============================================================================

output "versioning_enabled" {
  description = "Whether versioning is enabled"
  value       = var.versioning_enabled_out
}

output "encryption_enabled" {
  description = "Whether encryption is enabled"
  value       = var.encryption_enabled_out
}

output "storage_class" {
  description = "Storage class configuration"
  value       = var.storage_class_out
}

output "public_access_blocked" {
  description = "Whether public access is blocked"
  value       = var.public_access_blocked
}

# ============================================================================
# LOCATION INFORMATION
# ============================================================================

output "bucket_region" {
  description = "Region where bucket is located"
  value       = var.bucket_region
}

output "bucket_location" {
  description = "Location/zone of the bucket"
  value       = var.bucket_location
}

# ============================================================================
# METADATA
# ============================================================================

output "metadata" {
  description = "Bucket metadata"
  value = {
    provider        = var.provider_out
    storage_class   = var.storage_class_out
    versioning      = var.versioning_enabled_out
    encryption      = var.encryption_enabled_out
    created_at      = var.created_at
    managed_by      = "terraform-sea"
    architecture    = "SEA"
    layer           = "api"
  }
}

output "tags" {
  description = "All tags applied to the bucket"
  value       = var.tags_out
}

# ============================================================================
# COST INFORMATION
# ============================================================================

output "estimated_monthly_cost" {
  description = "Estimated monthly storage cost in USD (for 1GB)"
  value       = var.estimated_monthly_cost
}

# ============================================================================
# INTERNAL OUTPUT VARIABLES (for use by other modules)
# ============================================================================

variable "bucket_id" {
  description = "Internal: Bucket ID from provider"
  type        = string
  default     = ""
}

variable "bucket_arn" {
  description = "Internal: Bucket ARN from provider"
  type        = string
  default     = ""
}

variable "bucket_name" {
  description = "Internal: Bucket name"
  type        = string
  default     = ""
}

variable "bucket_url" {
  description = "Internal: Bucket URL"
  type        = string
  default     = ""
}

variable "bucket_domain_name" {
  description = "Internal: Bucket domain name"
  type        = string
  default     = ""
}

variable "bucket_regional_domain_name" {
  description = "Internal: Regional domain name"
  type        = string
  default     = null
}

variable "versioning_enabled_out" {
  description = "Internal: Versioning status"
  type        = bool
  default     = false
}

variable "encryption_enabled_out" {
  description = "Internal: Encryption status"
  type        = bool
  default     = false
}

variable "storage_class_out" {
  description = "Internal: Storage class"
  type        = string
  default     = ""
}

variable "public_access_blocked" {
  description = "Internal: Public access block status"
  type        = bool
  default     = false
}

variable "bucket_region" {
  description = "Internal: Bucket region"
  type        = string
  default     = ""
}

variable "bucket_location" {
  description = "Internal: Bucket location"
  type        = string
  default     = ""
}

variable "provider_out" {
  description = "Internal: Provider name"
  type        = string
  default     = ""
}

variable "created_at" {
  description = "Internal: Creation timestamp"
  type        = string
  default     = ""
}

variable "tags_out" {
  description = "Internal: Resource tags"
  type        = map(string)
  default     = {}
}

variable "estimated_monthly_cost" {
  description = "Internal: Estimated monthly cost"
  type        = number
  default     = 0
}
