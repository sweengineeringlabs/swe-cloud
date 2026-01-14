# Compute Core Variables
# Core Layer - Orchestration inputs

# ============================================================================
# FROM API LAYER (inputs)
# ============================================================================

variable "instance_name" {
  description = "Instance name"
  type        = string
}

variable "instance_size" {
  description = "Normalized instance size"
  type        = string
}

variable "provider" {
  description = "Cloud provider"
  type        = string
}

variable "ssh_public_key" {
  description = "SSH public key"
  type        = string
  default     = null
  sensitive   = true
}

variable "admin_username" {
  description = "Admin username"
  type        = string
  default     = "cloudadmin"
}

variable "allow_public_access" {
  description = "Allow public access"
  type        = bool
  default     = false
}

variable "enable_monitoring" {
  description = "Enable monitoring"
  type        = bool
  default     = true
}

variable "enable_backup" {
  description = "Enable backup"
  type        = bool
  default     = true
}

variable "user_data" {
  description = "User data script"
  type        = string
  default     = null
}

variable "network_id" {
  description = "Network/VPC ID"
  type        = string
  default     = null
}

variable "subnet_id" {
  description = "Subnet ID"
  type        = string
  default     = null
}

variable "security_group_ids" {
  description = "Security group IDs"
  type        = list(string)
  default     = []
}

variable "instance_tags" {
  description = "Instance-specific tags"
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

variable "compute_instance_types" {
  description = "Compute instance type mappings from common layer"
  type        = map(map(string))
}

variable "common_tags" {
  description = "Common tags from common layer"
  type        = map(string)
  default     = {}
}

# ============================================================================
# INTERNAL VARIABLES (from provider implementations)
# ============================================================================

variable "ssh_key_name" {
  description = "SSH key name (AWS-specific)"
  type        = string
  default     = null
}

variable "public_ip" {
  description = "Public IP (for output aggregation)"
  type        = string
  default     = null
}

variable "private_ip" {
  description = "Private IP (for output aggregation)"
  type        = string
  default     = ""
}
