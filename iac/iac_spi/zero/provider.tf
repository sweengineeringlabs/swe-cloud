# ZeroCloud Provider Configuration (via AWS Provider)
# SPI Layer - Service Provider Interface

terraform {
  required_version = ">= 1.0"
  
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  alias  = "zero"
  region = "local"
  
  # ZeroCloud Control Plane Endpoints
  endpoints {
    s3       = var.zero_endpoint_store    # Default: http://localhost:8080/v1/store
    dynamodb = var.zero_endpoint_db       # Default: http://localhost:8080/v1/db
    lambda   = var.zero_endpoint_func     # Default: http://localhost:8080/v1/func
    sqs      = var.zero_endpoint_queue    # Default: http://localhost:8080/v1/queue
    iam      = var.zero_endpoint_iam      # Default: http://localhost:8080/v1/iam
  }

  skip_credentials_validation = true
  skip_requesting_account_id  = true
  skip_metadata_api_check     = true
  s3_use_path_style           = true
}

locals {
  zero_tags = merge(var.common_tags, {
    Provider = "ZeroCloud"
  })
}
