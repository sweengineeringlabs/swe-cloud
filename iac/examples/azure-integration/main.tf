# Azure Integration Testing Example
terraform {
  required_version = ">= 1.5.0"
  
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

provider "azurerm" {
  features {}
  skip_provider_registration = true
  storage_use_azuread        = false
  
  # CloudEmu Azure endpoint
  metadata_host = "http://localhost:10000"
}

# 1. Storage Resource (Blob)
module "storage" {
  source = "../../facade/storage"
  
  provider_name = "azure"
  bucket_name   = var.bucket_name
  project_name  = "azure-test"
  environment   = var.environment
  
  # CloudEmu-specific
  versioning_enabled = true
}

# 2. NoSQL Resource (Cosmos DB)
module "nosql" {
  source = "../../facade/nosql"
  
  provider_name = "azure"
  table_name    = var.table_name
  hash_key      = "id"
  project_name  = "azure-test"
  environment   = var.environment
}

# Variables
variable "bucket_name" {
  type    = string
  default = "test-azure-container"
}

variable "table_name" {
  type    = string
  default = "test-azure-cosmos"
}

variable "environment" {
  type    = string
  default = "test"
}

# Outputs
output "bucket_name" {
  value = module.storage.bucket.name
}

output "bucket_url" {
  value = module.storage.bucket_url
}

output "table_name" {
  value = var.table_name
}
