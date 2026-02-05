variable "name" {
  description = "Alias name for the KMS key"
  type        = string
}

variable "description" {
  description = "Description"
  type        = string
  default     = null
}

variable "deletion_window_in_days" {
  description = "Deletion window"
  type        = number
  default     = 7
}

variable "tags" {
  description = "Tags"
  type        = map(string)
  default     = {}
}
