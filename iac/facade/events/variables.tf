variable "provider_name" {
  description = "Cloud provider"
  type        = string
}

variable "name" {
  description = "Event bus name"
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
