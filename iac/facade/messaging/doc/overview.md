# Messaging Facade Module

## WHAT: Unified Queues & Topics

The Messaging facade provides a simplified interface for AWS SQS/SNS, Azure Service Bus, and GCP Pub/Sub.

**Prerequisites**:
- Terraform `1.0.0+`
- Configured Cloud CLI for the target provider.

## WHY: Standardizing Event-Driven Patterns

### Problems Solved
- **Resource Selection**: Abstracting the choice between a `queue` (Point-to-Point) and a `topic` (Pub/Sub) into a single module.
- **Provider Consistency**: Hiding the platform-specific complexities of topic subscriptions and queue policies.

## HOW: Usage Example

```hcl
module "order_queue" {
  source   = "../../facade/messaging"
  provider = "aws"
  name     = "orders-inbound"
  type     = "queue"
}
```

## Examples and Tests
- **Unit Tests**: See `facade/messaging/messaging_test.go` for Terratest plan assertions.

---

**Last Updated**: 2026-01-14
