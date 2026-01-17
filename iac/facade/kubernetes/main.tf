terraform {
  required_version = ">= 1.0"
}

locals {
  common_tags = merge(
    var.tags,
    {
      ManagedBy    = "Terraform"
      Environment  = var.environment
      Provider     = var.provider_name
      Project      = var.project_name
      Module       = "Kubernetes-Facade"
    }
  )
}

# --------------------------------------------------------------------------------
# AWS EKS
# --------------------------------------------------------------------------------
module "aws_eks" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../aws/core/kubernetes"

  cluster_name  = var.cluster_name
  node_count    = var.node_count
  instance_size = var.instance_size
  vpc_id        = var.vpc_id
  subnet_ids    = var.subnet_ids
  tags          = local.common_tags
}

# --------------------------------------------------------------------------------
# Azure AKS (Placeholder)
# --------------------------------------------------------------------------------
# module "azure_aks" { ... }

# --------------------------------------------------------------------------------
# GCP GKE (Placeholder)
# --------------------------------------------------------------------------------
# module "gcp_gke" { ... }

# --------------------------------------------------------------------------------
# Outputs Logic
# --------------------------------------------------------------------------------
locals {
  cluster_endpoint = (
    var.provider_name == "aws" && length(module.aws_eks) > 0 ? module.aws_eks[0].cluster_endpoint :
    var.provider_name == "zero" ? "https://localhost:6443" :
    "pending-implementation"
  )

  kubeconfig_command = (
    var.provider_name == "aws" ? "aws eks update-kubeconfig --name ${var.cluster_name}" :
    var.provider_name == "zero" ? "echo 'ZeroCloud Kubernetes Mock Configured'" :
    "echo 'Provider not supported'"
  )
}
