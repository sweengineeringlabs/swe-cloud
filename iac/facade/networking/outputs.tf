output "network_id" {
  description = "The ID of the network (VPC/VNet)"
  value       = local.network_id
}

output "provider" {
  description = "Cloud provider"
  value       = var.provider
}

output "cidr" {
  description = "Network CIDR"
  value       = var.metrics.cidr
}
