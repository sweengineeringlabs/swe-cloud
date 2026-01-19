# IAC Routes

Infrastructure as Code deployment and management.

## Overview

| Route | Page | Description |
|-------|------|-------------|
| `/iac` | iac-overview | Feature landing page |
| `/iac-modules` | iac-modules | Module library |
| `/iac-modules/:module` | iac-module-detail | Module details |
| `/iac-deploy` | iac-deploy | Deploy infrastructure (workflow) |
| `/iac-deployments` | iac-deployments | Deployment history |
| `/iac-deployments/:id` | iac-deployment-detail | Deployment details |
| `/iac-state` | iac-state | Infrastructure state |
| `/iac-plans` | iac-plans | Execution plans |
| `/iac-plans/:id` | iac-plan-detail | Plan details |

## Parameters

### Module
- **Description**: Module name/identifier
- **Examples**: `vpc-network`, `eks-cluster`, `rds-postgres`
- **Required**: Yes (for module detail)

### ID
- **Description**: Deployment or plan identifier
- **Examples**: `deploy-456`, `plan-789`
- **Required**: Yes (for detail routes)

## Access Control

IAC requires elevated privileges:
- `devops`
- `admin`

```yaml
role_required: ["devops", "admin"]
```

Note: `developer` role does NOT have IAC access.

## Workflows

### Deploy Infrastructure
- **Route**: `/iac-deploy`
- **Workflow**: `deploy-infrastructure`
- **Icon**: `upload-cloud`

This workflow guides users through:
1. Module selection
2. Configuration
3. Plan review
4. Deployment approval
5. Execution monitoring

## Navigation

- Main navigation: Infrastructure
- Icon: `git-branch`

## Sub-Navigation Icons

| Route | Icon |
|-------|------|
| Modules | `box` |
| Deploy | `upload-cloud` |
| Deployments | `list` |
| State | `file-text` |
| Plans | `clipboard` |

## Dynamic Titles

Detail routes use dynamic titles:
- `/iac-deployments/:id` → "Deployment #456"
- `/iac-plans/:id` → "Plan #789"

## Examples

```
/iac                        → IAC overview
/iac-modules                → Browse modules
/iac-modules/vpc-network    → VPC Network module details
/iac-deploy                 → Start deployment workflow
/iac-deployments            → View all deployments
/iac-deployments/deploy-123 → Deployment #123 details
/iac-state                  → Current infrastructure state
/iac-plans                  → View execution plans
/iac-plans/plan-456         → Plan #456 details
```
