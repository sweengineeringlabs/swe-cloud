# AWS IAM (Identity and Access Management)
# Mirrors CloudKit's cloudkit_core/aws/src/iam.rs pattern

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

# IAM Role
resource "aws_iam_role" "this" {
  count = var.create_role ? 1 : 0
  
  name        = var.role_name
  description = var.role_description
  
  assume_role_policy = var.assume_role_policy != null ? var.assume_role_policy : jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = var.trusted_services
        }
      }
    ]
  })
  
  max_session_duration = var.max_session_duration
  
  tags = var.tags
}

# IAM Policy
resource "aws_iam_policy" "this" {
  count = var.create_policy ? 1 : 0
  
  name        = var.policy_name
  description = var.policy_description
  policy      = var.policy_document
  
  tags = var.tags
}

# Attach policy to role
resource "aws_iam_role_policy_attachment" "custom" {
  count = var.create_role && var.create_policy ? 1 : 0
  
  role       = aws_iam_role.this[0].name
  policy_arn = aws_iam_policy.this[0].arn
}

# Attach managed policies to role
resource "aws_iam_role_policy_attachment" "managed" {
  count = var.create_role ? length(var.managed_policy_arns) : 0
  
  role       = aws_iam_role.this[0].name
  policy_arn = var.managed_policy_arns[count.index]
}

# Instance Profile (for EC2)
resource "aws_iam_instance_profile" "this" {
  count = var.create_instance_profile && var.create_role ? 1 : 0
  
  name = "${var.role_name}-profile"
  role = aws_iam_role.this[0].name
  
  tags = var.tags
}

# IAM User
resource "aws_iam_user" "this" {
  count = var.create_user ? 1 : 0
  
  name = var.user_name
  path = var.user_path
  
  tags = var.tags
}

# User policy attachment
resource "aws_iam_user_policy_attachment" "this" {
  count = var.create_user ? length(var.user_policy_arns) : 0
  
  user       = aws_iam_user.this[0].name
  policy_arn = var.user_policy_arns[count.index]
}

# Access keys for user
resource "aws_iam_access_key" "this" {
  count = var.create_user && var.create_access_key ? 1 : 0
  
  user = aws_iam_user.this[0].name
}

# Outputs
output "role_arn" {
  description = "ARN of the IAM role"
  value       = var.create_role ? aws_iam_role.this[0].arn : null
}

output "role_name" {
  description = "Name of the IAM role"
  value       = var.create_role ? aws_iam_role.this[0].name : null
}

output "role_id" {
  description = "ID of the IAM role"
  value       = var.create_role ? aws_iam_role.this[0].id : null
}

output "policy_arn" {
  description = "ARN of the IAM policy"
  value       = var.create_policy ? aws_iam_policy.this[0].arn : null
}

output "policy_id" {
  description = "ID of the IAM policy"
  value       = var.create_policy ? aws_iam_policy.this[0].id : null
}

output "instance_profile_arn" {
  description = "ARN of the instance profile"
  value       = var.create_instance_profile && var.create_role ? aws_iam_instance_profile.this[0].arn : null
}

output "instance_profile_name" {
  description = "Name of the instance profile"
  value       = var.create_instance_profile && var.create_role ? aws_iam_instance_profile.this[0].name : null
}

output "user_arn" {
  description = "ARN of the IAM user"
  value       = var.create_user ? aws_iam_user.this[0].arn : null
}

output "user_name" {
  description = "Name of the IAM user"
  value       = var.create_user ? aws_iam_user.this[0].name : null
}

output "access_key_id" {
  description = "Access key ID"
  value       = var.create_user && var.create_access_key ? aws_iam_access_key.this[0].id : null
  sensitive   = true
}

output "secret_access_key" {
  description = "Secret access key"
  value       = var.create_user && var.create_access_key ? aws_iam_access_key.this[0].secret : null
  sensitive   = true
}
