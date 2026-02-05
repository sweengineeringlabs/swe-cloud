# ZeroCloud Provider Guide

This guide explains how to use the **ZeroCloud** provider within the IAC framework.

## Overview

**ZeroCloud** is the local private cloud emulator included in the CloudKit ecosystem (`cloudemu`). It allows you to develop, test, and run cloud infrastructure locally without incurring costs or needing internet connectivity.

In the IAC framework, ZeroCloud is treated as a first-class provider (`provider_name = "zero"`).

## Architecture: The AWS Shim

ZeroCloud's Control Plane is designed to be **wire-compatible** with AWS APIs for key services. This allows the IAC framework to use the robust **HashiCorp AWS Terraform Provider** as a client driver (SPI) for ZeroCloud.

When you select `provider_name = "zero"`:
1.  The IAC Facade routes your request to `iac/zero/core/...`.
2.  The Core module defines resources using the standard `aws_*` resource types.
3.  The Provider Configuration (`iac/zero/spi`) redirects these requests to `http://localhost:8080`.

## Supported Services

The following core services are fully supported:

| Feature | Zero Implementation | Interface Mapped To |
| :--- | :--- | :--- |
| **Object Storage** | `ZeroStore` | AWS S3 |
| **NoSQL Database** | `ZeroDB` | AWS DynamoDB |
| **Virtual Machines** | `ZeroCompute` | AWS EC2 |
| **Serverless Functions** | `ZeroFunc` | AWS Lambda |
| **Messaging** | `ZeroQueue` | AWS SQS / SNS |
| **Networking** | `ZeroNet` | AWS VPC |
| **Identity** | `ZeroID` | AWS IAM |

## Configuration

To use ZeroCloud, you must configure the AWS provider to point to your local emulator endpoint.

**Example Provider Config:**

```hcl
provider "aws" {
  alias                       = "zero"
  region                      = "us-east-1"
  skip_credentials_validation = true
  skip_requesting_account_id  = true
  skip_metadata_api_check     = true
  s3_use_path_style           = true
  
  endpoints {
    ec2      = "http://localhost:8080"
    s3       = "http://localhost:8080"
    dynamodb = "http://localhost:8080"
    lambda   = "http://localhost:8080"
    sqs      = "http://localhost:8080"
    iam      = "http://localhost:8080"
  }
}
```

## Usage Example

```hcl
module "my_app" {
  source        = "./facade/compute"
  provider_name = "zero"
  instance_name = "local-vm-01"
  instance_size = "small" # Maps to "zero.micro"
}
```

## Testing

Integration tests for ZeroCloud are located in `iac/zero/test/integration_test.go`. They require the `cloudemu` server to be running (`cargo run -p cloudemu-server`).
