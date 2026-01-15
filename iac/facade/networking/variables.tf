variable "provider_name" {
  description = "Cloud provider (aws, azure, gcp)"
  type        = string
}

variable "project_name" {
  description = "Project name"
  type        = string
}

variable "environment" {
  description = "Environment name"
  type        = string
}

variable "network_name" {
  description = "Name of the network/vpc"
  type        = string
}

variable "metrics" {
  description = "Network metrics including CIDR, AZs, and subnet ranges"
  type = object({
    cidr            = string
    azs             = list(string)
    public_subnets  = list(string)
    private_subnets = list(string)
  })
}

variable "internet_access" {
  description = "Enable internet access (IGW)"
  type        = bool
  default     = true
}

variable "provider_config" {
  description = "Provider specific configuration (region, resource_group, etc)"
  type        = map(string)
  default     = {}
}

variable "tags" {
  description = "Additional tags"
  type        = map(string)
  default     = {}
}
