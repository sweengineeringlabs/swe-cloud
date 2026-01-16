# Oracle Module Overview

## WHAT
This module provides local emulation for Oracle Cloud Infrastructure (OCI) services, with a primary focus on FinOps and Cost Management APIs.

### Supported Services
| Service | Type | Status |
|---------|------|--------|
| **Metering** | FinOps/Pricing | ✅ Active |
| **Object Storage** | Object Storage | 🚧 Planned |

## WHY
- **FinOps Development**: Build cost analysis tools compatible with OCI pricing models.
- **Completeness**: Include Oracle in multi-cloud assessments.

## HOW

### 1. Prerequisites
- **Curl** or HTTP Client
- **Rust**

### 2. Configuration
The Oracle Pricing API is available at Port 4568.

```bash
export CLOUDEMU_ORACLE_PORT=4568
```

### 3. Usage Example

**Get Prices**:

```bash
curl http://localhost:4568/metering/api/v1/prices
```

Response:
```json
{
  "items": [
    {
      "partNumber": "B9F0-5A32-9D1C",
      "service": "Compute",
      "price": { "unitPrice": 0.0638, "description": "$0.0638 per OCPU Hour" }
    }
  ]
}
```

### 4. Examples and Tests
- **Integration Tests**: `cloudemu/oracle/control-plane/oracle-control-core/tests/pricing.rs`
- **Unit Tests**: Run `cargo test -p oracle-control-core`
