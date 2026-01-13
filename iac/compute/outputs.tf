output "instance_name" {
  description = "The name of the provisioned compute instance."
  value       = var.instance_name
}

output "selected_provider" {
  description = "The cloud provider that was used to provision the resource."
  value       = var.provider
}
