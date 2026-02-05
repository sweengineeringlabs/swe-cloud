# Networking API Contract - Output Values
# Provider-agnostic network resource outputs

output "network_id" {
  description = "Unique identifier of the virtual network"
  value       = var.network_id
}

output "network_arn" {
  description = "Provider resource identifier (ARN/Resource ID)"
  value       = var.network_arn
}

output "network_name" {
  description = "Name of the virtual network"
  value       = var.network_name
}

output "network_cidr" {
  description = "CIDR block of the network"
  value       = var.network_cidr
}

output "internet_gateway_id" {
  description = "Internet gateway ID (if created)"
  value       = var.internet_gateway_id
}

output "nat_gateway_ids" {
  description = "List of NAT gateway IDs (if created)"
  value       = var.nat_gateway_ids
}

output "public_subnet_ids" {
  description = "List of public subnet IDs"
  value       = var.public_subnet_ids
}

output "private_subnet_ids" {
  description = "List of private subnet IDs"
  value       = var.private_subnet_ids
}

output "public_route_table_ids" {
  description = "List of public route table IDs"
  value       = var.public_route_table_ids
}

output "private_route_table_ids" {
  description = "List of private route table IDs"
  value       = var.private_route_table_ids
}

output "default_security_group_id" {
  description = "Default security group ID"
  value       = var.default_security_group_id
}

output "availability_zones" {
  description = "Availability zones where subnets are deployed"
  value       = var.availability_zones_used
}

output "subnet_details" {
  description = "Detailed information about all subnets"
  value = {
    public  = var.public_subnet_details
    private = var.private_subnet_details
  }
}

output "network_configuration" {
  description = "Network configuration summary"
  value = {
    dns_enabled            = var.dns_enabled
    internet_gateway       = var.has_internet_gateway
    nat_gateway_count      = var.nat_gateway_count_actual
    flow_logs_enabled      = var.flow_logs_enabled
    total_subnets          = var.total_subnets
    public_subnet_count    = var.public_subnet_count
    private_subnet_count   = var.private_subnet_count
  }
}

# Variable definitions for outputs (these would be populated by the core layer)
variable "network_id" { type = string }
variable "network_arn" { type = string }
variable "network_name" { type = string }
variable "network_cidr" { type = string }
variable "internet_gateway_id" { type = string }
variable "nat_gateway_ids" { type = list(string) }
variable "public_subnet_ids" { type = list(string) }
variable "private_subnet_ids" { type = list(string) }
variable "public_route_table_ids" { type = list(string) }
variable "private_route_table_ids" { type = list(string) }
variable "default_security_group_id" { type = string }
variable "availability_zones_used" { type = list(string) }
variable "public_subnet_details" { type = list(map(string)) }
variable "private_subnet_details" { type = list(map(string)) }
variable "dns_enabled" { type = bool }
variable "has_internet_gateway" { type = bool }
variable "nat_gateway_count_actual" { type = number }
variable "flow_logs_enabled" { type = bool }
variable "total_subnets" { type = number }
variable "public_subnet_count" { type = number }
variable "private_subnet_count" { type = number }
