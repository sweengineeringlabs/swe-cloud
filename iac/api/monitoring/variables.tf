variable "alarm_name" {
  description = "Name of the alarm"
  type        = string
}

variable "metric_name" {
  description = "Name of the metric to monitor"
  type        = string
}

variable "threshold" {
  description = "Threshold for the alarm"
  type        = number
}

variable "comparison_operator" {
  description = "Comparison operator for the alarm"
  type        = string
  default     = "GreaterThanThreshold"
}

variable "evaluation_periods" {
  description = "The number of periods over which data is compared to the specified threshold"
  type        = number
  default     = 1
}

variable "period" {
  description = "The period in seconds over which the specified statistic is applied"
  type        = number
  default     = 300
}

variable "tags" {
  description = "Resource tags"
  type        = map(string)
  default     = {}
}
