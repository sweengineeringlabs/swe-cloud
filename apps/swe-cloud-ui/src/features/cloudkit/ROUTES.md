# CloudKit Routes

Cloud resource management and API exploration toolkit.

## Overview

| Route | Page | Description |
|-------|------|-------------|
| `/cloudkit` | cloudkit-overview | Feature landing page |
| `/cloudkit-resources` | cloudkit-resources | All resources overview |
| `/cloudkit-resources/:type` | cloudkit-resource-list | Resources by type |
| `/cloudkit-resources/:type/:id` | cloudkit-resource-detail | Resource details |
| `/cloudkit-operations` | cloudkit-operations | Operations dashboard |
| `/cloudkit-explorer` | cloudkit-explorer | API Explorer (context-aware) |

## Parameters

### Type
- **Description**: Resource type identifier
- **Examples**: `buckets`, `tables`, `functions`, `queues`
- **Required**: Yes (for type-filtered routes)

### ID
- **Description**: Resource identifier
- **Required**: Yes (for detail routes)

## Access Control

CloudKit requires one of the following roles:
- `developer`
- `devops`
- `admin`

```yaml
role_required: ["developer", "devops", "admin"]
```

## Context Awareness

The API Explorer is context-aware, meaning it adapts to the currently selected provider:

```yaml
- path: "/cloudkit-explorer"
  context_aware: true
```

## Navigation

- Main navigation: CloudKit overview
- Icon: `package`

## Sub-Navigation Icons

| Route | Icon |
|-------|------|
| Resources | `database` |
| Operations | `activity` |
| Explorer | `compass` |

## Examples

```
/cloudkit                        → CloudKit overview
/cloudkit-resources              → All resources
/cloudkit-resources/buckets      → All buckets
/cloudkit-resources/buckets/foo  → Bucket "foo" details
/cloudkit-operations             → Operations dashboard
/cloudkit-explorer               → API Explorer
```
