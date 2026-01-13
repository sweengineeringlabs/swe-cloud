# variables.tf for the 'test' provider

variable "instance_name" {
  description = "The name of the test instance."
  type        = string
  default     = "test-instance"
}

variable "instance_type" {
  description = "The type of test instance."
  type        = string
  default     = "test-small"
}

# This module accepts other variables but does nothing with them.
# This is to ensure the facade can pass them without error.
variable "ami" {
  type    = any
  default = null
}

variable "machine_type" {
  type    = any
  default = null
}

variable "project_id" {
  type    = any
  default = null
}

variable "zone" {
  type    = any
  default = null
}

variable "vm_size" {
  type    = any
  default = null
}

variable "location" {
  type    = any
  default = null
}

variable "resource_group_name" {
  type    = any
  default = null
}

variable "admin_password" {
  type    = any
  default = null
}
