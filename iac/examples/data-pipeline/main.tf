# Multi-Cloud Data Pipeline Example
# Demonstrates usage of Networking, Compute, Storage, and Database facades

terraform {
  required_version = ">= 1.0"
}

variable "environment" {
  type    = string
  default = "dev"
}

variable "project_name" {
  type    = string
  default = "data-pipe-demo"
}

# ============================================================================
# AWS DEPLOYMENT (Primary)
# ============================================================================

# 1. Networking
module "network_aws" {
  source = "../../facade/networking"
  
  provider_name = "aws"
  project_name = var.project_name
  environment  = var.environment
  network_name = "${var.project_name}-net"
  
  metrics = {
    cidr            = "10.0.0.0/16"
    azs             = ["us-east-1a", "us-east-1b"]
    public_subnets  = ["10.0.1.0/24", "10.0.2.0/24"]
    private_subnets = ["10.0.10.0/24", "10.0.11.0/24"]
  }
}

# 2. Ingestion (Storage)
module "ingestion_bucket" {
  source = "../../facade/storage"
  
  provider_name = "aws"
  bucket_name  = "ingest-${var.project_name}-${var.environment}"
  project_name = var.project_name
  environment  = var.environment
  
  storage_class      = "standard"
  versioning_enabled = true
}

# 3. Processing (Compute)
module "processor" {
  source = "../../facade/compute"
  
  provider_name = "aws"
  instance_name = "processor-${var.environment}"
  instance_size = "medium"
  project_name  = var.project_name
  environment   = var.environment
  
  network_id    = module.network_aws.network_id
  
  tags = {
    Role = "DataProcessor"
  }
}

# 4. Storage (Database)
module "metadata_db" {
  source = "../../facade/database"
  
  provider_name = "aws"
  identifier   = "meta-${var.project_name}"
  project_name = var.project_name
  environment  = var.environment
  
  engine          = "postgres"
  instance_class  = "small"
  master_username = "admin"
  master_password = "SecurePassword123!" # In real usage, use secrets manager
  
  allocated_storage_gb = 20
  
  provider_config = {
    subnet_group = "default" # Simplified for example
  }
}

# ============================================================================
# OUTPUTS
# ============================================================================

output "ingestion_url" {
  value = module.ingestion_bucket.bucket_url
}

output "processor_ip" {
  value = module.processor.public_ip
}

output "db_endpoint" {
  value = module.metadata_db.db_endpoint
}
