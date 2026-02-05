# AWS SPI Layer Variables

# ============================================================================
# PROVIDER CONFIGURATION
# ============================================================================

variable "aws_region" {
  description = "AWS region for resources"
  type        = string
  default     = "us-east-1"
  validation {
    condition     = can(regex("^[a-z]{2}-[a-z]+-[0-9]{1}$", var.aws_region))
    error_message = "AWS region must be in format: xx-region-n (e.g., us-east-1)"
  }
}

variable "aws_profile" {
  description = "AWS CLI profile to use"
  type        = string
  default     = null
}

variable "aws_assume_role" {
  description = "AWS IAM role to assume"
  type = object({
    role_arn     = string
    session_name = string
    external_id  = optional(string)
  })
  default = null
}

# ============================================================================
# COMMON VARIABLES (from common layer)
# ============================================================================

variable "environment" {
  description = "Environment name"
  type        = string
}

variable "project_name" {
  description = "Project name"
  type        = string
}

variable "common_tags" {
  description = "Common tags from common layer"
  type        = map(string)
  default     = {}
}
