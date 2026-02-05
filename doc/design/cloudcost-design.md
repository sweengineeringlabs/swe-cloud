# CloudCost: Standalone FinOps Engine Design

## 1. Vision
**CloudCost** is a multi-cloud FinOps engine designed to run locally, providing cost estimation, budget tracking, and optimization recommendations. It is built to work seamlessly with **CloudEmu** for development and testing, while supporting real cloud providers (AWS, Azure, GCP) for production usage.

## 2. Core Value Proposition
- **Offline First**: Develop FinOps logic (alerting, forecasting) against CloudEmu without waiting for 24h cloud billing delays.
- **Unified Schema**: Normalizes pricing data across providers using the **FOCUS** ([FinOps Open Cost & Usage Specification](https://focus.finops.org/)) standard.
- **Safety**: Test "destructing" cost-saving actions (e.g., "Delete unused EBS volumes") against CloudEmu first.

## 3. Architecture

### 3.1 Technology Stack
- **Language**: Rust (for performance and type safety).
- **Interface**: CLI (Command Line Interface) + TUI (Terminal UI via `ratatui` or `crossterm`).
- **Storage**: Local SQLite (or reuse `cloudemu` metadata format).

### 3.2 Modules
1.  **Ingestion Layer (`cloudcost-ingest`)**:
    *   Clients for AWS Pricing API, Azure Retail Prices, GCP Billing.
    *   *CloudEmu Advantage*: Uses the exact same client code, just pointing to `localhost`.
2.  **Normalization Layer (`cloudcost-core`)**:
    *   **FOCUS Integration**: Maps provider-specific pricing/usage to the **FOCUS 1.0** standard.
    *   **Standard Columns**: `Provider`, `ServiceCategory` (e.g., "Compute"), `ResourceId`, `BilledCost`, `EffectiveCost`.
    *   *Benefit*: The Estimation Engine only needs to understand this single schema, ignoring proprietary formats like AWS CUR or Azure Exports.
3.  **Estimation Engine (`cloudcost-calc`)**:
    *   Input: Infrastructure definition (Terraform/HCL parsing) OR Live metrics (CloudWatch/Monitor).
    *   Output: Hourly/Monthly cost projection.
4.  **Policy Engine (`cloudcost-guard`)**:
    *   Define rules: "Warn if EC2 cost > $50/day", "Error if GP2 volume is unattached > 7 days".

## 4. Workflows

### Scenario A: Pre-Deployment Check (IAC)
```bash
# Parses terraform plan, queries CloudEmu Pricing API, estimates cost
cloudcost estimate --path ./iac/examples/multi-cloud --provider cloudemu
```

### Scenario B: Live Optimization
```bash
# Queries CloudEmu Metrics API (Usage) + Pricing API (Rates)
cloudcost analyze --target localhost:4566
> Report:
  - Idle EBS Volume: vol-123 (Cost: $5.00/mo) -> Action: DELETE
  - Oversized EC2: i-abc (CPU < 5%) -> Action: DOWNSIZE t3.medium -> t3.micro
```

## 5. Implementation Roadmap

### Phase 1: The Scanner (v0.1)
- [ ] Scaffold `apps/cloudcost`.
- [ ] Implement `ingest` for AWS/Azure/GCP pricing APIs.
- [ ] Basic CLI: `cloudcost prices list --service ec2`.

### Phase 2: The Estimator (v0.2)
- [ ] Integration with Terraform Plan JSON.
- [ ] Simple match logic: `aws_instance.t3_micro` -> `Price(0.0104)`.

### Phase 3: The Advisor (v1.0)
- [ ] Metric-driven rightsizing (connecting to CloudEmu CloudWatch metrics).
- [ ] Budget alerts system.

## 6. Why Rust?
Rust fits perfectly here because we can share code with `CloudEmu` (e.g., the data types for Pricing/Billing) but compile it into a fast, standalone binary that can run in CI/CD pipelines.
