variable "name" {
  description = "Name of the Step Function"
  type        = string
}

variable "definition" {
  description = "ASL Definition"
  type        = string
}

variable "role_arn" {
  description = "IAM Role"
  type        = string
}

variable "type" {
  description = "Execution type"
  type        = string
  default     = "STANDARD"
}

variable "tags" {
  description = "Tags"
  type        = map(string)
  default     = {}
}
