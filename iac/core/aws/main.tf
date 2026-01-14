# AWS Provider Module
# Groups all AWS resources (compute, storage, database, etc.)

terraform {
  required_version = ">= 1.0"
  
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

# ============================================================================
# COMPUTE RESOURCES
# ============================================================================

# EC2 Instance
resource "aws_instance" "compute" {
  count = var.compute_config != null ? 1 : 0
  
  ami           = var.compute_config.ami
  instance_type = var.compute_config.instance_type
  key_name      = var.compute_config.ssh_key_name
  subnet_id     = var.compute_config.subnet_id
  
  vpc_security_group_ids = var.compute_config.security_group_ids
  iam_instance_profile   = var.compute_config.instance_profile_name
  
  user_data = var.compute_config.user_data
  
  monitoring    = var.compute_config.enable_monitoring
  ebs_optimized = var.compute_config.ebs_optimized
  
  tags = var.compute_config.tags
}

# ============================================================================
# STORAGE RESOURCES
# ============================================================================

# S3 Bucket
resource "aws_s3_bucket" "storage" {
  count = var.storage_config != null ? 1 : 0
  
  bucket        = var.storage_config.bucket_name
  force_destroy = var.storage_config.force_destroy
  
  tags = var.storage_config.tags
}

# S3 Bucket Versioning
resource "aws_s3_bucket_versioning" "storage" {
  count  = var.storage_config != null && var.storage_config.versioning_enabled ? 1 : 0
  bucket = aws_s3_bucket.storage[0].id
  
  versioning_configuration {
    status = "Enabled"
  }
}

# S3 Bucket Encryption
resource "aws_s3_bucket_server_side_encryption_configuration" "storage" {
  count  = var.storage_config != null && var.storage_config.encryption_enabled ? 1 : 0
  bucket = aws_s3_bucket.storage[0].id
  
  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm     = var.storage_config.encryption_key_id != null ? "aws:kms" : "AES256"
      kms_master_key_id = var.storage_config.encryption_key_id
    }
  }
}

# S3 Public Access Block
resource "aws_s3_bucket_public_access_block" "storage" {
  count  = var.storage_config != null && var.storage_config.public_access_block ? 1 : 0
  bucket = aws_s3_bucket.storage[0].id
  
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

# ============================================================================
# OUTPUTS
# ============================================================================

output "compute" {
  description = "Compute resource outputs"
  value = var.compute_config != null ? {
    instance_id       = aws_instance.compute[0].id
    arn               = aws_instance.compute[0].arn
    public_ip         = aws_instance.compute[0].public_ip
    private_ip        = aws_instance.compute[0].private_ip
    instance_state    = aws_instance.compute[0].instance_state
    availability_zone = aws_instance.compute[0].availability_zone
  } : null
}

output "storage" {
  description = "Storage resource outputs"
  value = var.storage_config != null ? {
    bucket_id          = aws_s3_bucket.storage[0].id
    bucket_arn         = aws_s3_bucket.storage[0].arn
    bucket_domain_name = aws_s3_bucket.storage[0].bucket_domain_name
    region             = aws_s3_bucket.storage[0].region
  } : null
}
