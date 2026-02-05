variable "account_name" {
  description = "CosmosDB account name"
  type        = string
}

variable "resource_group_name" {
  description = "Resource group name"
  type        = string
}

variable "location" {
  description = "Azure region"
  type        = string
}

variable "database_name" {
  description = "CosmosDB database name"
  type        = string
  default     = "cloudkit-db"
}

variable "container_name" {
  description = "CosmosDB container name"
  type        = string
}

variable "partition_key_path" {
  description = "Partition key path"
  type        = string
  default     = "/id"
}

variable "throughput" {
  description = "Throughput (RU/s)"
  type        = number
  default     = 400
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
