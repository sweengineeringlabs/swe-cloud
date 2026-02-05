output "db_instance_id" {
  description = "Database instance ID"
  value       = local.db_id
}

output "db_endpoint" {
  description = "Database connection endpoint"
  value       = local.db_endpoint
}

output "db_name" {
  description = "Database name"
  value       = var.database_name
}

output "engine" {
  description = "Database engine"
  value       = var.engine
}

output "provider" {
  description = "Cloud provider"
  value       = var.provider
}
