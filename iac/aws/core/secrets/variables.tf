variable "name" {
  description = "Name of the secret"
  type        = string
}

variable "description" {
  description = "Description of the secret"
  type        = string
  default     = null
}

variable "secret_string" {
  description = "The secret value (string)"
  type        = string
  default     = null
  sensitive   = true
}

variable "kms_key_id" {
  description = "KMS key ID to encrypt the secret"
  type        = string
  default     = null
}

variable "recovery_window_in_days" {
  description = "Number of days to retain secret after deletion"
  type        = number
  default     = 7
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
