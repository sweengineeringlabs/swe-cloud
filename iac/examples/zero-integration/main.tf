terraform {
  required_version = ">= 1.0"
}

# 1. Storage Resource (ZeroStore)
module "storage" {
  source        = "../../facade/storage"
  provider_name = "zero"
  bucket_name   = var.bucket_name
  project_name  = "zero-test-project"
  environment   = var.environment
}

# 2. NoSQL Resource (ZeroDB)
module "nosql" {
  source        = "../../facade/nosql"
  provider_name = "zero"
  table_name    = var.table_name
  hash_key      = "id"
  project_name  = "zero-test-project"
  environment   = var.environment
}

# Variables
variable "bucket_name" {
  type    = string
  default = "test-zero-bucket"
}

variable "table_name" {
  type    = string
  default = "test-zero-table"
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
