# CloudEmu Routes

Cloud emulation services with multi-provider support.

## Overview

| Route | Page | Description |
|-------|------|-------------|
| `/cloudemu` | cloudemu-overview | Feature landing page |
| `/cloudemu/:provider` | cloudemu-provider | Provider dashboard (AWS, Azure, GCP, Zero) |
| `/cloudemu/:provider/:service` | cloudemu-service | Service resource list |
| `/cloudemu/:provider/:service/new` | cloudemu-service-new | Create new resource |
| `/cloudemu/:provider/:service/:id` | cloudemu-service-detail | Resource details |
| `/cloudemu/logs` | cloudemu-logs | Request logs viewer |

## Parameters

### Provider
- **Pattern**: `aws|azure|gcp|zero`
- **Required**: Yes
- **Sets Context**: `{ provider: ":provider" }`

### Service
- **Pattern**: `s3|dynamodb|lambda|sqs|sns|ec2|blobs|keyvault|functions|storage|pubsub|compute|queue`
- **Required**: Yes (for service routes)

### ID
- **Pattern**: Any string
- **Required**: Yes (for detail routes)

## Context

The provider route sets the active provider context:

```yaml
context:
  set: { provider: ":provider" }
```

This context is used by child routes and components to filter data.

## Workflows

### Create Resource
- **Route**: `/cloudemu/:provider/:service/new`
- **Workflow**: `create-resource`
- **Data**: `{ resource_type: ":service" }`

## Navigation

- Main navigation: CloudEmu overview
- Icon: `cloud`

## Examples

```
/cloudemu              → CloudEmu overview
/cloudemu/aws          → AWS provider dashboard
/cloudemu/aws/s3       → S3 buckets list
/cloudemu/aws/s3/new   → Create new S3 bucket
/cloudemu/aws/s3/my-bucket → Bucket details
/cloudemu/logs         → All request logs
```
