output "identity_id" {
  description = "The ID of the identity"
  value       = local.identity_id
}

output "principal_id" {
  description = "The Principal ID / ARN / Email"
  value       = local.principal_id
}

output "provider" {
  description = "Cloud provider"
  value       = var.provider
}
