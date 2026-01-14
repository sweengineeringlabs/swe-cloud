# Compute Module Toolchain

## Overview
This module utilizes a standardized toolchain for infrastructure definition and automated verification.

## Tools

### Terraform
| | |
|---|---|
| **What** | Infrastructure as Code Engine |
| **Version** | `1.0.0+` |
| **Install** | `brew install terraform` / `choco install terraform` |

**Why we use it**: Industry standard for multi-cloud resource provisioning.

### Terratest (Go)
| | |
|---|---|
| **What** | Infrastructure Testing Framework |
| **Version** | `1.19+ (Go)` |
| **Install** | `go get github.com/gruntwork-io/terratest` |

**Why we use it**: Enables programmatic validation of resource plans and attributes before deployment.

## Version Matrix
| Tool | Minimum | Recommended |
|------|---------|-------------|
| Terraform | 1.0.0 | 1.5.0+ |
| Go | 1.19 | 1.21+ |
| AWS Provider | 4.0 | 5.0+ |

## Verification
Run the module validation:
```bash
go test -v ./compute_test.go
```
