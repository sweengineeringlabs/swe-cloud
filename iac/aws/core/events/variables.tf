variable "name" {
  description = "Name of the event bus"
  type        = string
}

variable "description" {
  description = "Description"
  type        = string
  default     = null
}

variable "tags" {
  description = "Tags"
  type        = map(string)
  default     = {}
}
