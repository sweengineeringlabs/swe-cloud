variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "topic_name" {
  description = "PubSub topic name"
  type        = string
}

variable "labels" {
  description = "Resource labels"
  type        = map(string)
  default     = {}
}
