variable "subscription_id" { type = string }
variable "tenant_id" { type = string }
variable "client_id" { type = string; default = null }
variable "client_secret" { type = string; default = null }

variable "stack_name" {
  description = "Name of the stack (e.g. dev, prod)"
  type        = string
}

variable "region" {
  description = "Default Azure region"
  type        = string
  default     = "eastus"
}
