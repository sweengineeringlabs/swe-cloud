# Zero Provider Variables

variable "zero_endpoint_store" {
  description = "ZeroStore endpoint"
  type        = string
  default     = "http://localhost:8080/v1/store"
}

variable "zero_endpoint_db" {
  description = "ZeroDB endpoint"
  type        = string
  default     = "http://localhost:8080/v1/db"
}

variable "zero_endpoint_func" {
  description = "ZeroFunc endpoint"
  type        = string
  default     = "http://localhost:8080/v1/func"
}

variable "zero_endpoint_queue" {
  description = "ZeroQueue endpoint"
  type        = string
  default     = "http://localhost:8080/v1/queue"
}

variable "zero_endpoint_iam" {
  description = "ZeroID endpoint"
  type        = string
  default     = "http://localhost:8080/v1/iam"
}

variable "common_tags" {
  description = "Common tags for all resources"
  type        = map(string)
  default     = {}
}
