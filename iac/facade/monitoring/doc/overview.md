# Monitoring Facade Module

## WHAT: Unified Metric Alerts & Alarms

The Monitoring facade provides a unified interface for AWS CloudWatch Alarms, Azure Monitor Metric Alerts, and GCP Cloud Monitoring Alert Policies.

**Prerequisites**:
- Terraform `1.0.0+`
- Configured Cloud CLI for the target provider.

## WHY: Multi-Cloud Observability Consistency

### Problems Solved
- **Metric Mapping**: Normalizing metric names like `CPUUtilization` (AWS) vs `Percentage CPU` (Azure).
- **Threshold Normalization**: Handling different platform thresholds (e.g., 80% vs 0.8).

## HOW: Usage Example

```hcl
module "cpu_alarm" {
  source      = "../../facade/monitoring"
  provider_name = "aws"
  alarm_name  = "cpu-high"
  metric_name = "CPUUtilization"
  threshold   = 85
}
```

## Examples and Tests
- **Unit Tests**: See `facade/monitoring/monitoring_test.go` for Terratest plan assertions.

---

**Last Updated**: 2026-01-14
