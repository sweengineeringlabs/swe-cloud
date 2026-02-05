variable "bucket_name" {
  description = "Bucket name (globally unique)"
  type        = string
}

variable "location" {
  description = "Bucket location (region or multi-region)"
  type        = string
  default     = "US"
}

variable "storage_class" {
  description = "Storage class (STANDARD, NEARLINE, COLDLINE, ARCHIVE)"
  type        = string
  default     = "STANDARD"
}

variable "uniform_bucket_level_access" {
  description = "Enable uniform bucket-level access"
  type        = bool
  default     = true
}

variable "versioning_enabled" {
  description = "Enable object versioning"
  type        = bool
  default     = false
}

variable "encryption_key_name" {
  description = "KMS key name for encryption"
  type        = string
  default     = null
  sensitive   = true
}

variable "block_public_access" {
  description = "Block all public access"
  type        = bool
  default     = true
}

variable "force_destroy" {
  description = "Allow deletion with objects"
  type        = bool
  default     = false
}

variable "lifecycle_rules" {
  description = "Lifecycle rules"
  type = list(object({
    action = object({
      type          = string
      storage_class = optional(string)
    })
    condition = object({
      age                   = optional(number)
      num_newer_versions    = optional(number)
      with_state            = optional(string)
      matches_storage_class = optional(list(string))
    })
  }))
  default = []
}

variable "labels" {
  description = "Resource labels"
  type        = map(string)
  default     = {}
}
