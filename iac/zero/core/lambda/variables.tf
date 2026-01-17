variable "function_name" {
  description = "Name of the Lambda function"
  type        = string
}

variable "description" {
  description = "Description of what the function does"
  type        = string
  default     = null
}

variable "handler" {
  description = "Function entrypoint (e.g. index.handler)"
  type        = string
}

variable "runtime" {
  description = "Lambda runtime (e.g. nodejs18.x, python3.9)"
  type        = string
  default     = "python3.9"
}

variable "memory_size" {
  description = "Memory size in MB"
  type        = number
  default     = 128
}

variable "timeout" {
  description = "Function timeout in seconds"
  type        = number
  default     = 3
}

# Source Code (Option A: Local file)
variable "filename" {
  description = "Path to the deployment package (zip)"
  type        = string
  default     = null
}

variable "source_code_hash" {
  description = "Base64-encoded SHA256 hash of the package"
  type        = string
  default     = null
}

# Source Code (Option B: S3)
variable "s3_bucket" {
  description = "S3 bucket location containing the package"
  type        = string
  default     = null
}

variable "s3_key" {
  description = "S3 key for the package"
  type        = string
  default     = null
}

variable "environment_variables" {
  description = "Map of environment variables"
  type        = map(string)
  default     = {}
}

# VPC Configuration
variable "vpc_subnet_ids" {
  description = "List of subnet IDs for VPC access"
  type        = list(string)
  default     = null
}

variable "vpc_security_group_ids" {
  description = "List of security group IDs for VPC access"
  type        = list(string)
  default     = null
}

variable "tracing_mode" {
  description = "X-Ray tracing mode (PassThrough or Active)"
  type        = string
  default     = "PassThrough"
}

variable "log_retention_days" {
  description = "CloudWatch log retention in days"
  type        = number
  default     = 14
}

# Triggers
variable "create_apigw_permission" {
  description = "Create permission for API Gateway invoke"
  type        = bool
  default     = false
}

variable "apigw_execution_arn" {
  description = "Execution ARN of the API Gateway"
  type        = string
  default     = null
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
