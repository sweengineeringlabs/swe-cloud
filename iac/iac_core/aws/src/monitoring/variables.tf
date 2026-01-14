variable "create_alarm" { description = "Create metric alarm"; type = bool; default = false }
variable "alarm_name" { description = "Alarm name"; type = string; default = null }
variable "comparison_operator" { description = "Comparison operator"; type = string; default = "GreaterThanThreshold" }
variable "evaluation_periods" { description = "Evaluation periods"; type = number; default = 1 }
variable "metric_name" { description = "Metric name"; type = string; default = null }
variable "namespace" { description = "Namespace"; type = string; default = null }
variable "period" { description = "Period in seconds"; type = number; default = 300 }
variable "statistic" { description = "Statistic (SampleCount, Average, Sum, Minimum, Maximum)"; type = string; default = "Average" }
variable "threshold" { description = "Threshold"; type = number; default = 0 }
variable "alarm_description" { description = "Description"; type = string; default = null }
variable "alarm_actions" { description = "List of actions ARN"; type = list(string); default = [] }
variable "ok_actions" { description = "List of OK actions ARN"; type = list(string); default = [] }
variable "dimensions" { description = "Dimensions map"; type = map(string); default = {} }

variable "create_log_group" { description = "Create log group"; type = bool; default = false }
variable "log_group_name" { description = "Log group name"; type = string; default = null }
variable "retention_in_days" { description = "Log retention days"; type = number; default = 14 }
variable "kms_key_id" { description = "KMS Key ID"; type = string; default = null }

variable "create_dashboard" { description = "Create dashboard"; type = bool; default = false }
variable "dashboard_name" { description = "Dashboard name"; type = string; default = null }
variable "dashboard_body" { description = "Dashboard JSON body"; type = string; default = null }

variable "tags" { description = "Tags"; type = map(string); default = {} }
