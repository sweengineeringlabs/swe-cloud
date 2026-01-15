variable "project_id" {
  description = "GCP project ID"
  type        = string
}

variable "database_id" {
  description = "Firestore database ID"
  type        = string
  default     = "(default)"
}

variable "location_id" {
  description = "Location ID"
  type        = string
  default     = "us-central"
}

variable "type" {
  description = "Firestore type (FIRESTORE_NATIVE, DATASTORE_MODE)"
  type        = string
  default     = "FIRESTORE_NATIVE"
}
