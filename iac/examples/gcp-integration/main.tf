# GCP Integration Testing Example
terraform {
  required_version = ">= 1.5.0"
  
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
  }
}

provider "google" {
  project = "local-test"
  region  = "us-east1"
  
  # CloudEmu GCP endpoints
  storage_custom_endpoint   = "http://localhost:4567"
  firestore_custom_endpoint = "http://localhost:4567/firestore/"
  pubsub_custom_endpoint    = "http://localhost:4567/"
}

# 1. Storage Resource (GCS)
module "storage" {
  source = "../../facade/storage"
  
  provider_name = "gcp"
  bucket_name   = var.bucket_name
  project_name  = "gcp-test"
  environment   = var.environment
  
  versioning_enabled = true
}

# 2. NoSQL Resource (Firestore)
module "nosql" {
  source = "../../facade/nosql"
  
  provider_name = "gcp"
  table_name    = var.table_name
  hash_key      = "id"
  project_name  = "gcp-test"
  environment   = var.environment
}

# Variables
variable "bucket_name" {
  type    = string
  default = "test-gcp-bucket"
}

variable "table_name" {
  type    = string
  default = "test-gcp-collection"
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
