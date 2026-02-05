terraform {
  required_providers {
    google = {
      source = "hashicorp/google"
    }
  }
}

variable "function_name" { type = string }
variable "handler" { type = string }
variable "runtime" { type = string }
variable "filename" { type = string }
variable "environment_variables" { type = map(string) }
variable "tags" { type = map(string) }

resource "google_storage_bucket" "bucket" {
  name     = "${var.function_name}-src"
  location = "US"
}

resource "google_storage_bucket_object" "archive" {
  name   = "source.zip"
  bucket = google_storage_bucket.bucket.name
  source = var.filename
}

resource "google_cloudfunctions_function" "function" {
  name        = var.function_name
  description = "Managed by Terraform SEA"
  runtime     = var.runtime == "python3.11" ? "python311" : "nodejs18"

  available_memory_mb   = 128
  source_archive_bucket = google_storage_bucket.bucket.name
  source_archive_object = google_storage_bucket_object.archive.name
  trigger_http          = true
  entry_point           = replace(var.handler, ".handler", "")
  
  environment_variables = var.environment_variables
  labels                = var.tags
}
