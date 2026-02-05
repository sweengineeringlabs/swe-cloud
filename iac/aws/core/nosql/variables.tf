variable "table_name" {
  description = "Name of the DynamoDB table"
  type        = string
}

variable "hash_key" {
  description = "Partition key"
  type        = string
}

variable "hash_key_type" {
  description = "Partition key type"
  type        = string
  default     = "S"
}

variable "range_key" {
  description = "Sort key"
  type        = string
  default     = null
}

variable "range_key_type" {
  description = "Sort key type"
  type        = string
  default     = "S"
}

variable "billing_mode" {
  description = "Billing mode"
  type        = string
}

variable "read_capacity" {
  description = "Read capacity"
  type        = number
}

variable "write_capacity" {
  description = "Write capacity"
  type        = number
}

variable "attributes" {
  description = "Additional attributes"
  type = list(object({
    name = string
    type = string
  }))
  default = []
}

variable "tags" {
  description = "Tags"
  type        = map(string)
  default     = {}
}
