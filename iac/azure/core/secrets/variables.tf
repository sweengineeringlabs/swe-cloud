variable "name" {
  description = "Secret name"
  type        = string
}

variable "secret_value" {
  description = "Secret value"
  type        = string
  sensitive   = true
}

variable "key_vault_id" {
  description = "Key Vault ID"
  type        = string
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
