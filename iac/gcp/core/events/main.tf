# GCP PubSub Topic

resource "google_pubsub_topic" "this" {
  project = var.project_id
  name    = var.topic_name
  labels  = var.labels
}

output "topic_id" {
  value = google_pubsub_topic.this.id
}

output "topic_name" {
  value = google_pubsub_topic.this.name
}
