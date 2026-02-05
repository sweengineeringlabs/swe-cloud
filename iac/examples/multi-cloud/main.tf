# Multi-Cloud Deployment Example
# Deploys resources to AWS, Azure, and GCP simultaneously using Facades

terraform {
  required_version = ">= 1.0"
}

variable "project_name" { default = "global-app" }
variable "environment" { default = "prod" }

# ============================================================================
# AWS: Frontend & API Layer
# ============================================================================

module "aws_network" {
  source = "../../facade/networking"
  
  provider_name = "aws"
  project_name = var.project_name
  environment  = var.environment
  network_name = "aws-vpc"
  
  metrics = {
    cidr            = "10.1.0.0/16"
    azs             = ["us-east-1a"]
    public_subnets  = ["10.1.1.0/24"]
    private_subnets = ["10.1.2.0/24"]
  }
}

module "aws_compute" {
  source = "../../facade/compute"
  
  provider_name = "aws"
  project_name  = var.project_name
  environment   = var.environment
  instance_name = "api-server"
  instance_size = "medium"
  network_id    = module.aws_network.network_id
}

# ============================================================================
# AZURE: Corporate Data & Auth
# ============================================================================

module "azure_network" {
  source = "../../facade/networking"
  
  provider_name = "azure"
  project_name = var.project_name
  environment  = var.environment
  network_name = "azure-vnet"
  
  metrics = {
    cidr            = "10.2.0.0/16"
    azs             = []
    public_subnets  = ["10.2.1.0/24"]
    private_subnets = ["10.2.2.0/24"]
  }
  
  provider_config = {
    resource_group_name = "global-app-rg"
    location            = "eastus"
  }
}

module "azure_db" {
  source = "../../facade/database"
  
  provider_name = "azure"
  identifier   = "corp-db"
  project_name = var.project_name
  environment  = var.environment
  engine       = "sqlserver"
  
  master_username = "adminuser"
  master_password = "SecurePassword123!"
  
  provider_config = {
    resource_group_name = "global-app-rg"
    location            = "eastus"
  }
}

# ============================================================================
# GCP: Analytics & Big Data
# ============================================================================

module "gcp_storage" {
  source = "../../facade/storage"
  
  provider_name = "gcp"
  bucket_name  = "global-analytics-data"
  project_name = var.project_name
  environment  = var.environment
  
  provider_config = {
    region = "us-central1"
  }
}

module "gcp_compute" {
  source = "../../facade/compute"
  
  provider_name = "gcp"
  project_name  = var.project_name
  environment   = var.environment
  instance_name = "analytics-worker"
  
  provider_config = {
    zone = "us-central1-a"
  }
}

# ============================================================================
# OUTPUTS
# ============================================================================

output "aws_api_endpoint" { value = module.aws_compute.public_ip }
output "azure_db_connection" { value = module.azure_db.db_endpoint }
output "gcp_bucket_url" { value = module.gcp_storage.bucket_url }
