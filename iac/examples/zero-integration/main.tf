terraform {
  required_version = ">= 1.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

# ZeroCloud Provider Configuration (SPI)
# Redirects AWS protocol requests to local ZeroCloud endpoints
provider "aws" {
  region                      = "us-east-1"
  skip_credentials_validation = true
  skip_requesting_account_id  = true
  skip_metadata_api_check     = true
  s3_use_path_style           = true
  
  endpoints {
    ec2        = "http://localhost:8080"
    s3         = "http://localhost:8080"
    dynamodb   = "http://localhost:8080"
    lambda     = "http://localhost:8080"
    sqs        = "http://localhost:8080"
    iam        = "http://localhost:8080"
    sts        = "http://localhost:8080"
  }
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

# 3. Networking Resource (ZeroNet)
module "networking" {
  source        = "../../facade/networking"
  provider_name = "zero"
  network_name  = "zero-vpc"
  
  metrics = {
    cidr            = "10.0.0.0/16"
    azs             = ["us-east-1a", "us-east-1b"]
    public_subnets  = ["10.0.1.0/24", "10.0.2.0/24"]
    private_subnets = ["10.0.3.0/24", "10.0.4.0/24"]
  }
  
  project_name  = "zero-test-project"
  environment   = var.environment
}

# 4. Identity Resource (ZeroID)
module "iam" {
  source        = "../../facade/iam"
  provider_name = "zero"
  identity_type = "role"
  identity_name = "zero-app-role"
  principals    = ["lambda.amazonaws.com"] # ZeroFunc uses AWS style principals
  roles         = ["storage_read", "nosql_write"]
  
  project_name  = "zero-test-project"
  environment   = var.environment
}

# 5. Compute Resource (ZeroFunc)
module "lambda" {
  source        = "../../facade/lambda"
  provider_name = "zero"
  function_name = "zero-test-func"
  handler       = "index.handler"
  runtime       = "nodejs18.x"
  
  # Basic inline code for testing
  # source_code   = "exports.handler = async (event) => { return 'Hello from ZeroFunc'; };"
  
  project_name  = "zero-test-project"
  environment   = var.environment
}

# 6. Messaging Resource (ZeroQueue)
module "queue" {
  source        = "../../facade/messaging"
  provider_name = "zero"
  type          = "queue"
  name          = "zero-test-queue"
  
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

output "vpc_id" {
  value = module.networking.network_id
}

output "role_arn" {
  value = module.iam.principal_id
}

output "function_arn" {
  value = module.lambda.function_arn
}

output "queue_url" {
  value = module.queue.resource_url
}
