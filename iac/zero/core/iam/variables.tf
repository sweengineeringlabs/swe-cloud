# IAM Role variables
variable "create_role" {
  description = "Create IAM role"
  type        = bool
  default     = false
}

variable "role_name" {
  description = "Name of the IAM role"
  type        = string
  default     = null
}

variable "role_description" {
  description = "Description of the IAM role"
  type        = string
  default     = null
}

variable "assume_role_policy" {
  description = "Custom assume role policy (JSON)"
  type        = string
  default     = null
}

variable "trusted_services" {
  description = "AWS services that can assume this role"
  type        = list(string)
  default     = ["ec2.amazonaws.com"]
}

variable "max_session_duration" {
  description = "Maximum session duration in seconds"
  type        = number
  default     = 3600
}

# IAM Policy variables
variable "create_policy" {
  description = "Create IAM policy"
  type        = bool
  default     = false
}

variable "policy_name" {
  description = "Name of the IAM policy"
  type        = string
  default     = null
}

variable "policy_description" {
  description = "Description of the IAM policy"
  type        = string
  default     = null
}

variable "policy_document" {
  description = "IAM policy document (JSON)"
  type        = string
  default     = null
}

variable "managed_policy_arns" {
  description = "List of managed policy ARNs to attach to role"
  type        = list(string)
  default     = []
}

# Instance Profile variables
variable "create_instance_profile" {
  description = "Create instance profile for EC2"
  type        = bool
  default     = false
}

# IAM User variables
variable "create_user" {
  description = "Create IAM user"
  type        = bool
  default     = false
}

variable "user_name" {
  description = "Name of the IAM user"
  type        = string
  default     = null
}

variable "user_path" {
  description = "Path for the IAM user"
  type        = string
  default     = "/"
}

variable "user_policy_arns" {
  description = "List of policy ARNs to attach to user"
  type        = list(string)
  default     = []
}

variable "create_access_key" {
  description = "Create access key for user"
  type        = bool
  default     = false
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
