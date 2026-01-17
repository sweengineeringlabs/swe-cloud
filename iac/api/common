# Common local values for normalization and mapping
# Following CloudKit SEA architecture pattern

locals {
  # ============================================================================
  # COMPUTE INSTANCE TYPE MAPPINGS
  # ============================================================================
  
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

  # ============================================================================
  # STORAGE SIZE MAPPINGS (GB)
  # ============================================================================
  
  storage_sizes = {
    small  = 20
    medium = 100
    large  = 500
    xlarge = 1000
  }

  # ============================================================================
  # DATABASE INSTANCE TYPE MAPPINGS
  # ============================================================================
  
  database_instance_types = {
    aws = {
      small  = "db.t3.micro"
      medium = "db.t3.medium"
      large  = "db.r5.large"
      xlarge = "db.r5.xlarge"
    }
    azure = {
      small  = "GP_Gen5_2"
      medium = "GP_Gen5_4"
      large  = "GP_Gen5_8"
      xlarge = "GP_Gen5_16"
    }
    gcp = {
      small  = "db-n1-standard-1"
      medium = "db-n1-standard-2"
      large  = "db-n1-standard-4"
      xlarge = "db-n1-standard-8"
    }
    oracle = {
      small  = "VM.Standard.E4.Flex"
      medium = "VM.Standard.E4.Flex"
      large  = "VM.Standard3.Flex"
      xlarge = "VM.Standard3.Flex"
    }
  }

  # ============================================================================
  # NETWORK CIDR BLOCK MAPPINGS
  # ============================================================================
  
  network_cidrs = {
    small  = "10.0.0.0/24"   # 256 addresses
    medium = "10.0.0.0/20"   # 4,096 addresses
    large  = "10.0.0.0/16"   # 65,536 addresses
    xlarge = "10.0.0.0/12"   # 1,048,576 addresses
  }

  # ============================================================================
  # STORAGE CLASS MAPPINGS
  # ============================================================================
  
  storage_class_mapping = {
    aws = {
      standard   = "STANDARD"
      infrequent = "STANDARD_IA"
      archive    = "GLACIER"
      cold       = "DEEP_ARCHIVE"
    }
    azure = {
      standard   = "Hot"
      infrequent = "Cool"
      archive    = "Archive"
      cold       = "Archive"
    }
    gcp = {
      standard   = "STANDARD"
      infrequent = "NEARLINE"
      archive    = "COLDLINE"
      cold       = "ARCHIVE"
    }
    oracle = {
      standard   = "Standard"
      infrequent = "InfrequentAccess"
      archive    = "Archive"
      cold       = "Archive"
    }
  }

  # ============================================================================
  # REGION MAPPINGS
  # ============================================================================
  
  region_display_names = {
    # AWS
    "us-east-1"      = "US East (N. Virginia)"
    "us-west-2"      = "US West (Oregon)"
    "eu-west-1"      = "Europe (Ireland)"
    "ap-southeast-1" = "Asia Pacific (Singapore)"
    
    # Azure
    "eastus"       = "East US"
    "westus2"      = "West US 2"
    "westeurope"   = "West Europe"
    "southeastasia" = "Southeast Asia"
    
    # GCP
    "us-central1" = "US Central (Iowa)"
    "us-west1"    = "US West (Oregon)"
    "europe-west1" = "Europe West (Belgium)"
    "asia-southeast1" = "Asia Southeast (Singapore)"
  }

  # ============================================================================
  # ENVIRONMENT-SPECIFIC SETTINGS
  # ============================================================================
  
  environment_settings = {
    dev = {
      instance_count    = 1
      enable_ha         = false
      backup_retention  = 7
      log_retention     = 7
    }
    staging = {
      instance_count    = 2
      enable_ha         = true
      backup_retention  = 14
      log_retention     = 14
    }
    prod = {
      instance_count    = 3
      enable_ha         = true
      backup_retention  = 30
      log_retention     = 90
    }
  }
}
