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

# 3. Networking Resource (VPC)
module "networking" {
  source = "../../facade/networking"
  
  provider_name = "gcp"
  network_name  = "gcp-vpc"
  
  metrics = {
    cidr            = "10.0.0.0/16"
    azs             = ["us-east1-b", "us-east1-c"]
    public_subnets  = ["10.0.1.0/24", "10.0.2.0/24"]
    private_subnets = ["10.0.3.0/24", "10.0.4.0/24"]
  }
  
  project_name  = "gcp-test"
  environment   = var.environment
}

# 4. Identity Resource (Service Account)
module "iam" {
  source = "../../facade/iam"
  
  provider_name = "gcp"
  identity_type = "service_agent"
  identity_name = "gcp-test-sa"
  principals    = []
  
  project_name  = "gcp-test"
  environment   = var.environment
}

# 5. Compute Resource (Cloud Function)
module "lambda" {
  source = "../../facade/lambda"
  
  provider_name = "gcp"
  function_name = "gcp-test-func"
  handler       = "main.handler"
  runtime       = "python3.11"
  
  filename      = "${path.module}/files/test_function.zip"
  
  project_name  = "gcp-test"
  environment   = var.environment
}

# 6. Messaging Resource (Pub/Sub)
module "queue" {
  source = "../../facade/messaging"
  
  provider_name = "gcp"
  type          = "topic" # Start with Topic for Pub/Sub
  name          = "gcp-test-topic"
  
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

output "vpc_id" {
  value = module.networking.network_id
}

output "sa_email" {
  value = module.iam.principal_id
}

output "function_name" {
  value = module.lambda.function_name
}

output "topic_arn" {
  value = module.queue.resource_arn
}
