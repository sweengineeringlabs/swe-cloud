# IAM API Contract - Output Values
# Provider-agnostic Identity outputs

output "identity_id" {
  description = "Unique identifier of the identity"
  value       = var.identity_id
}

output "identity_arn" {
  description = "Provider resource identifier (ARN, Resource ID, Email)"
  value       = var.identity_arn
}

output "identity_name" {
  description = "Name of the identity"
  value       = var.identity_name
}

output "principal_id" {
  description = "Principal ID used for policy binding"
  value       = var.principal_id
}

output "client_id" {
  description = "Client/Access Key ID (if applicable)"
  value       = var.client_id
  sensitive   = true
}

output "client_secret" {
  description = "Client/Secret Key (if applicable)"
  value       = var.client_secret
  sensitive   = true
}

# Variable definitions for contract enforcement
variable "identity_id" { type = string }
variable "identity_arn" { type = string }
variable "identity_name" { type = string }
variable "principal_id" { type = string }
variable "client_id" { type = string }
variable "client_secret" { type = string }
