# Oracle Integration Guide

## WHAT
Integration details for Oracle OCI emulation, specifically for FinOps tools.

## WHY
Enable cost management tools to consume emulated pricing data.

## HOW

### 1. Unified Server Integration
The `oracle-control-core` provides the implementation of `CloudProviderTrait`.

**Wiring:**
```rust
let oracle_provider = OracleProvider::new(...);
// Wrapped in Axum router for port 4568
```

### 2. Focus Integration (FinOps)
This emulator emits data compatible with the CloudCost ingestion engine.

**Ingestion Flow:**
1. CloudCost requests `GET /metering/api/v1/prices`.
2. Oracle Emulator queries internal DB.
3. Returns JSON matching OCI Metering API format.

### 3. HTTP Client
Since OCI CLI emulation is limited, direct HTTP usage is recommended.

```bash
curl http://localhost:4568/metering/api/v1/prices
```
