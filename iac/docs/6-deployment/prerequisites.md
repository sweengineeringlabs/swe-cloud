# Prerequisites Guide

This document outlines the software and configurations required to use the Multi-Cloud IAC framework.

## 1. Core Tooling

The framework requires the following base tools installed on your local machine or CI/CD runner:

| Tool | Minimum Version | Required For |
| :--- | :--- | :--- |
| **Terraform** | `1.0.0+` | Infrastructure Provisioning |
| **Go** | `1.19+` | Running Terratests |
| **Git** | `2.0+` | Version Control |

## 2. Cloud Provider CLI Tools

You must have the CLI tools for each cloud provider you intend to deploy to:

*   **AWS CLI**: Version 2.x recommended. [Install Guide](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)
*   **Azure CLI**: `az` command available. [Install Guide](https://learn.microsoft.com/en-us/cli/azure/install-azure-cli)
*   **Google Cloud SDK**: `gcloud` command available. [Install Guide](https://cloud.google.com/sdk/docs/install)

## 3. Provider Authentication

Ensure you are authenticated with the following environment variables or configuration files:

### AWS
- `AWS_ACCESS_KEY_ID`
- `AWS_SECRET_ACCESS_KEY`
- `AWS_DEFAULT_REGION`

### Azure
- `ARM_SUBSCRIPTION_ID`
- `ARM_CLIENT_ID`
- `ARM_CLIENT_SECRET`
- `ARM_TENANT_ID`
- *Or run `az login`*

### GCP
- `GOOGLE_CREDENTIALS` (path to JSON key)
- `GOOGLE_PROJECT`
- `GOOGLE_REGION`

## 4. Local Environment Setup

Before running tests or deploying, ensure Go dependencies are initialized:

```bash
cd iac/
go mod tidy
```

And initialize Terraform modules in the specific facade or example directory:

```bash
cd iac/facade/compute
terraform init
```
