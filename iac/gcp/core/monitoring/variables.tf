variable "create_alert_policy" { type = bool; default = false }
variable "display_name" { type = string; default = null }
variable "combiner" { type = string; default = "OR" }

variable "condition_display_name" { type = string; default = "Condition" }
variable "filter" { type = string; default = null } # Required: e.g. "metric.type=\"compute.googleapis.com/instance/cpu/utilization\" AND resource.type=\"gce_instance\""
variable "duration" { type = string; default = "60s" }
variable "comparison" { type = string; default = "COMPARISON_GT" }
variable "alignment_period" { type = string; default = "60s" }
variable "per_series_aligner" { type = string; default = "ALIGN_MEAN" }
variable "threshold_value" { type = number; default = 0.8 }

variable "notification_channels" { type = list(string); default = [] }

variable "create_email_channel" { type = bool; default = false }
variable "email_address" { type = string; default = null }
