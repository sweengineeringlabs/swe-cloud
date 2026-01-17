# Zero NoSQL Variables

variable "table_name" {
  type = string
}

variable "hash_key" {
  type = string
}

variable "hash_key_type" {
  type    = string
  default = "S"
}

variable "range_key" {
  type    = string
  default = null
}

variable "range_key_type" {
  type    = string
  default = "S"
}

variable "tags" {
  type    = map(string)
  default = {}
}
