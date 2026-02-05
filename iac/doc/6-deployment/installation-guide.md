# Installation & Deployment Guide

This guide describes how to install and deploy infrastructure using the Multi-Cloud IAC framework.

## 1. Installation

Since this is an IAC project, "installation" refers to cloning the repository and preparing the local environment.

### Clone the Repository
```bash
git clone <repository-url>
cd iac
```

### Dependency Check
Run the prerequisite check to ensure your environment is ready:
1. Ensure `terraform` is in your PATH.
2. Ensure `go` is installed for testing.
3. (Optional) Install **CloudEmu** for local ZeroCloud testing (`cargo install --path cloudemu/server`).
4. Run `go mod tidy` in the root of the `iac` folder.

## 2. Deployment Workflow

The framework follows the **SEA (Service-Engine-Adapter)** architecture. You should deploy resources through the `facade/` layer.

### Step 1: Initialize
Navigate to the desired service facade and initialize Terraform:

```bash
cd facade/storage
terraform init
```

### Step 2: Configure
Create a `terraform.tfvars` file or prepare environment variables.
Example `terraform.tfvars`:
```hcl
provider_name = "aws"
project_name = "my-awesome-project"
environment  = "dev"
bucket_name  = "my-data-bucket-unique-123"
```

### Step 3: Plan and Apply
Run a plan to verify the changes:
```bash
terraform plan
```

Apply the configuration:
```bash
terraform apply
```

## 3. Deployment Examples

For complex multi-cloud deployments, refer to the `examples/` directory:

- `examples/web-app`: Standard 3-tier web application.
- `examples/multi-cloud`: A setup using AWS, Azure, and GCP simultaneously.

### Deploying an Example:
```bash
cd examples/multi-cloud
terraform init
terraform apply
```

## 4. Post-Deployment Validation

After deployment, you can verify your infrastructure using the included Go validation test:

```bash
go test -v ./validation_test.go
```

Or run the Terratest suite for a specific service:

```bash
go test -v ./facade/storage/...
```
