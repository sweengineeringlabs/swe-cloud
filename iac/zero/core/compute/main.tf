terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

variable "instance_name" { type = string }
variable "instance_type" { type = string }
variable "ami" { type = string }
variable "tags" { type = map(string) }

# Reuse AWS Provider for ZeroCompute (redirected via SPI)
resource "aws_instance" "this" {
  ami           = var.ami
  instance_type = var.instance_type
  
  tags = merge(var.tags, {
    Name = var.instance_name
  })
}

# Outputs
output "instance_id" {
  value = aws_instance.this.id
}

output "public_ip" {
  value = aws_instance.this.public_ip
}

output "private_ip" {
  value = aws_instance.this.private_ip
}
