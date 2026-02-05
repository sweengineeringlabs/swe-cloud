variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "key_ring_name" {
  description = "Key ring name"
  type        = string
}

variable "key_name" {
  description = "Crypto key name"
  type        = string
}

variable "location" {
  description = "GCP region"
  type        = string
  default     = "global"
}

variable "labels" {
  description = "Resource labels"
  type        = map(string)
  default     = {}
}
