variable "ami" {
  description = "AMI ID for the EC2 instance"
  type        = string
}

variable "instance_type" {
  description = "Instance type (e.g., t3.medium)"
  type        = string
}

variable "ssh_key_name" {
  description = "SSH key pair name"
  type        = string
  default     = null
}

variable "subnet_id" {
  description = "Subnet ID"
  type        = string
  default     = null
}

variable "security_group_ids" {
  description = "Security group IDs"
  type        = list(string)
  default     = []
}

variable "instance_profile_name" {
  description = "IAM instance profile name"
  type        = string
  default     = null
}

variable "user_data" {
  description = "User data script"
  type        = string
  default     = null
}

variable "enable_monitoring" {
  description = "Enable detailed monitoring"
  type        = bool
  default     = true
}

variable "ebs_optimized" {
  description = "Enable EBS optimization"
  type        = bool
  default     = false
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
