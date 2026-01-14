# AWS Provider Variables
# All AWS-specific configuration

# ============================================================================
# COMPUTE CONFIGURATION
# ============================================================================

variable "compute_config" {
  description = "Compute instance configuration"
  type = object({
    ami                   = string
    instance_type         = string
    ssh_key_name          = optional(string)
    subnet_id             = optional(string)
    security_group_ids    = optional(list(string), [])
    instance_profile_name = optional(string)
    user_data             = optional(string)
    enable_monitoring     = optional(bool, true)
    ebs_optimized         = optional(bool, false)
    tags                  = map(string)
  })
  default = null
}

# ============================================================================
# STORAGE CONFIGURATION
# ============================================================================

variable "storage_config" {
  description = "S3 bucket configuration"
  type = object({
    bucket_name          = string
    versioning_enabled   = optional(bool, false)
    encryption_enabled   = optional(bool, true)
    encryption_key_id    = optional(string)
    public_access_block  = optional(bool, true)
    force_destroy        = optional(bool, false)
    tags                 = map(string)
  })
  default = null
}

# ============================================================================
# FUTURE: DATABASE, NETWORKING, etc.
# ============================================================================

# variable "database_config" { ... }
# variable "network_config" { ... }
