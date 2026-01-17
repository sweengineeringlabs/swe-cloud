terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

variable "create_role" { type = bool }
variable "role_name" { type = string }
variable "create_user" { type = bool }
variable "user_name" { type = string }
variable "trusted_services" { type = list(string) }
variable "tags" { type = map(string) }

# Reuse AWS Provider for ZeroID (redirected via SPI)
resource "aws_iam_role" "this" {
  count = var.create_role ? 1 : 0
  name  = var.role_name
  
  assume_role_policy = jsonencode({
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

  tags = var.tags
}

resource "aws_iam_user" "this" {
  count = var.create_user ? 1 : 0
  name  = var.user_name
  tags  = var.tags
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

output "user_arn" {
  description = "ARN of the IAM user"
  value       = var.create_user ? aws_iam_user.this[0].arn : null
}

output "user_name" {
  description = "Name of the IAM user"
  value       = var.create_user ? aws_iam_user.this[0].name : null
}
