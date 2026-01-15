module "aws_lambda" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../iac_core/aws/src/lambda"

  function_name = var.function_name
  handler       = var.handler
  runtime       = var.runtime

  # Source Code handling
  filename = var.source_code != null ? data.archive_file.lambda_zip[0].output_path : null
  
  environment_variables = var.environment_variables
  
  # Map other variables
  tags = merge(var.tags, {
    Environment = var.environment
    Project     = var.project_name
  })
}

locals {
  is_python = length(regexall("python", var.runtime)) > 0
  file_ext  = local.is_python ? "py" : "js"
}

data "archive_file" "lambda_zip" {
  count       = var.source_code != null ? 1 : 0
  type        = "zip"
  output_path = "${path.module}/lambda_${var.function_name}.zip"

  source {
    content  = var.source_code
    filename = "index.${local.file_ext}"
  }
}

output "function_arn" {
  value = var.provider_name == "aws" ? module.aws_lambda[0].function_arn : "placeholder-arn"
}

output "function_name" {
  value = var.function_name
}
