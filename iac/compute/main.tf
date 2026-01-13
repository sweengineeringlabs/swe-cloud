locals {
  # Mappings from generic size to provider-specific instance types/sizes
  aws_instance_types = {
    small  = "t2.micro"
    medium = "t2.medium"
    large  = "t2.large"
  }
  gcp_machine_types = {
    small  = "e2-small"
    medium = "e2-medium"
    large  = "e2-standard-4"
  }
  azure_vm_sizes = {
    small  = "Standard_B1s"
    medium = "Standard_B2s"
    large  = "Standard_DS2_v2"
  }
  test_instance_types = {
    small  = "test-small"
    medium = "test-medium"
    large  = "test-large"
  }
}

# AWS Module instantiation
module "aws_compute" {
  count  = var.provider == "aws" ? 1 : 0
  source = "../aws"

  instance_name = var.instance_name
  instance_type = local.aws_instance_types[var.instance_size]
  ami           = var.provider_config.ami
}

# GCP Module instantiation
module "gcp_compute" {
  count  = var.provider == "gcp" ? 1 : 0
  source = "../gcp"

  instance_name = var.instance_name
  machine_type  = local.gcp_machine_types[var.instance_size]
  project_id    = var.provider_config.project_id
  zone          = var.provider_config.zone
}

# Azure Module instantiation
module "azure_compute" {
  count  = var.provider == "azure" ? 1 : 0
  source = "../azure"

  instance_name       = var.instance_name
  vm_size             = local.azure_vm_sizes[var.instance_size]
  location            = var.provider_config.location
  resource_group_name = var.provider_config.resource_group_name
  admin_password      = try(var.provider_config.admin_password, null)
}

# Test Module instantiation
module "test_compute" {
  count  = var.provider == "test" ? 1 : 0
  source = "../test"

  instance_name = var.instance_name
  instance_type = local.test_instance_types[var.instance_size]
}
