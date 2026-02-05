# GCP Firestore Core Module

resource "google_firestore_database" "this" {
  project     = var.project_id
  name        = var.database_id
  location_id = var.location_id
  type        = var.type

  # Note: Deleting a firestore database is a destructive operation.
  # delete_protection_state = "DELETE_PROTECTION_DISABLED"
}

output "database_id" {
  value = google_firestore_database.this.name
}

output "location" {
  value = google_firestore_database.this.location_id
}
