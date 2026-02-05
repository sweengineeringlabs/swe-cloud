# AWS Storage (S3)
# Mirrors CloudKit's cloudkit_core/aws/s3.rs

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

resource "aws_s3_bucket" "this" {
  bucket        = var.bucket_name
  force_destroy = var.force_destroy
  tags          = var.tags
}

resource "aws_s3_bucket_versioning" "this" {
  count  = var.versioning_enabled ? 1 : 0
  bucket = aws_s3_bucket.this.id
  
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "this" {
  count  = var.encryption_enabled ? 1 : 0
  bucket = aws_s3_bucket.this.id
  
  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm     = var.encryption_key_id != null ? "aws:kms" : "AES256"
      kms_master_key_id = var.encryption_key_id
    }
  }
}

resource "aws_s3_bucket_public_access_block" "this" {
  count  = var.public_access_block ? 1 : 0
  bucket = aws_s3_bucket.this.id
  
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

output "bucket_id" {
  value = aws_s3_bucket.this.id
}

output "bucket_arn" {
  value = aws_s3_bucket.this.arn
}

output "bucket_domain_name" {
  value = aws_s3_bucket.this.bucket_domain_name
}

output "region" {
  value = aws_s3_bucket.this.region
}
