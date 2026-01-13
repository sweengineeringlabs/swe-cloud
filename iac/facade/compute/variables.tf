# Compute Facade Variables
# Facade Layer - User-facing inputs (simplified)

# ============================================================================
# REQUIRED INPUTS
# ============================================================================

variable "provider" {
  description = "Cloud provider (aws, azure, gcp, or oracle)"
  type        = string
  validation {
    condition     = contains(["aws", "azure", "gcp", "oracle"], var.provider)
    error_message = "Provider must be one of: aws, azure, gcp, oracle"
  }
}

variable "instance_name" {
  description = "Name of the compute instance (3-63 lowercase alphanumeric characters with hyphens)"
  type        = string
  validation {
    condition     = can(regex("^[a-z0-9]([a-z0-9-]*[a-z0-9])?$", var.instance_name))
    error_message = "Instance name must be lowercase alphanumeric with hyphens, starting and ending with alphanumeric"
  }
}

variable "instance_size" {
  description = "Instance size (small, medium, large, or xlarge)"
  type        = string
  default     = "medium"
  validation {
    condition     = contains(["small", "medium", "large", "xlarge"], var.instance_size)
    error_message = "Instance size must be one of: small, medium, large, xlarge"
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
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "Environment must be one of: dev, staging, prod"
  }
}

# ============================================================================
# OPTIONAL INPUTS
# ============================================================================

variable "ssh_public_key" {
  description = "SSH public key for instance access (optional)"
  type        = string
  default     = null
  sensitive   = true
}

variable "admin_username" {
  description = "Admin username for the instance"
  type        = string
  default     = "cloudadmin"
}

variable "allow_public_access" {
  description = "Allow instance to have a public IP address"
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
  description = "Network/VPC ID (optional, will use default if not specified)"
  type        = string
  default     = null
}

variable "subnet_id" {
  description = "Subnet ID (optional, will use default if not specified)"
  type        = string
  default     = null
}

variable "security_group_ids" {
  description = "Security group IDs to attach (optional)"
  type        = list(string)
  default     = []
}

variable "tags" {
  description = "Additional tags to apply to the instance"
  type        = map(string)
  default     = {}
}

variable "instance_tags" {
  description = "Instance-specific tags (merged with common tags)"
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
      - ami: AMI ID (required for AWS)
      - instance_profile_name: IAM instance profile
      - ebs_optimized: Enable EBS optimization
    
    Azure:
      - resource_group_name: Resource group (required for Azure)
      - location: Azure region (e.g., eastus)
      - os_publisher: OS image publisher
      - os_offer: OS image offer
      - os_sku: OS image SKU
    
    GCP:
      - project_id: GCP project ID (required for GCP)
      - zone: GCP zone (e.g., us-central1-a)
      - machine_image: Boot disk image
    
    Oracle:
      - compartment_id: OCI compartment ID
      - availability_domain: OCI availability domain
      - image_id: OCI image ID
  EOT
  type        = any
  default     = {}
}
