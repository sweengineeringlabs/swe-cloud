variable "server_name" {
  description = "SQL Server name"
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

variable "server_version" {
  description = "SQL Server version"
  type        = string
  default     = "12.0"
}

variable "admin_username" {
  description = "Administrator username"
  type        = string
}

variable "admin_password" {
  description = "Administrator password"
  type        = string
  sensitive   = true
}

variable "database_name" {
  description = "Database name"
  type        = string
}

variable "sku_name" {
  description = "Database SKU (e.g., S0, S1, P1)"
  type        = string
  default     = "S0"
}

variable "max_size_gb" {
  description = "Maximum database size in GB"
  type        = number
  default     = 2
}

variable "zone_redundant" {
  description = "Enable zone redundancy"
  type        = bool
  default     = false
}

variable "storage_account_type" {
  description = "Storage account type"
  type        = string
  default     = "Geo"
}

variable "allowed_ip_ranges" {
  description = "Allowed IP ranges for firewall"
  type = list(object({
    start = string
    end   = string
  }))
  default = []
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
