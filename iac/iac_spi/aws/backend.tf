# AWS S3 Backend Configuration
# SPI Layer - Remote state storage

terraform {
  backend "s3" {
    # State file configuration
    bucket = var.state_bucket
    key    = "${var.environment}/${var.project_name}/terraform.tfstate"
    region = var.aws_region

    # Encryption
    encrypt        = true
    kms_key_id     = var.state_kms_key_id

    # State locking
    dynamodb_table = var.state_lock_table

    # Versioning (recommended for state recovery)
    # Note: Enable versioning on the S3 bucket itself
    
    # Access logging
    # Note: Configure bucket logging separately for audit trails
  }
}

# Variables for backend configuration
variable "state_bucket" {
  description = "S3 bucket for Terraform state"
  type        = string
}

variable "state_lock_table" {
  description = "DynamoDB table for state locking"
  type        = string
  default     = "terraform-state-locks"
}

variable "state_kms_key_id" {
  description = "KMS key ID for state encryption"
  type        = string
  default     = null
}
