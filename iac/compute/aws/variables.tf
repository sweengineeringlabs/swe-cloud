variable "instance_name" {
  description = "The name of the AWS instance."
  type        = string
  default     = "ExampleAWSCompute"
}

variable "ami" {
  description = "The AMI to use for the instance."
  type        = string
}

variable "instance_type" {
  description = "The type of instance to start."
  type        = string
}
