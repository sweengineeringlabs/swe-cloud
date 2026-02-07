variable "table_name" {
  description = "Name of the NoSQL table"
  type        = string
}

variable "hash_key" {
  description = "Partition key (Hash Key) attribute name"
  type        = string
}

variable "hash_key_type" {
  description = "Partition key type (S, N, B)"
  type        = string
  default     = "S"
}

variable "range_key" {
  description = "Sort key (Range Key) attribute name"
  type        = string
  default     = null
}

variable "range_key_type" {
  description = "Sort key type (S, N, B)"
  type        = string
  default     = "S"
}

variable "billing_mode" {
  description = "Billing mode (PROVISIONED or PAY_PER_REQUEST)"
  type        = string
  default     = "PAY_PER_REQUEST"
}

variable "read_capacity" {
  description = "Read capacity units (only for PROVISIONED)"
  type        = number
  default     = 5
}

variable "write_capacity" {
  description = "Write capacity units (only for PROVISIONED)"
  type        = number
  default     = 5
}

variable "attributes" {
  description = "List of attribute definitions (name and type)"
  type = list(object({
    name = string
    type = string
  }))
  default = []
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
