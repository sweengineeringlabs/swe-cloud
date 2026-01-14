# GCP Cloud Monitoring
# Mirrors CloudKit's cloudkit_core/gcp/src/monitoring.rs pattern

terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.0"
    }
  }
}

# Alert Policy
resource "google_monitoring_alert_policy" "this" {
  count = var.create_alert_policy ? 1 : 0
  
  display_name = var.display_name
  combiner     = var.combiner
  
  conditions {
    display_name = var.condition_display_name
    
    condition_threshold {
      filter     = var.filter
      duration   = var.duration
      comparison = var.comparison
      threshold_value = var.threshold_value
      
      aggregations {
        alignment_period   = var.alignment_period
        per_series_aligner = var.per_series_aligner
      }
    }
  }
  
  notification_channels = var.notification_channels
}

# Notification Channel (Email)
resource "google_monitoring_notification_channel" "email" {
  count = var.create_email_channel ? 1 : 0
  
  display_name = "Email Channel ${var.email_address}"
  type         = "email"
  
  labels = {
    email_address = var.email_address
  }
}

output "alert_policy_id" {
  value = var.create_alert_policy ? google_monitoring_alert_policy.this[0].name : null
}

output "notification_channel_id" {
  value = var.create_email_channel ? google_monitoring_notification_channel.email[0].name : null
}
