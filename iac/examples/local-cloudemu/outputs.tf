# Outputs from CloudEmu testing

# Storage outputs
output "bucket_name" {
  description = "Name of the created S3 bucket"
  value       = module.storage.bucket_name
}

output "bucket_arn" {
  description = "ARN of the created S3 bucket"
  value       = module.storage.bucket_arn
}

output "bucket_endpoint" {
  description = "Endpoint URL for the S3 bucket"
  value       = module.storage.endpoint
}

# Database outputs
output "table_name" {
  description = "Name of the created DynamoDB table"
  value       = module.database.table_name
}

output "table_arn" {
  description = "ARN of the created DynamoDB table"
  value       = module.database.table_arn
}

# Messaging outputs
output "queue_url" {
  description = "URL of the created SQS queue"
  value       = module.messaging.queue_url
}

output "topic_arn" {
  description = "ARN of the created SNS topic"
  value       = module.messaging.topic_arn
}

# Lambda outputs
output "function_name" {
  description = "Name of the created Lambda function"
  value       = module.lambda.function_name
}

output "function_arn" {
  description = "ARN of the created Lambda function"
  value       = module.lambda.function_arn
}

# CloudEmu connection info
output "cloudemu_endpoint" {
  description = "CloudEmu AWS endpoint URL"
  value       = "http://localhost:4566"
}

output "verification_commands" {
  description = "Commands to verify resources in CloudEmu"
  value = {
    list_buckets   = "aws --endpoint-url=http://localhost:4566 s3 ls"
    list_tables    = "aws --endpoint-url=http://localhost:4566 dynamodb list-tables"
    list_queues    = "aws --endpoint-url=http://localhost:4566 sqs list-queues"
    list_topics    = "aws --endpoint-url=http://localhost:4566 sns list-topics"
    list_functions = "aws --endpoint-url=http://localhost:4566 lambda list-functions"
  }
}
