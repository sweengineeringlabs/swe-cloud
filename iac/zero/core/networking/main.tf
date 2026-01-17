terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

variable "vpc_name" { type = string }
variable "vpc_cidr" { type = string }
variable "availability_zones" { type = list(string) }
variable "public_subnet_cidrs" { type = list(string) }
variable "private_subnet_cidrs" { type = list(string) }
variable "create_internet_gateway" { type = bool }
variable "create_default_security_group" { type = bool }
variable "tags" { type = map(string) }

# Reuse AWS Provider for ZeroNet (redirected via SPI)
resource "aws_vpc" "this" {
  cidr_block = var.vpc_cidr
  
  tags = merge(var.tags, {
    Name = var.vpc_name
  })
}

resource "aws_subnet" "public" {
  count             = length(var.public_subnet_cidrs)
  vpc_id            = aws_vpc.this.id
  cidr_block        = var.public_subnet_cidrs[count.index]
  
  # For Zero, AZs are mocked but we pass them for parity
  availability_zone = element(var.availability_zones, count.index)
  
  tags = merge(var.tags, {
    Name = "${var.vpc_name}-public-${count.index}"
  })
}

resource "aws_subnet" "private" {
  count             = length(var.private_subnet_cidrs)
  vpc_id            = aws_vpc.this.id
  cidr_block        = var.private_subnet_cidrs[count.index]
  availability_zone = element(var.availability_zones, count.index)
  
  tags = merge(var.tags, {
    Name = "${var.vpc_name}-private-${count.index}"
  })
}
