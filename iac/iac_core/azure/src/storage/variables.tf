variable "storage_account_name" {
  description = "Storage account name (3-24 chars, lowercase alphanumeric)"
  type        = string
  
  validation {
    condition     = can(regex("^[a-z0-9]{3,24}$", var.storage_account_name))
    error_message = "Storage account name must be 3-24 lowercase alphanumeric characters."
  }
}

variable "resource_group_name" {
  description = "Resource group name"
  type        = string
}

variable "location" {
  description = "Azure region"
  type        = string
}

variable "account_tier" {
  description = "Storage account tier (Standard, Premium)"
  type        = string
  default     = "Standard"
}

variable "replication_type" {
  description = "Replication type (LRS, GRS, RAGRS, ZRS)"
  type        = string
  default     = "LRS"
}

variable "account_kind" {
  description = "Account kind (StorageV2, BlobStorage, FileStorage, BlockBlobStorage)"
  type        = string
  default     = "StorageV2"
}

variable "versioning_enabled" {
  description = "Enable blob versioning"
  type        = bool
  default     = false
}

variable "block_public_access" {
  description = "Block all public access"
  type        = bool
  default     = true
}

variable "delete_retention_days" {
  description = "Blob delete retention days"
  type        = number
  default     = 7
}

variable "container_delete_retention_days" {
  description = "Container delete retention days"
  type        = number
  default     = 7
}

variable "create_container" {
  description = "Create a default container"
  type        = bool
  default     = false
}

variable "container_name" {
  description = "Container name"
  type        = string
  default     = "data"
}

variable "container_access_type" {
  description = "Container access type (private, blob, container)"
  type        = string
  default     = "private"
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
