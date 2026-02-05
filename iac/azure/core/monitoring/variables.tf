variable "resource_group_name" { description = "Resource group"; type = string }
variable "location" { description = "Location"; type = string; default = null } # Required for workspace

variable "create_action_group" { type = bool; default = false }
variable "action_group_name" { type = string; default = null }
variable "short_name" { type = string; default = null }
variable "email_receivers" { type = list(object({ name = string, email = string })); default = [] }

variable "create_alert" { type = bool; default = false }
variable "alert_name" { type = string; default = null }
variable "scopes" { type = list(string); default = [] }
variable "description" { type = string; default = null }
variable "metric_namespace" { type = string; default = null }
variable "metric_name" { type = string; default = null }
variable "aggregation" { type = string; default = "Average" }
variable "operator" { type = string; default = "GreaterThan" }
variable "threshold" { type = number; default = 0 }
variable "action_group_id" { type = string; default = null }

variable "create_workspace" { type = bool; default = false }
variable "workspace_name" { type = string; default = null }
variable "sku" { type = string; default = "PerGB2018" }
variable "retention_in_days" { type = number; default = 30 }

variable "tags" { type = map(string); default = {} }
