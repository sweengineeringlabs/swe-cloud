variable "cluster_name" {
  description = "Name of the Kubernetes cluster"
  type        = string
}

variable "node_count" {
  description = "Number of worker nodes"
  type        = number
  default     = 2
}

variable "instance_size" {
  description = "Size of worker nodes (small, medium, large)"
  type        = string
  default     = "medium"
  validation {
    condition     = contains(["small", "medium", "large"], var.instance_size)
    error_message = "Instance size must be one of: small, medium, large."
  }
}

variable "vpc_id" {
  description = "VPC ID where the cluster will be deployed"
  type        = string
}

variable "subnet_ids" {
  description = "List of subnet IDs for the cluster"
  type        = list(string)
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}

variable "environment" {
  description = "Deployment environment"
  type        = string
}

variable "project_name" {
  description = "Project name"
  type        = string
}
