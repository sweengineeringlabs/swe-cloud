# AWS Networking (VPC)
# Mirrors CloudKit's cloudkit_core/aws/src/vpc.rs pattern

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

# VPC
resource "aws_vpc" "this" {
  cidr_block           = var.vpc_cidr
  enable_dns_hostnames = var.enable_dns_hostnames
  enable_dns_support   = var.enable_dns_support
  
  tags = merge(
    var.tags,
    {
      Name = var.vpc_name
    }
  )
}

# Internet Gateway
resource "aws_internet_gateway" "this" {
  count = var.create_internet_gateway ? 1 : 0
  
  vpc_id = aws_vpc.this.id
  
  tags = merge(
    var.tags,
    {
      Name = "${var.vpc_name}-igw"
    }
  )
}

# Public Subnets
resource "aws_subnet" "public" {
  count = length(var.public_subnet_cidrs)
  
  vpc_id                  = aws_vpc.this.id
  cidr_block              = var.public_subnet_cidrs[count.index]
  availability_zone       = var.availability_zones[count.index]
  map_public_ip_on_launch = true
  
  tags = merge(
    var.tags,
    {
      Name = "${var.vpc_name}-public-${count.index + 1}"
      Type = "Public"
    }
  )
}

# Private Subnets
resource "aws_subnet" "private" {
  count = length(var.private_subnet_cidrs)
  
  vpc_id            = aws_vpc.this.id
  cidr_block        = var.private_subnet_cidrs[count.index]
  availability_zone = var.availability_zones[count.index]
  
  tags = merge(
    var.tags,
    {
      Name = "${var.vpc_name}-private-${count.index + 1}"
      Type = "Private"
    }
  )
}

# Public Route Table
resource "aws_route_table" "public" {
  count = length(var.public_subnet_cidrs) > 0 ? 1 : 0
  
  vpc_id = aws_vpc.this.id
  
  tags = merge(
    var.tags,
    {
      Name = "${var.vpc_name}-public-rt"
    }
  )
}

# Public Route to Internet
resource "aws_route" "public_internet" {
  count = var.create_internet_gateway && length(var.public_subnet_cidrs) > 0 ? 1 : 0
  
  route_table_id         = aws_route_table.public[0].id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.this[0].id
}

# Public Route Table Associations
resource "aws_route_table_association" "public" {
  count = length(var.public_subnet_cidrs)
  
  subnet_id      = aws_subnet.public[count.index].id
  route_table_id = aws_route_table.public[0].id
}

# Private Route Tables (one per AZ for flexibility)
resource "aws_route_table" "private" {
  count = length(var.private_subnet_cidrs)
  
  vpc_id = aws_vpc.this.id
  
  tags = merge(
    var.tags,
    {
      Name = "${var.vpc_name}-private-rt-${count.index + 1}"
    }
  )
}

# Private Route Table Associations
resource "aws_route_table_association" "private" {
  count = length(var.private_subnet_cidrs)
  
  subnet_id      = aws_subnet.private[count.index].id
  route_table_id = aws_route_table.private[count.index].id
}

# Default Security Group
resource "aws_security_group" "default" {
  count = var.create_default_security_group ? 1 : 0
  
  name_prefix = "${var.vpc_name}-default-"
  description = "Default security group for ${var.vpc_name}"
  vpc_id      = aws_vpc.this.id
  
  # Allow all outbound
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
  
  # Allow inbound from VPC
  ingress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = [var.vpc_cidr]
  }
  
  tags = merge(
    var.tags,
    {
      Name = "${var.vpc_name}-default-sg"
    }
  )
}

# Outputs
output "vpc_id" {
  description = "VPC ID"
  value       = aws_vpc.this.id
}

output "vpc_arn" {
  description = "VPC ARN"
  value       = aws_vpc.this.arn
}

output "vpc_cidr" {
  description = "VPC CIDR block"
  value       = aws_vpc.this.cidr_block
}

output "internet_gateway_id" {
  description = "Internet Gateway ID"
  value       = length(aws_internet_gateway.this) > 0 ? aws_internet_gateway.this[0].id : null
}

output "public_subnet_ids" {
  description = "Public subnet IDs"
  value       = aws_subnet.public[*].id
}

output "private_subnet_ids" {
  description = "Private subnet IDs"
  value       = aws_subnet.private[*].id
}

output "public_route_table_id" {
  description = "Public route table ID"
  value       = length(aws_route_table.public) > 0 ? aws_route_table.public[0].id : null
}

output "private_route_table_ids" {
  description = "Private route table IDs"
  value       = aws_route_table.private[*].id
}

output "default_security_group_id" {
  description = "Default security group ID"
  value       = length(aws_security_group.default) > 0 ? aws_security_group.default[0].id : null
}
