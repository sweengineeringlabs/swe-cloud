# AWS Provider Configuration
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
  region = var.aws_region

  # Apply default tags to all resources
  default_tags {
    tags = local.aws_tags
  }

  # Assume role if provided (for cross-account access)
  dynamic "assume_role" {
    for_each = var.aws_assume_role != null ? [1] : []
    content {
      role_arn     = var.aws_assume_role.role_arn
      session_name = var.aws_assume_role.session_name
      external_id  = try(var.aws_assume_role.external_id, null)
    }
  }

  # Allow retries for transient failures
  retry_mode = "standard"
  max_retries = 3
}

# Local values for AWS-specific configurations
locals {
  # Import common tags and format for AWS
  aws_tags = var.common_tags

  # AWS-specific settings
  aws_config = {
    region      = var.aws_region
    environment = var.environment
    project     = var.project_name
  }
}
