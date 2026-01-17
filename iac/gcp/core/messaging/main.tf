terraform {
  required_providers {
    google = {
      source = "hashicorp/google"
    }
  }
}

variable "queue_name" { type = string }
variable "topic_name" { type = string }
variable "create_queue" { type = bool }
variable "create_topic" { type = bool }
variable "tags" { type = map(string) }

# Pub/Sub Topic
resource "google_pubsub_topic" "this" {
  count = var.create_topic ? 1 : 0
  name  = var.topic_name
  labels = var.tags
}

# Pub/Sub Subscription (acts as a queue)
resource "google_pubsub_subscription" "this" {
  count = var.create_queue ? 1 : 0
  name  = var.queue_name
  topic = var.topic_name # In GCP, queues (subscriptions) need a topic. Simplified for parity.
  labels = var.tags
}
