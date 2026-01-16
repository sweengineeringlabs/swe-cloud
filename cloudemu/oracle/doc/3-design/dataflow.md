# Oracle Dataflow

## WHAT
Data flow for Oracle operations.

## WHY
Trace OCI request processing.

## HOW

### List Prices
1. **Ingress**: GET /metering/api/v1/prices.
2. **Logic**: `PricingService` queries `pricing_products` table.
3. **Response**: JSON array of price items.
