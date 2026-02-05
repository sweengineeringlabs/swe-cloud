variable "name" {
  description = "Key name"
  type        = string
}

variable "key_vault_id" {
  description = "Key Vault ID"
  type        = string
}

variable "key_type" {
  description = "Key type (RSA, RSA-HSM, EC, EC-HSM)"
  type        = string
  default     = "RSA"
}

variable "key_size" {
  description = "Key size"
  type        = number
  default     = 2048
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
