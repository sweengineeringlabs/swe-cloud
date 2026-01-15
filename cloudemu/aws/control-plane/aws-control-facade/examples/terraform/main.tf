# CloudEmu Terraform Example
#
# This example shows how to use Terraform with CloudEmu to:
# 1. Create an S3 bucket
# 2. Enable versioning
# 3. Set a bucket policy
# 4. Upload objects
#
# Usage:
#   1. Start CloudEmu Server: cargo run -p cloudemu_server
#   2. Run Terraform:
#      cd cloudemu/aws/control-plane/aws-control-facade/examples/terraform
#      terraform init
#      terraform plan
#      terraform apply

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

# Configure AWS provider to use CloudEmu
provider "aws" {
  region                      = "us-east-1"
  access_key                  = "test"
  secret_key                  = "test"
  
  # Point to CloudEmu
  endpoints {
    s3 = "http://localhost:4566"
  }
  
  # Skip AWS-specific validations
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true
  
  # Use path-style addressing (required for local emulation)
  s3_use_path_style = true
}

# Create an S3 bucket
resource "aws_s3_bucket" "my_bucket" {
  bucket = "my-terraform-bucket"
  
  tags = {
    Name        = "My Terraform Bucket"
    Environment = "Development"
    ManagedBy   = "Terraform"
  }
}

# Enable versioning on the bucket
resource "aws_s3_bucket_versioning" "my_bucket_versioning" {
  bucket = aws_s3_bucket.my_bucket.id
  
  versioning_configuration {
    status = "Enabled"
  }
}

# Set a bucket policy
resource "aws_s3_bucket_policy" "my_bucket_policy" {
  bucket = aws_s3_bucket.my_bucket.id
  
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid       = "PublicReadGetObject"
        Effect    = "Allow"
        Principal = "*"
        Action    = "s3:GetObject"
        Resource  = "${aws_s3_bucket.my_bucket.arn}/*"
      }
    ]
  })
}

# Upload a sample object
resource "aws_s3_object" "hello_world" {
  bucket       = aws_s3_bucket.my_bucket.id
  key          = "hello.txt"
  content      = "Hello, World! This was deployed with Terraform to CloudEmu."
  content_type = "text/plain"
}

resource "aws_s3_object" "config_json" {
  bucket       = aws_s3_bucket.my_bucket.id
  key          = "config/app.json"
  content      = jsonencode({
    name    = "my-app"
    version = "1.0.0"
    debug   = true
  })
  content_type = "application/json"
}

# Outputs
output "bucket_name" {
  value = aws_s3_bucket.my_bucket.id
}

output "bucket_arn" {
  value = aws_s3_bucket.my_bucket.arn
}

output "hello_object_key" {
  value = aws_s3_object.hello_world.key
}

output "versioning_status" {
  value = aws_s3_bucket_versioning.my_bucket_versioning.versioning_configuration[0].status
}
