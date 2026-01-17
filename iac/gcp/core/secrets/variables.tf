variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "secret_id" {
  description = "Secret ID"
  type        = string
}

variable "secret_data" {
  description = "Secret data"
  type        = string
  sensitive   = true
}

variable "labels" {
  description = "Resource labels"
  type        = map(string)
  default     = {}
}
