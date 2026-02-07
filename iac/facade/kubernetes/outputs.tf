output "cluster_name" {
  description = "Name of the Kubernetes cluster"
  value       = var.cluster_name
}

output "cluster_endpoint" {
  description = "Endpoint for the Kubernetes API server"
  value       = local.cluster_endpoint
}

output "kubeconfig_command" {
  description = "Command to update local kubeconfig"
  value       = local.kubeconfig_command
}
