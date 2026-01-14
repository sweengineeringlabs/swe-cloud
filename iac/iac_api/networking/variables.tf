# Networking API Contract - Input Variables
# Provider-agnostic network resource schema

variable "network_name" {
  description = "Name of the virtual network"
  type        = string
  
  validation {
    condition     = can(regex("^[a-z0-9-]+$", var.network_name))
    error_message = "Network name must contain only lowercase letters, numbers, and hyphens."
  }
}

variable "network_cidr" {
  description = "CIDR block for the network"
  type        = string
  
  validation {
    condition     = can(cidrhost(var.network_cidr, 0))
    error_message = "Network CIDR must be a valid CIDR block (e.g., 10.0.0.0/16)."
  }
}

variable "enable_dns" {
  description = "Enable DNS resolution within the network"
  type        = bool
  default     = true
}

variable "availability_zones" {
  description = "List of availability zones to use"
  type        = list(string)
}

variable "public_subnets" {
  description = "Configuration for public subnets"
  type = list(object({
    name = string
    cidr = string
    az   = string
  }))
  default = []
}

variable "private_subnets" {
  description = "Configuration for private subnets"
  type = list(object({
    name = string
    cidr = string
    az   = string
  }))
  default = []
}

variable "create_internet_gateway" {
  description = "Create an internet gateway for public access"
  type        = bool
  default     = true
}

variable "create_nat_gateway" {
  description = "Create NAT gateway for private subnet internet access"
  type        = bool
  default     = false
}

variable "nat_gateway_count" {
  description = "Number of NAT gateways (one per AZ for HA)"
  type        = number
  default     = 1
  
  validation {
    condition     = var.nat_gateway_count >= 0 && var.nat_gateway_count <= 10
    error_message = "NAT gateway count must be between 0 and 10."
  }
}

variable "enable_flow_logs" {
  description = "Enable network flow logs"
  type        = bool
  default     = false
}

variable "default_security_rules" {
  description = "Default security rules for the network"
  type = list(object({
    name        = string
    priority    = number
    direction   = string
    access      = string
    protocol    = string
    source_port = string
    dest_port   = string
    source_cidr = string
    dest_cidr   = string
  }))
  default = []
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
