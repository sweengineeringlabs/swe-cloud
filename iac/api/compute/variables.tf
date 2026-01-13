# Compute Resource API Contract
# API Layer - Input Schema

# ============================================================================
# REQUIRED INPUTS
# ============================================================================

variable "instance_name" {
  description = "Name of the compute instance"
  type        = string
  validation {
    condition     = can(regex("^[a-z0-9]([a-z0-9-]*[a-z0-9])?$", var.instance_name))
    error_message = "Instance name must start and end with alphanumeric, contain only lowercase letters, numbers, and hyphens"
  }
  validation {
    condition     = length(var.instance_name) >= 3 && length(var.instance_name) <= 63
    error_message = "Instance name must be between 3 and 63 characters"
  }
}

variable "instance_size" {
  description = "Normalized instance size (from common layer)"
  type        = string
  validation {
    condition     = contains(["small", "medium", "large", "xlarge"], var.instance_size)
    error_message = "Instance size must be one of: small, medium, large, xlarge"
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
# OPTIONAL INPUTS
# ============================================================================

variable "ssh_public_key" {
  description = "SSH public key for instance access"
  type        = string
  default     = null
  sensitive   = true
  validation {
    condition     = var.ssh_public_key == null || can(regex("^ssh-(rsa|ed25519|ecdsa) ", var.ssh_public_key))
    error_message = "SSH key must be in valid OpenSSH format (ssh-rsa, ssh-ed25519, or ssh-ecdsa)"
  }
}

variable "admin_username" {
  description = "Admin username for the instance"
  type        = string
  default     = "cloudadmin"
  validation {
    condition     = can(regex("^[a-z][a-z0-9_-]*$", var.admin_username))
    error_message = "Admin username must start with a letter and contain only lowercase letters, numbers, underscores, and hyphens"
  }
}

variable "allow_public_access" {
  description = "Whether to allow public internet access"
  type        = bool
  default     = false
}

variable "enable_monitoring" {
  description = "Enable cloud provider monitoring"
  type        = bool
  default     = true
}

variable "enable_backup" {
  description = "Enable automated backups"
  type        = bool
  default     = true
}

variable "user_data" {
  description = "User data script for instance initialization"
  type        = string
  default     = null
}

variable "network_id" {
  description = "Network/VPC ID for the instance"
  type        = string
  default     = null
}

variable "subnet_id" {
  description = "Subnet ID for the instance"
  type        = string
  default     = null
}

variable "security_group_ids" {
  description = "Security group IDs to attach"
  type        = list(string)
  default     = []
}

variable "instance_tags" {
  description = "Instance-specific tags"
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
    ami                    = optional(string)
    instance_profile_name  = optional(string)
    ebs_optimized         = optional(bool, false)
    
    # Azure-specific
    resource_group_name   = optional(string)
    location              = optional(string)
    os_publisher          = optional(string, "Canonical")
    os_offer              = optional(string, "0001-com-ubuntu-server-jammy")
    os_sku                = optional(string, "22_04-lts")
    
    # GCP-specific
    project_id            = optional(string)
    zone                  = optional(string)
    machine_image         = optional(string, "ubuntu-2204-lts")
    
    # Oracle-specific
    compartment_id        = optional(string)
    availability_domain   = optional(string)
    image_id              = optional(string)
  })
  default = {}
}
