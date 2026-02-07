# ZeroCloud NoSQL (ZeroDB)
# Mirrors CloudKit's cloudkit_core/zero/dynamodb.rs

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

resource "aws_dynamodb_table" "this" {
  name           = var.table_name
  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = var.hash_key
  range_key      = var.range_key

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

  tags = var.tags
}

output "table_id" {
  value = aws_dynamodb_table.this.id
}

output "table_arn" {
  value = aws_dynamodb_table.this.arn
}
