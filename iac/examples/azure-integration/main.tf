# Azure Integration Testing Example
terraform {
  required_version = ">= 1.5.0"
  
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

provider "azurerm" {
  features {}
  skip_provider_registration = true
  storage_use_azuread        = false
  
  # CloudEmu Azure endpoint
  metadata_host = "http://localhost:10000"
}

# 1. Storage Resource (Blob)
module "storage" {
  source = "../../facade/storage"
  
  provider_name = "azure"
  bucket_name   = var.bucket_name
  project_name  = "azure-test"
  environment   = var.environment
  
  # CloudEmu-specific
  versioning_enabled = true
}

# 2. NoSQL Resource (Cosmos DB)
module "nosql" {
  source = "../../facade/nosql"
  
  provider_name = "azure"
  table_name    = var.table_name
  hash_key      = "id"
  project_name  = "azure-test"
  environment   = var.environment
}

# 3. Networking Resource (VNet)
module "networking" {
  source = "../../facade/networking"
  
  provider_name = "azure"
  network_name  = "azure-vnet"
  
  metrics = {
    cidr            = "10.0.0.0/16"
    azs             = ["1", "2"]
    public_subnets  = ["10.0.1.0/24", "10.0.2.0/24"]
    private_subnets = ["10.0.3.0/24", "10.0.4.0/24"]
  }
  
  project_name  = "azure-test"
  environment   = var.environment
}

# 4. Identity Resource (Managed Identity)
module "iam" {
  source = "../../facade/iam"
  
  provider_name = "azure"
  identity_type = "user"
  identity_name = "azure-test-identity"
  principals    = []
  
  project_name  = "azure-test"
  environment   = var.environment
}

# 5. Compute Resource (Function App)
module "lambda" {
  source = "../../facade/lambda"
  
  provider_name = "azure"
  function_name = "azure-test-func"
  handler       = "index.handler"
  runtime       = "node"
  
  # Basic inline code not supported easily in Azure provider via Terraform directly
  # filename is required, but we can mock it or assume file existence for integration test
  # This part relies on core module implementation details
  filename      = "${path.module}/files/test_function.zip" 
  
  project_name  = "azure-test"
  environment   = var.environment
}

# 6. Messaging Resource (Service Bus Queue)
module "queue" {
  source = "../../facade/messaging"
  
  provider_name = "azure"
  type          = "queue"
  name          = "azure-test-queue"
  
  project_name  = "azure-test"
  environment   = var.environment
}

# Variables
variable "bucket_name" {
  type    = string
  default = "test-azure-container"
}

variable "table_name" {
  type    = string
  default = "test-azure-cosmos"
}

variable "environment" {
  type    = string
  default = "test"
}

# Outputs
output "bucket_name" {
  value = module.storage.bucket.name
}

output "bucket_url" {
  value = module.storage.bucket_url
}

output "table_name" {
  value = var.table_name
}

output "vnet_id" {
  value = module.networking.network_id
}

output "identity_id" {
  value = module.iam.principal_id
}

output "function_name" {
  value = module.lambda.function_name
}

output "queue_url" {
  value = module.queue.resource_url
}
