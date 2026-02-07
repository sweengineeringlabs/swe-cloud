terraform {
  required_providers {
    null = {
      source = "hashicorp/null"
      version = "3.2.1"
    }
  }
}

variable "bucket_name" {
  default = "my-zero-bucket"
}

variable "table_name" {
  default = "my-zero-table"
}

# 1. Provision a ZeroStore Bucket
resource "null_resource" "zero_bucket" {
  triggers = {
    bucket = var.bucket_name
  }

  provisioner "local-exec" {
    command = "zero store create --name ${self.triggers.bucket}"
  }

  # Cleanup on destroy
  # provisioner "local-exec" {
  #   when    = destroy
  #   command = "zero store delete --name ${self.triggers.bucket}"
  # }
}

# 2. Provision a ZeroDB Table
resource "null_resource" "zero_table" {
  triggers = {
    table = var.table_name
  }

  provisioner "local-exec" {
    command = "zero db create --name ${self.triggers.table} --pk id"
  }
}

# 3. Deploy a ZeroFunction
resource "null_resource" "zero_func" {
  provisioner "local-exec" {
    command = "zero func deploy --name hello-tf --code 'console.log(\"Hello from Terraform!\")' --handler index.handler"
  }
}

output "status" {
  value = "ZeroCloud resources provisioned via CLI bridge."
}
