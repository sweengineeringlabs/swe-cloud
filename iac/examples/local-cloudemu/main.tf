# Local CloudEmu Testing Example
#
# This configuration demonstrates using IAC modules with CloudEmu
# for local testing without AWS costs.

terraform {
  required_version = ">= 1.5.0"
  
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

# Configure AWS provider to use CloudEmu endpoints
provider "aws" {
  region = var.aws_region
  
  # CloudEmu endpoints for all services
  endpoints {
    s3             = "http://localhost:4566"
    dynamodb       = "http://localhost:4566"
    sqs            = "http://localhost:4566"
    sns            = "http://localhost:4566"
    lambda         = "http://localhost:4566"
    kms            = "http://localhost:4566"
    secretsmanager = "http://localhost:4566"
    cloudwatch     = "http://localhost:4566"
    events         = "http://localhost:4566"
    sts            = "http://localhost:4566"
    iam            = "http://localhost:4566"
  }
  
  # Skip AWS API validation (not needed for CloudEmu)
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true
  
  # Use path-style S3 URLs (required for CloudEmu)
  s3_use_path_style = true
  
  # Dummy credentials (CloudEmu doesn't validate)
  access_key = "test"
  secret_key = "test"
}

# Storage Facade Example
module "storage" {
  source = "../../facade/storage"
  
  provider_name = "aws"
  project_name  = "local-test"
  bucket_name   = var.bucket_name
  environment   = var.environment
  
  # CloudEmu-specific settings
  versioning_enabled = true
  encryption_enabled = true
}

# NoSQL Facade Example (DynamoDB)
module "nosql_table" {
  source = "../../facade/nosql"
  
  provider_name = "aws"
  project_name  = "local-test"
  table_name    = var.database_name # Reusing the variable name for simplicity
  environment   = var.environment
  
  hash_key      = "id"
  hash_key_type = "S"
}

# Messaging Facade Example (SQS + SNS)
module "queue" {
  source = "../../facade/messaging"
  
  provider_name = "aws"
  name          = var.queue_name
  type          = "queue"
  project_name  = "local-test"
  environment   = var.environment
}

module "topic" {
  source = "../../facade/messaging"
  
  provider_name = "aws"
  name          = var.topic_name
  type          = "topic"
  project_name  = "local-test"
  environment   = var.environment
}

# Lambda Facade Example
module "lambda" {
  source = "../../facade/lambda"
  
  provider_name    = "aws"
  project_name     = "local-test"
  function_name    = var.function_name
  runtime          = "python3.11"
  handler          = "index.handler"
  environment_variables = {
    Environment = var.environment
  }
  
  # Simple test function
  source_code = <<-EOT
    def handler(event, context):
        return {
            'statusCode': 200,
            'body': 'Hello from CloudEmu!'
        }
  EOT
}
