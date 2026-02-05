variable "provider_name" {
  description = "Cloud provider"
  type        = string
}

variable "name" {
  description = "Workflow name"
  type        = string
}

variable "definition" {
  description = "Workflow definition"
  type        = string
}

variable "role_arn" {
  description = "IAM Role ARN"
  type        = string
}

variable "environment" {
  description = "Environment"
  type        = string
}

variable "project_name" {
  description = "Project name"
  type        = string
}

variable "tags" {
  description = "Additional tags"
  type        = map(string)
  default     = {}
}
