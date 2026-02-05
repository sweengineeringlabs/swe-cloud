variable "name" {
  description = "Name of the workflow/state machine"
  type        = string
}

variable "definition" {
  description = "The JSON/YAML definition of the workflow"
  type        = string
}

variable "role_arn" {
  description = "IAM role ARN for the workflow execution"
  type        = string
}

variable "type" {
  description = "Workflow type (STANDARD or EXPRESS)"
  type        = string
  default     = "STANDARD"
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
