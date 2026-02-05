variable "bucket_name" {
  description = "S3 bucket name"
  type        = string
}

variable "versioning_enabled" {
  description = "Enable bucket versioning"
  type        = bool
  default     = false
}

variable "encryption_enabled" {
  description = "Enable server-side encryption"
  type        = bool
  default     = true
}

variable "encryption_key_id" {
  description = "KMS key ID for encryption"
  type        = string
  default     = null
  sensitive   = true
}

variable "public_access_block" {
  description = "Block all public access"
  type        = bool
  default     = true
}

variable "force_destroy" {
  description = "Allow bucket deletion with objects"
  type        = bool
  default     = false
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
