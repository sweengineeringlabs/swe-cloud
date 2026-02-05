# AWS Database (RDS)
# Mirrors CloudKit's cloudkit_core/aws/src/dynamodb.rs pattern

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

resource "aws_db_instance" "this" {
  # Instance configuration
  identifier     = var.identifier
  engine         = var.engine
  engine_version = var.engine_version
  instance_class = var.instance_class
  
  # Storage
  allocated_storage     = var.allocated_storage
  max_allocated_storage = var.max_allocated_storage
  storage_type          = var.storage_type
  storage_encrypted     = var.storage_encrypted
  kms_key_id            = var.kms_key_id
  
  # Database
  db_name  = var.database_name
  username = var.master_username
  password = var.master_password
  port     = var.port
  
  # Network
  db_subnet_group_name   = var.db_subnet_group_name
  vpc_security_group_ids = var.vpc_security_group_ids
  publicly_accessible    = var.publicly_accessible
  
  # Backup & Maintenance
  backup_retention_period = var.backup_retention_period
  backup_window           = var.backup_window
  maintenance_window      = var.maintenance_window
  
  # High Availability
  multi_az               = var.multi_az
  availability_zone      = var.multi_az ? null : var.availability_zone
  
  # Monitoring
  enabled_cloudwatch_logs_exports = var.enabled_cloudwatch_logs_exports
  monitoring_interval             = var.monitoring_interval
  monitoring_role_arn             = var.monitoring_role_arn
  
  # Performance
  performance_insights_enabled    = var.performance_insights_enabled
  performance_insights_kms_key_id = var.performance_insights_kms_key_id
  
  # Deletion protection
  deletion_protection = var.deletion_protection
  skip_final_snapshot = var.skip_final_snapshot
  final_snapshot_identifier = var.skip_final_snapshot ? null : "${var.identifier}-final-snapshot"
  
  # Tags
  tags = var.tags
}

# Outputs
output "db_instance_id" {
  description = "Database instance ID"
  value       = aws_db_instance.this.id
}

output "db_instance_arn" {
  description = "Database instance ARN"
  value       = aws_db_instance.this.arn
}

output "db_instance_endpoint" {
  description = "Database connection endpoint"
  value       = aws_db_instance.this.endpoint
}

output "db_instance_address" {
  description = "Database hostname"
  value       = aws_db_instance.this.address
}

output "db_instance_port" {
  description = "Database port"
  value       = aws_db_instance.this.port
}

output "db_instance_name" {
  description = "Database name"
  value       = aws_db_instance.this.db_name
}

output "db_instance_status" {
  description = "Database instance status"
  value       = aws_db_instance.this.status
}

output "availability_zone" {
  description = "Availability zone"
  value       = aws_db_instance.this.availability_zone
}

output "resource_id" {
  description = "Resource ID"
  value       = aws_db_instance.this.resource_id
}
