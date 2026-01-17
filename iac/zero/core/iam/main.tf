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
