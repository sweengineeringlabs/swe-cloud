variable "name" {
  description = "Name of the event bus or topic"
  type        = string
}

variable "description" {
  description = "Description of the event resource"
  type        = string
  default     = null
}

variable "event_bus_name" {
  description = "Name of the event bus (for AWS)"
  type        = string
  default     = "default"
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
