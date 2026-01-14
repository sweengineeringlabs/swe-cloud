module "aws_lambda" {
  count  = var.provider == "aws" ? 1 : 0
  source = "../../iac_core/aws/src/lambda"

  function_name = var.function_name
  handler       = var.handler
  runtime       = var.runtime
  
  # Map other variables
  tags = merge(var.tags, {
    Environment = var.environment
    Project     = var.project_name
  })
}

# Azure and GCP would be similar if core modules existed for them.
# For now we route to AWS or provide placeholders.

output "function_arn" {
  value = var.provider == "aws" ? module.aws_lambda[0].function_arn : "placeholder-arn"
}

output "function_name" {
  value = var.function_name
}
