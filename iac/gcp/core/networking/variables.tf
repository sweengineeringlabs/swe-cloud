variable "network_name" {
  description = "VPC network name"
  type        = string
}

variable "description" {
  description = "Network description"
  type        = string
  default     = "Managed by Terraform"
}

variable "auto_create_subnetworks" {
  description = "Auto-create subnetworks"
  type        = bool
  default     = false
}

variable "routing_mode" {
  description = "Network routing mode (GLOBAL or REGIONAL)"
  type        = string
  default     = "GLOBAL"
}

variable "subnets" {
  description = "List of subnets to create"
  type = list(object({
    name                     = string
    cidr                     = string
    region                   = string
    private_ip_google_access = optional(bool)
    secondary_ip_ranges      = optional(list(object({
      range_name    = string
      ip_cidr_range = string
    })))
  }))
  default = []
}

variable "create_internal_firewall" {
  description = "Create firewall rule allowing internal traffic"
  type        = bool
  default     = true
}

variable "create_ssh_firewall" {
  description = "Create firewall rule allowing SSH"
  type        = bool
  default     = true
}

variable "ssh_source_ranges" {
  description = "Source ranges for SSH firewall rule"
  type        = list(string)
  default     = ["0.0.0.0/0"]
}
