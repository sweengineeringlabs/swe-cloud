variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "name" {
  description = "Workflow name"
  type        = string
}

variable "region" {
  description = "GCP region"
  type        = string
  default     = "us-central1"
}

variable "source_contents" {
  description = "Workflow source contents (YAML)"
  type        = string
}

variable "labels" {
  description = "Resource labels"
  type        = map(string)
  default     = {}
}
