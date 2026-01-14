variable "vnet_name" {
  description = "Virtual network name"
  type        = string
}

variable "resource_group_name" {
  description = "Resource group name"
  type        = string
}

variable "location" {
  description = "Azure region"
  type        = string
}

variable "address_space" {
  description = "Address space for VNet (CIDR)"
  type        = string
  default     = "10.0.0.0/16"
}

variable "public_subnets" {
  description = "Public subnet configurations"
  type = list(object({
    name           = string
    address_prefix = string
  }))
  default = []
}

variable "private_subnets" {
  description = "Private subnet configurations"
  type = list(object({
    name           = string
    address_prefix = string
  }))
  default = []
}

variable "create_default_nsg" {
  description = "Create default network security group"
  type        = bool
  default     = true
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
