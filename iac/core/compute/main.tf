# Compute Core Orchestration
# Core Layer - Resource composition and dependency management

terraform {
  required_version = ">= 1.0"
}

# ============================================================================
# IMPORT COMMON DEFINITIONS
# ============================================================================

# Size mappings from common layer
locals {
  # Get provider-specific instance type from common layer mappings
  instance_type = var.compute_instance_types[var.provider][var.instance_size]
  
  # Compute resource tags (from common layer + instance-specific)
  resource_tags = merge(
    var.common_tags,
    {
      ResourceType = "Compute"
      Service      = "VirtualMachine"
      InstanceName = var.instance_name
      Size         = var.instance_size
    },
    var.instance_tags
  )

  # SSH connection string
  ssh_user = var.admin_username
  ssh_host = var.allow_public_access && var.public_ip != null ? var.public_ip : var.private_ip
  ssh_connection = var.ssh_public_key != null ? "ssh ${local.ssh_user}@${local.ssh_host}" : null
}

# ============================================================================
# PROVIDER ROUTING
# ============================================================================

# Route to AWS provider
module "aws_instance" {
  count  = var.provider == "aws" ? 1 : 0
  source = "../../providers/aws/compute"

  instance_name        = var.instance_name
  instance_type        = local.instance_type
  ami                  = var.provider_config.ami
  key_name             = var.ssh_key_name
  subnet_id            = var.subnet_id
  security_group_ids   = var.security_group_ids
  iam_instance_profile = var.provider_config.instance_profile_name
  user_data            = var.user_data
  
  enable_monitoring    = var.enable_monitoring
  enable_ebs_optimized = var.provider_config.ebs_optimized
  
  tags = local.resource_tags
}

# Route to Azure provider
module "azure_instance" {
  count  = var.provider == "azure" ? 1 : 0
  source = "../../providers/azure/compute"

  instance_name       = var.instance_name
  vm_size             = local.instance_type
  resource_group_name = var.provider_config.resource_group_name
  location            = var.provider_config.location
  admin_username      = var.admin_username
  admin_ssh_key       = var.ssh_public_key
  
  os_publisher = var.provider_config.os_publisher
  os_offer     = var.provider_config.os_offer
  os_sku       = var.provider_config.os_sku
  
  subnet_id            = var.subnet_id
  enable_public_ip     = var.allow_public_access
  
  tags = local.resource_tags
}

# Route to GCP provider
module "gcp_instance" {
  count  = var.provider == "gcp" ? 1 : 0
  source = "../../providers/gcp/compute"

  instance_name  = var.instance_name
  machine_type   = local.instance_type
  project_id     = var.provider_config.project_id
  zone           = var.provider_config.zone
  
  boot_disk_image = var.provider_config.machine_image
  ssh_keys        = var.ssh_public_key != null ? ["${var.admin_username}:${var.ssh_public_key}"] : []
  
  subnet_id           = var.subnet_id
  enable_external_ip  = var.allow_public_access
  
  labels = local.resource_tags
}

# ============================================================================
# OUTPUT AGGREGATION (normalize outputs across providers)
# ============================================================================

locals {
  # Aggregate instance ID from whichever provider was used
  instance_id = try(
    module.aws_instance[0].instance_id,
    module.azure_instance[0].instance_id,
    module.gcp_instance[0].instance_id,
    ""
  )

  # Aggregate ARN/Resource ID
  instance_arn = try(
    module.aws_instance[0].arn,
    module.azure_instance[0].id,
    module.gcp_instance[0].self_link,
    ""
  )

  # Aggregate network information
  public_ip = var.allow_public_access ? try(
    module.aws_instance[0].public_ip,
    module.azure_instance[0].public_ip_address,
    module.gcp_instance[0].external_ip,
    null
  ) : null

  private_ip = try(
    module.aws_instance[0].private_ip,
    module.azure_instance[0].private_ip_address,
    module.gcp_instance[0].internal_ip,
    ""
  )

  # Aggregate state
  instance_state = try(
    module.aws_instance[0].instance_state,
    module.azure_instance[0].power_state,
    module.gcp_instance[0].status,
    "unknown"
  )

  # Zone information
  availability_zone = try(
    module.aws_instance[0].availability_zone,
    module.azure_instance[0].zone,
    module.gcp_instance[0].zone,
    ""
  )
}

# ============================================================================
# DEPENDENCY MANAGEMENT
# ============================================================================

# Ensure network resources exist before creating instance
resource "null_resource" "network_dependency" {
  count = var.subnet_id != null ? 1 : 0

  triggers = {
    subnet_id = var.subnet_id
  }

  lifecycle {
    precondition {
      condition     = var.subnet_id != null && var.subnet_id != ""
      error_message = "Valid subnet_id is required when specified"
    }
  }
}

# ============================================================================
# LIFECYCLE HOOKS
# ============================================================================

# Post-creation validation
resource "null_resource" "instance_ready" {
  depends_on = [
    module.aws_instance,
    module.azure_instance,
    module.gcp_instance
  ]

  triggers = {
    instance_id = local.instance_id
  }

  provisioner "local-exec" {
    command = "echo 'Instance ${local.instance_id} is ready on ${var.provider}'"
  }

  lifecycle {
    postcondition {
      condition     = local.instance_id != ""
      error_message = "Instance creation failed - no instance ID returned"
    }
  }
}

# Monitoring setup (if enabled)
resource "null_resource" "monitoring_setup" {
  count = var.enable_monitoring ? 1 : 0

  depends_on = [null_resource.instance_ready]

  triggers = {
    instance_id = local.instance_id
  }

  provisioner "local-exec" {
    command = "echo 'Monitoring enabled for instance ${local.instance_id}'"
  }
}

# ============================================================================
# OUTPUTS (conforming to API contract)
# ============================================================================

output "instance_id" {
  description = "Unique identifier of the compute instance"
  value       = local.instance_id
}

output "instance_arn" {
  description = "ARN/Resource ID of the instance"
  value       = local.instance_arn
}

output "instance_type" {
  description = "Provider-specific instance type used"
  value       = local.instance_type
}

output "instance_size" {
  description = "Normalized size"
  value       = var.instance_size
}

output "public_ip" {
  description = "Public IP address"
  value       = local.public_ip
}

output "private_ip" {
  description = "Private IP address"
  value       = local.private_ip
}

output "ssh_connection" {
  description = "SSH connection string"
  value       = local.ssh_connection
  sensitive   = true
}

output "state" {
  description = "Instance state"
  value       = local.instance_state
}

output "availability_zone" {
  description = "Availability zone"
  value       = local.availability_zone
}

output "tags" {
  description = "Applied tags"
  value       = local.resource_tags
}
