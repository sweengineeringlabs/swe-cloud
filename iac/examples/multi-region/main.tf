# Multi-Region Deployment Example
# Demonstrates High Availability and Disaster Recovery using Facades

terraform {
  required_version = ">= 1.0"
}

variable "project_name" { default = "dr-capable-app" }
variable "environment" { default = "prod" }

# ============================================================================
# PRIMARY REGION: US-EAST-1
# ============================================================================

module "primary_storage" {
  source = "../../facade/storage"
  
  provider_name = "aws"
  bucket_name   = "dr-app-primary-storage"
  project_name  = var.project_name
  environment   = var.environment
  
  provider_config = {
    region = "us-east-1"
  }
}

module "primary_compute" {
  source = "../../facade/compute"
  
  provider_name = "aws"
  instance_name = "dr-app-primary-instance"
  instance_size = "medium"
  project_name  = var.project_name
  environment   = var.environment
  
  provider_config = {
    region = "us-east-1"
    ami    = "ami-0c55b159cbfafe1f0"
  }
}

# ============================================================================
# SECONDARY REGION: US-WEST-2 (DR)
# ============================================================================

module "secondary_storage" {
  source = "../../facade/storage"
  
  provider_name = "aws"
  bucket_name   = "dr-app-secondary-storage"
  project_name  = var.project_name
  environment   = var.environment
  
  provider_config = {
    region = "us-west-2"
  }
}

module "secondary_compute" {
  source = "../../facade/compute"
  
  provider_name = "aws"
  instance_name = "dr-app-secondary-instance"
  instance_size = "medium"
  project_name  = var.project_name
  environment   = var.environment
  
  provider_config = {
    region = "us-west-2"
    ami    = "ami-03d5c68bab01f3496"
  }
}

# ============================================================================
# OUTPUTS
# ============================================================================

output "primary_endpoint" { value = module.primary_compute.public_ip }
output "secondary_endpoint" { value = module.secondary_compute.public_ip }
output "primary_bucket" { value = module.primary_storage.bucket_url }
output "secondary_bucket" { value = module.secondary_storage.bucket_url }
