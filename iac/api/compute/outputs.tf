# Compute Resource API Contract
# API Layer - Output Schema

# ============================================================================
# INSTANCE IDENTIFICATION
# ============================================================================

output "instance_id" {
  description = "Unique identifier of the compute instance (provider-specific format)"
  value       = var.instance_id
}

output "instance_arn" {
  description = "ARN/Resource ID of the instance (for cloud APIs)"
  value       = var.instance_arn
}

output "instance_name" {
  description = "Name of the instance"
  value       = var.instance_name
}

# ============================================================================
# INSTANCE SPECIFICATIONS
# ============================================================================

output "instance_type" {
  description = "Provider-specific instance type used (e.g., t3.medium, Standard_B2s)"
  value       = var.instance_type
}

output "instance_size" {
  description = "Normalized size that was requested (small, medium, large, xlarge)"
  value       = var.normalized_size
}

output "cpu_count" {
  description = "Number of vCPUs"
  value       = var.cpu_count
}

output "memory_gb" {
  description = "Memory in GB"
  value       = var.memory_gb
}

# ============================================================================
# NETWORK INFORMATION
# ============================================================================

output "public_ip" {
  description = "Public IP address (null if public access disabled)"
  value       = var.public_ip
}

output "private_ip" {
  description = "Private IP address"
  value       = var.private_ip
}

output "public_dns" {
  description = "Public DNS name (null if public access disabled)"
  value       = var.public_dns
}

output "private_dns" {
  description = "Private DNS name"
  value       = var.private_dns
}

output "network_interface_ids" {
  description = "Network interface IDs"
  value       = var.network_interface_ids
}

# ============================================================================
# CONNECTION INFORMATION
# ============================================================================

output "ssh_connection" {
  description = "SSH connection string"
  value       = var.ssh_connection
  sensitive   = true
}

output "ssh_command" {
  description = "Complete SSH command to connect"
  value       = var.ssh_command
  sensitive   = true
}

# ============================================================================
# STATE INFORMATION
# ============================================================================

output "state" {
  description = "Current state of the instance (running, stopped, etc.)"
  value       = var.state
}

output "availability_zone" {
  description = "Availability zone where instance is located"
  value       = var.availability_zone
}

output "region" {
  description = "Cloud region where instance is located"
  value       = var.region
}

# ============================================================================
# METADATA
# ============================================================================

output "metadata" {
  description = "Instance metadata"
  value = {
    provider          = var.provider
    normalized_size   = var.normalized_size
    instance_type     = var.instance_type
    created_at        = var.created_at
    managed_by        = "terraform-sea"
    architecture      = "SEA"
    layer             = "api"
  }
}

output "tags" {
  description = "All tags applied to the instance"
  value       = var.tags
}

# ============================================================================
# COST INFORMATION
# ============================================================================

output "estimated_hourly_cost" {
  description = "Estimated hourly cost in USD (approximate)"
  value       = var.estimated_hourly_cost
}

# ============================================================================
# INTERNAL OUTPUT VARIABLES (for use by other modules)
# ============================================================================

# These are the actual values that will be passed from provider implementations
variable "instance_id" {
  description = "Internal: Instance ID from provider"
  type        = string
  default     = ""
}

variable "instance_arn" {
  description = "Internal: Instance ARN from provider"
  type        = string
  default     = ""
}

variable "instance_type" {
  description = "Internal: Instance type from provider"
  type        = string
  default     = ""
}

variable "normalized_size" {
  description = "Internal: Normalized size"
  type        = string
  default     = ""
}

variable "cpu_count" {
  description = "Internal: CPU count"
  type        = number
  default     = 0
}

variable "memory_gb" {
  description = "Internal: Memory in GB"
  type        = number
  default     = 0
}

variable "public_ip" {
  description = "Internal: Public IP"
  type        = string
  default     = null
}

variable "private_ip" {
  description = "Internal: Private IP"
  type        = string
  default     = ""
}

variable "public_dns" {
  description = "Internal: Public DNS"
  type        = string
  default     = null
}

variable "private_dns" {
  description = "Internal: Private DNS"
  type        = string
  default     = ""
}

variable "network_interface_ids" {
  description = "Internal: Network interface IDs"
  type        = list(string)
  default     = []
}

variable "ssh_connection" {
  description = "Internal: SSH connection string"
  type        = string
  default     = ""
  sensitive   = true
}

variable "ssh_command" {
  description = "Internal: SSH command"
  type        = string
  default     = ""
  sensitive   = true
}

variable "state" {
  description = "Internal: Instance state"
  type        = string
  default     = ""
}

variable "availability_zone" {
  description = "Internal: Availability zone"
  type        = string
  default     = ""
}

variable "region" {
  description = "Internal: Region"
  type        = string
  default     = ""
}

variable "provider" {
  description = "Internal: Provider name"
  type        = string
  default     = ""
}

variable "created_at" {
  description = "Internal: Creation timestamp"
  type        = string
  default     = ""
}

variable "tags" {
  description = "Internal: Resource tags"
  type        = map(string)
  default     = {}
}

variable "estimated_hourly_cost" {
  description = "Internal: Estimated hourly cost"
  type        = number
  default     = 0
}
