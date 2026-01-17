variable "instance_name" {
  description = "Database instance name"
  type        = string
}

variable "database_version" {
  description = "Database version (e.g., POSTGRES_15)"
  type        = string
  default     = "POSTGRES_15"
}

variable "region" {
  description = "GCP region"
  type        = string
}

variable "tier" {
  description = "Machine type (e.g., db-f1-micro)"
  type        = string
  default     = "db-f1-micro"
}

variable "high_availability" {
  description = "Enable high availability (REGIONAL)"
  type        = bool
  default     = false
}

variable "disk_size_gb" {
  description = "Disk size in GB"
  type        = number
  default     = 10
}

variable "disk_type" {
  description = "Disk type (PD_SSD, PD_HDD)"
  type        = string
  default     = "PD_SSD"
}

variable "disk_autoresize" {
  description = "Enable disk auto-resize"
  type        = bool
  default     = true
}

variable "backup_enabled" {
  description = "Enable automated backups"
  type        = bool
  default     = true
}

variable "binary_log_enabled" {
  description = "Enable binary logging (required for replication)"
  type        = bool
  default     = false
}

variable "backup_start_time" {
  description = "Backup start time (HH:MM)"
  type        = string
  default     = "03:00"
}

variable "transaction_log_retention_days" {
  description = "Transaction log retention days"
  type        = number
  default     = 7
}

variable "public_ip_enabled" {
  description = "Enable public IP"
  type        = bool
  default     = false
}

variable "private_network" {
  description = "VPC network link for private IP"
  type        = string
  default     = null
}

variable "authorized_networks" {
  description = "List of authorized networks"
  type = list(object({
    name = string
    cidr = string
  }))
  default = []
}

variable "max_connections" {
  description = "Max connections flag"
  type        = string
  default     = "100"
}

variable "deletion_protection" {
  description = "Enable deletion protection"
  type        = bool
  default     = true
}

variable "database_name" {
  description = "Database name"
  type        = string
}

variable "user_name" {
  description = "Database user name"
  type        = string
}

variable "user_password" {
  description = "Database user password"
  type        = string
  sensitive   = true
}
