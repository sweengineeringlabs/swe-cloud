# zero-cli Overview

## WHAT
The primary management tool for ZeroCloud. A command-line interface for creating workloads, networks, and volumes.

## WHY
| Problem | Solution |
|---------|----------|
| User Experience | Provides a clean, colored terminal interface for humans. |
| Automation | Allows scripting of private cloud resources via CLI. |

## HOW

```bash
# Spin up a container
zero workload up --id testsrv --image alpine

# List hardware nodes
zero node list
```

---

**Status**: Stable
