# Multi-Cloud Web Application Example
# Demonstrates IAC SEA architecture with real deployments
# Updated: 2026-01-14 - Now uses iac_core/{provider}/src/ structure

terraform {
  required_version = ">= 1.0"
}

# ============================================================================
# CONFIGURATION
# ============================================================================

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "dev"
}

variable "project_name" {
  description = "Project name"
  type        = string
  default     = "web-app-demo"
}

# ============================================================================
# AWS DEPLOYMENT
# ============================================================================

module "aws_web_server" {
  source = "../../facade/compute"

  provider_name = "aws"
  instance_name = "web-aws-${var.environment}"
  instance_size = var.environment == "prod" ? "large" : "medium"
  project_name  = var.project_name
  environment   = var.environment

  # Network
  allow_public_access = true
  
  # Access
  ssh_public_key = file("~/.ssh/id_rsa.pub")
  admin_username = "ubuntu"

  # Features
  enable_monitoring = true
  enable_backup     = var.environment == "prod"

  # Initialization script
  user_data = <<-EOF
    #!/bin/bash
    apt-get update
    apt-get install -y nginx
    echo "<h1>Hello from AWS (${var.environment})</h1>" > /var/www/html/index.html
    systemctl start nginx
  EOF

  # AWS-specific
  provider_config = {
    ami              = "ami-0c55b159cbfafe1f0"  # Ubuntu 22.04 LTS
    ebs_optimized    = var.environment == "prod"
  }

  # Tags
  tags = {
    Application = "WebApp"
    Tier        = "Frontend"
    Cloud       = "AWS"
  }
}

module "aws_storage" {
  source = "../../facade/storage"

  provider_name = "aws"
  bucket_name  = "webapp-storage-aws-${var.environment}"
  project_name = var.project_name
  environment  = var.environment

  # Configuration
  storage_class       = "standard"
  versioning_enabled  = var.environment == "prod"
  encryption_enabled  = true
  public_access_block = true

  # Lifecycle for cost optimization
  lifecycle_rules = [{
    id      = "move-old-data"
    enabled = true
    
    transition = [{
      days          = 30
      storage_class = "infrequent"
    }, {
      days          = 90
      storage_class = "archive"
    }]
  }]

  # Tags
  tags = {
    Application = "WebApp"
    DataType    = "UserContent"
    Cloud       = "AWS"
  }
}

# ============================================================================
# AZURE DEPLOYMENT (Optional - demonstrates multi-cloud)
# ============================================================================

# Uncomment to deploy on Azure
/*
module "azure_web_server" {
  source = "../../facade/compute"

  provider_name = "azure"
  instance_name = "web-azure-${var.environment}"
  instance_size = var.environment == "prod" ? "large" : "medium"
  project_name  = var.project_name
  environment   = var.environment

  allow_public_access = true
  ssh_public_key      = file("~/.ssh/id_rsa.pub")
  admin_username      = "azureuser"

  provider_config = {
    resource_group_name = "rg-${var.project_name}-${var.environment}"
    location            = "eastus"
  }

  tags = {
    Application = "WebApp"
    Cloud       = "Azure"
  }
}
*/

# ============================================================================
# GCP DEPLOYMENT (Optional - demonstrates multi-cloud)
# ============================================================================

# Uncomment to deploy on GCP
/*
module "gcp_web_server" {
  source = "../../facade/compute"

  provider_name = "gcp"
  instance_name = "web-gcp-${var.environment}"
  instance_size = var.environment == "prod" ? "large" : "medium"
  project_name  = var.project_name
  environment   = var.environment

  allow_public_access = true
  ssh_public_key      = file("~/.ssh/id_rsa.pub")
  admin_username      = "gcpuser"

  provider_config = {
    project_id = "my-gcp-project"
    zone       = "us-central1-a"
  }

  tags = {
    Application = "WebApp"
    Cloud       = "GCP"
  }
}
*/

# ============================================================================
# OUTPUTS
# ============================================================================

output "aws_server" {
  description = "AWS server details"
  value = {
    instance_id = module.aws_web_server.instance_id
    public_ip   = module.aws_web_server.public_ip
    ssh_command = module.aws_web_server.ssh_connection
    web_url     = "http://${module.aws_web_server.public_ip}"
  }
  sensitive = true
}

output "aws_storage" {
  description = "AWS storage details"
  value = {
    bucket_id  = module.aws_storage.bucket_id
    bucket_url = module.aws_storage.bucket_url
    bucket_arn = module.aws_storage.bucket_arn
  }
}

output "deployment_info" {
  description = "Deployment information"
  value = {
    environment = var.environment
    project     = var.project_name
    
    aws = {
      instance_size = var.environment == "prod" ? "large" : "medium"
      monitoring    = true
      backup        = var.environment == "prod"
      
      # Actual type used
      instance_type = var.environment == "prod" ? "m5.large" : "t3.medium"
    }
  }
}

# ============================================================================
# USAGE INSTRUCTIONS
# ============================================================================

/*
To use this example:

1. Configure AWS credentials:
   export AWS_ACCESS_KEY_ID="your-key"
   export AWS_SECRET_ACCESS_KEY="your-secret"

2. Initialize Terraform:
   cd iac/examples/web-app
   terraform init

3. Deploy to dev environment:
   terraform plan -var="environment=dev"
   terraform apply -var="environment=dev"

4. Deploy to prod environment:
   terraform plan -var="environment=prod"
   terraform apply -var="environment=prod"

5. Access the web server:
   terraform output aws_server
   # Visit http://<public_ip> in browser

6. Enable multi-cloud:
   - Uncomment Azure or GCP modules above
   - Configure provider credentials
   - Run terraform apply

7. Cleanup:
   terraform destroy -var="environment=dev"

Key Features Demonstrated:
- Size normalization (medium → t3.medium on AWS)
- Environment-based configuration (prod gets large instances)
- Automatic tagging (ManagedBy, Environment, etc.)
- Lifecycle management (storage class transitions)
- Security defaults (encryption, private access)
- User data initialization
- Multi-cloud capability (uncomment other providers)
*/
