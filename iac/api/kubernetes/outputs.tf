output "cluster_name" {
  description = "Name of the Kubernetes cluster"
  value       = var.cluster_name
}

output "cluster_endpoint" {
  description = "Endpoint for the Kubernetes API server"
}

output "cluster_certificate_authority_data" {
  description = "Base64 encoded certificate data required to communicate with the cluster"
}

output "kubeconfig_command" {
  description = "Command to update local kubeconfig"
}
