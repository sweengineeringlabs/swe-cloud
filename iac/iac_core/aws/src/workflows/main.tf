# AWS Step Functions Core Module

resource "aws_sfn_state_machine" "this" {
  name     = var.name
  role_arn = var.role_arn
  type     = var.type

  definition = var.definition

  tags = var.tags
}

output "workflow_id" {
  value = aws_sfn_state_machine.this.id
}

output "workflow_arn" {
  value = aws_sfn_state_machine.this.arn
}
