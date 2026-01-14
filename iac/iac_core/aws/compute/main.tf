# AWS Compute (EC2)
# Mirrors CloudKit's cloudkit_core/aws/ec2.rs

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

resource "aws_instance" "this" {
  ami           = var.ami
  instance_type = var.instance_type
  key_name      = var.ssh_key_name
  subnet_id     = var.subnet_id
  
  vpc_security_group_ids = var.security_group_ids
  iam_instance_profile   = var.instance_profile_name
  
  user_data = var.user_data
  
  monitoring    = var.enable_monitoring
  ebs_optimized = var.ebs_optimized
  
  tags = var.tags
}

output "instance_id" {
  value = aws_instance.this.id
}

output "arn" {
  value = aws_instance.this.arn
}

output "public_ip" {
  value = aws_instance.this.public_ip
}

output "private_ip" {
  value = aws_instance.this.private_ip
}

output "instance_state" {
  value = aws_instance.this.instance_state
}

output "availability_zone" {
  value = aws_instance.this.availability_zone
}
