terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

variable "function_name" { type = string }
variable "handler" { type = string }
variable "runtime" { type = string }
variable "filename" { type = string }
variable "environment_variables" { type = map(string) }
variable "tags" { type = map(string) }

# Reuse AWS Provider for ZeroFunc (redirected via SPI)
resource "aws_lambda_function" "this" {
  filename      = var.filename
  function_name = var.function_name
  role          = "arn:aws:iam::000000000000:role/lambda-role" # ZeroCloud mock role
  handler       = var.handler

  runtime = var.runtime

  environment {
    variables = var.environment_variables
  }
  
  tags = var.tags
}
