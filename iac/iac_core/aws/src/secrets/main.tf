# AWS Secrets Manager Core Module

resource "aws_secretsmanager_secret" "this" {
  name                    = var.name
  description             = var.description
  kms_key_id              = var.kms_key_id
  recovery_window_in_days = var.recovery_window_in_days
  tags                    = var.tags
}

resource "aws_secretsmanager_secret_version" "this" {
  count         = var.secret_string != null ? 1 : 0
  secret_id     = aws_secretsmanager_secret.this.id
  secret_string = var.secret_string
}

output "secret_id" {
  value = aws_secretsmanager_secret.this.id
}

output "secret_arn" {
  value = aws_secretsmanager_secret.this.arn
}
