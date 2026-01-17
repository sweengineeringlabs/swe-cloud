variable "name" {
  description = "Name for the encryption key alias"
  type        = string
}

variable "description" {
  description = "Description of the key"
  type        = string
  default     = "Unified encryption key"
}

variable "deletion_window_in_days" {
  description = "Duration in days after which the key is deleted"
  type        = number
  default     = 30
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
