# Compute Facade
# Facade Layer - Public interface for compute resources

terraform {
  required_version = ">= 1.0"
}

# ============================================================================
# IMPORT COMMON LAYER
# ============================================================================

# Import common definitions
data "terraform_remote_state" "common" {
  backend = "local"
  
  config = {
    path = "../../common/terraform.tfstate"
  }
}

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
  }

  # Build common tags
  common_tags = merge(
    var.tags,
    {
      ManagedBy    = "Terraform"
      Environment  = var.environment
      Provider     = var.provider
      Project      = var.project_name
      Architecture = "SEA"
    }
  )
}

# ============================================================================
# CORE ORCHESTRATION
# ============================================================================

module "compute_core" {
  source = "../../core/compute"

  # API contract inputs
  instance_name        = var.instance_name
  instance_size        = var.instance_size
  provider             = var.provider
  ssh_public_key       = var.ssh_public_key
  admin_username       = var.admin_username
  allow_public_access  = var.allow_public_access
  enable_monitoring    = var.enable_monitoring
  enable_backup        = var.enable_backup
  
  # Network configuration
  network_id           = var.network_id
  subnet_id            = var.subnet_id
  security_group_ids   = var.security_group_ids
  
  # Advanced options
  user_data            = var.user_data
  instance_tags        = var.instance_tags
  provider_config      = var.provider_config
  
  # From common layer
  compute_instance_types = local.compute_instance_types
  common_tags            = local.common_tags
}

# ============================================================================
# OUTPUTS (User-facing, simplified)
# ============================================================================

output "instance" {
  description = "Complete instance details"
  value = {
    # Identification
    id   = module.compute_core.instance_id
    arn  = module.compute_core.instance_arn
    name = var.instance_name
    
    # Specifications
    type            = module.compute_core.instance_type
    size            = module.compute_core.instance_size
    provider        = var.provider
    
    # Network
    public_ip       = module.compute_core.public_ip
    private_ip      = module.compute_core.private_ip
    
    # Connection
    ssh_connection  = module.compute_core.ssh_connection
    
    # State
    state           = module.compute_core.state
    zone            = module.compute_core.availability_zone
    
    # Metadata
    tags            = module.compute_core.tags
  }
  sensitive = true
}

# Convenience outputs
output "instance_id" {
  description = "Instance ID for reference in other resources"
  value       = module.compute_core.instance_id
}

output "public_ip" {
  description = "Public IP address (null if public access disabled)"
  value       = module.compute_core.public_ip
}

output "private_ip" {
  description = "Private IP address"
  value       = module.compute_core.private_ip
}

output "ssh_connection" {
  description = "SSH connection command"
  value       = module.compute_core.ssh_connection
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
