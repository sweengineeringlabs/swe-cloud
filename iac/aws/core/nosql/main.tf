# AWS NoSQL (DynamoDB) Core Module

resource "aws_dynamodb_table" "this" {
  name         = var.table_name
  billing_mode = var.billing_mode

  read_capacity  = var.billing_mode == "PROVISIONED" ? var.read_capacity : null
  write_capacity = var.billing_mode == "PROVISIONED" ? var.write_capacity : null

  hash_key  = var.hash_key
  range_key = var.range_key

  attribute {
    name = var.hash_key
    type = var.hash_key_type
  }

  dynamic "attribute" {
    for_each = var.range_key != null ? [1] : []
    content {
      name = var.range_key
      type = var.range_key_type
    }
  }

  dynamic "attribute" {
    for_each = var.attributes
    content {
      name = attribute.value.name
      type = attribute.value.type
    }
  }

  tags = var.tags
}

output "table_id" {
  value = aws_dynamodb_table.this.id
}

output "table_arn" {
  value = aws_dynamodb_table.this.arn
}
