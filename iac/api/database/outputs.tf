# Database API Contract - Output Values
# Provider-agnostic database resource outputs

output "database_id" {
  description = "Unique identifier of the database instance"
  value       = var.database_id
}

output "database_arn" {
  description = "Provider resource identifier (ARN/Resource ID)"
  value       = var.database_arn
}

output "endpoint" {
  description = "Database connection endpoint (hostname:port)"
  value       = var.endpoint
}

output "address" {
  description = "Database hostname"
  value       = var.address
}

output "port" {
  description = "Database port number"
  value       = var.port
}

output "database_name" {
  description = "Name of the database"
  value       = var.database_name
}

output "engine" {
  description = "Database engine type"
  value       = var.engine
}

output "engine_version" {
  description = "Database engine version"
  value       = var.engine_version
}

output "status" {
  description = "Current status of the database instance"
  value       = var.status
}

output "availability_zone" {
  description = "Availability zone where the instance is deployed"
  value       = var.availability_zone
}

output "multi_az" {
  description = "Whether Multi-AZ is enabled"
  value       = var.multi_az
}

output "storage_encrypted" {
  description = "Whether storage encryption is enabled"
  value       = var.storage_encrypted
}

output "allocated_storage_gb" {
  description = "Allocated storage in GB"
  value       = var.allocated_storage_gb
}

output "backup_retention_days" {
  description = "Backup retention period in days"
  value       = var.backup_retention_days
}

output "publicly_accessible" {
  description = "Whether the database is publicly accessible"
  value       = var.publicly_accessible
}

output "connection_string" {
  description = "Database connection string (without password)"
  value       = var.connection_string
  sensitive   = true
}

output "metadata" {
  description = "Additional database metadata"
  value = {
    created_at          = var.created_at
    last_modified       = var.last_modified
    instance_class      = var.instance_class
    deletion_protection = var.deletion_protection
  }
}

# Variable definitions for outputs (these would be populated by the core layer)
variable "database_id" { type = string }
variable "database_arn" { type = string }
variable "endpoint" { type = string }
variable "address" { type = string }
variable "port" { type = number }
variable "database_name" { type = string }
variable "engine" { type = string }
variable "engine_version" { type = string }
variable "status" { type = string }
variable "availability_zone" { type = string }
variable "multi_az" { type = bool }
variable "storage_encrypted" { type = bool }
variable "allocated_storage_gb" { type = number }
variable "backup_retention_days" { type = number }
variable "publicly_accessible" { type = bool }
variable "connection_string" { type = string }
variable "created_at" { type = string }
variable "last_modified" { type = string }
variable "instance_class" { type = string }
variable "deletion_protection" { type = bool }
