# Compute Facade
# Facade Layer - Public interface for compute resources

terraform {
  required_version = ">= 1.0"
}

# ============================================================================
# IMPORT COMMON LAYER
# ============================================================================

locals {
  # Import size mappings
  compute_instance_types = {
    aws = {
      small  = "t3.micro"
      medium = "t3.medium"
      large  = "m5.large"
      xlarge = "m5.xlarge"
    }
    azure = {
      small  = "Standard_B1s"
      medium = "Standard_B2s"
      large  = "Standard_DS2_v2"
      xlarge = "Standard_DS3_v2"
    }
    gcp = {
      small  = "e2-micro"
      medium = "e2-medium"
      large  = "n2-standard-2"
      xlarge = "n2-standard-4"
    }
    oracle = {
      small  = "VM.Standard.E4.Flex"
      medium = "VM.Standard.E4.Flex"
      large  = "VM.Standard3.Flex"
      xlarge = "VM.Standard3.Flex"
    }
    zero = {
      small  = "zero.micro"
      medium = "zero.medium"
      large  = "zero.large"
      xlarge = "zero.xlarge"
    }
  }

  # Build common tags
  common_tags = merge(
    var.tags,
    {
      ManagedBy    = "Terraform"
      Environment  = var.environment
      Provider     = var.provider_name
      Project      = var.project_name
      Architecture = "SEA"
    }
  )
}

# ============================================================================
# PROVIDER-SPECIFIC MODULE ROUTING
# ============================================================================

# Route to AWS compute module
module "aws_compute" {
  count  = var.provider_name == "aws" ? 1 : 0
  source = "../../aws/core/compute"
  
  ami           = lookup(var.provider_config, "ami", "ami-0c55b159cbfafe1f0")
  instance_type = local.compute_instance_types[var.provider_name][var.instance_size]
  ssh_key_name  = var.ssh_public_key != null ? "compute-key" : null
  tags          = local.common_tags
}

# Route to Azure compute module  
module "azure_compute" {
  count  = var.provider_name == "azure" ? 1 : 0
  source = "../../azure/core/compute"
  
  vm_name             = var.instance_name
  vm_size             = local.compute_instance_types[var.provider_name][var.instance_size]
  resource_group_name = "${var.project_name}-${var.environment}-rg"
  location            = "East US"
  admin_username      = "cloudkit"
  ssh_public_key      = var.ssh_public_key != null ? var.ssh_public_key : "ssh-rsa AAAAB3NzaC1yc2EA..." # Default dummy key
  subnet_id           = "/subscriptions/sub/resourceGroups/rg/providers/Microsoft.Network/virtualNetworks/vn/subnets/sn" # Placeholder
  create_public_ip    = true
  tags                = local.common_tags
}

# Route to GCP compute module
module "gcp_compute" {
  count  = var.provider_name == "gcp" ? 1 : 0
  source = "../../gcp/core/compute"
  
  instance_name  = var.instance_name
  machine_type   = local.compute_instance_types[var.provider_name][var.instance_size]
  zone           = "us-east1-b"
  boot_disk_image = "debian-cloud/debian-11"
  network        = "default"
  subnetwork     = "default"
  create_external_ip = true
  labels         = local.common_tags
}

# Route to Zero compute module
module "zero_compute" {
  count  = var.provider_name == "zero" ? 1 : 0
  source = "../../zero/core/compute"
  
  instance_name = var.instance_name
  instance_type = local.compute_instance_types[var.provider_name][var.instance_size]
  ami           = "zero-ami-latest" # Mocked in Zero
  tags          = local.common_tags
}

# Aggregated outputs (select based on provider)
locals {
  instance_id = (
    var.provider_name == "aws" ? (length(module.aws_compute) > 0 ? module.aws_compute[0].instance_id : null) :
    var.provider_name == "azure" ? (length(module.azure_compute) > 0 ? module.azure_compute[0].vm_id : null) :
    var.provider_name == "gcp" ? (length(module.gcp_compute) > 0 ? module.gcp_compute[0].instance_id : null) :
    var.provider_name == "zero" ? (length(module.zero_compute) > 0 ? module.zero_compute[0].instance_id : null) :
    null
  )
  
  public_ip = (
    var.provider_name == "aws" ? (length(module.aws_compute) > 0 ? module.aws_compute[0].public_ip : null) :
    var.provider_name == "azure" ? (length(module.azure_compute) > 0 ? module.azure_compute[0].public_ip : null) :
    var.provider_name == "gcp" ? (length(module.gcp_compute) > 0 ? module.gcp_compute[0].public_ip : null) :
    var.provider_name == "zero" ? (length(module.zero_compute) > 0 ? module.zero_compute[0].public_ip : null) :
    null
  )
  
  private_ip = (
    var.provider_name == "aws" ? (length(module.aws_compute) > 0 ? module.aws_compute[0].private_ip : null) :
    var.provider_name == "azure" ? (length(module.azure_compute) > 0 ? module.azure_compute[0].private_ip : null) :
    var.provider_name == "gcp" ? (length(module.gcp_compute) > 0 ? module.gcp_compute[0].private_ip : null) :
    var.provider_name == "zero" ? (length(module.zero_compute) > 0 ? module.zero_compute[0].private_ip : null) :
    null
  )
}

# ============================================================================
# OUTPUTS (User-facing, simplified)
# ============================================================================

output "instance" {
  description = "Complete instance details"
  value = {
    # Identification
    id   = local.instance_id
    name = var.instance_name
    
    # Specifications
    type     = local.compute_instance_types[var.provider_name][var.instance_size]
    size     = var.instance_size
    provider = var.provider_name
    
    # Network
    public_ip  = local.public_ip
    private_ip = local.private_ip
    
    # Metadata
    tags = local.common_tags
  }
  sensitive = true
}

# Convenience outputs
output "instance_id" {
  description = "Instance ID for reference in other resources"
  value       = local.instance_id
}

output "public_ip" {
  description = "Public IP address (null if public access disabled)"
  value       = local.public_ip
}

output "private_ip" {
  description = "Private IP address"
  value       = local.private_ip
}

output "ssh_connection" {
  description = "SSH connection command"
  value       = local.public_ip != null ? "ssh user@${local.public_ip}" : null
  sensitive   = true
}

# ============================================================================
# USAGE EXAMPLE (in comments for reference)
# ============================================================================

/*
Example usage:

module "web_server" {
  source = "./facade/compute"
  
  # Required
  provider      = "aws"
  instance_name = "web-server-01"
  instance_size = "medium"
  
  # Project info
  project_name = "my-project"
  environment  = "prod"
  
  # Optional
  ssh_public_key      = file("~/.ssh/id_rsa.pub")
  allow_public_access = true
  
  # Provider-specific
  provider_config = {
    ami = "ami-0c55b159cbfafe1f0"
  }
}

# Access outputs
output "server_ip" {
  value = module.web_server.public_ip
}
*/
