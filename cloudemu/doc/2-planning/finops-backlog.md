# FinOps and Cost Estimation Tools Backlog

This document outlines the roadmap and backlog for implementing pricing and billing APIs in CloudEmu. These APIs enable users to build and test FinOps, cost estimation, and budget management tools against the emulator.

## 🔄 High Level Goals
- **Parity**: Emulate the public pricing APIs of AWS, Azure, and GCP.
- **Mock Data**: Serve realistic, consistent pricing data from the local `metadata.db`.
- **No Auth**: Follow the providers' patterns (some are public, some require auth).

---

## 🟢 Priority 1: AWS Price List Service API

**Target Endpoint**: `api.pricing.us-east-1.amazonaws.com`
**Protocol**: JSON 1.1 (AWS Standard)

### Phase 1: Foundation & Routing
- [ ] **Create Service Module**: `cloudemu/aws/control-plane/aws-control-core/src/services/pricing.rs`
- [ ] **Update Router**: Register `api.pricing.{region}.amazonaws.com` or `/pricing` routes in `aws-control-api`.
- [ ] **Define Shapes**: Create structs for `GetServicesRequest`, `GetServicesResponse`, `GetProductsRequest`, etc.

### Phase 2: Operations
- [ ] **Implement `GetServices`**:
    - [ ] Return list of available services (e.g., "AmazonEC2", "AmazonS3").
    - [ ] Support `ServiceCode` filter.
- [ ] **Implement `GetAttributeValues`**:
    - [ ] Return attributes for a service (e.g., `location` = `US East (N. Virginia)`).
- [ ] **Implement `GetProducts`**:
    - [ ] Design SQLite schema for pricing products (or hardcode JSON responses for v1).
    - [ ] Support filters (e.g., `PreInstalledSw`, `location`, `operatingSystem`).
    - [ ] Return standard AWS Price List JSON format.

### Phase 3: Integration
- [ ] **CLI Validation**: Verify using `aws pricing get-products --service-code AmazonEC2 --region us-east-1`.

---

## 🟡 Priority 2: Azure Retail Prices API

**Target Endpoint**: `prices.azure.com`
**Protocol**: REST / OData

### Tasks
- [ ] **Create Facade**: `cloudemu/azure/control-plane/azure-control-core/src/services/pricing.rs`
- [ ] **Implement `GET /api/retail/prices`**:
    - [ ] Support OData `$filter` queries (e.g., `serviceName eq 'Virtual Machines'`).
    - [ ] Return JSON response with retail prices.
    - [ ] Map "Consumption" prices to internal S3/DynamoDB mocks.

---

## 🟡 Priority 3: GCP Cloud Billing Catalog API

**Target Endpoint**: `cloudbilling.googleapis.com`
**Protocol**: gRPC / REST

### Tasks
- [ ] **Create Facade**: `cloudemu/gcp/control-plane/gcp-control-core/src/services/billing.rs`
- [ ] **Implement `ListServices`**:
    - [ ] Return `services/6F81-5844-456A` (Compute Engine), etc.
- [ ] **Implement `ListSkus`**:
    - [ ] Return SKUs for a given parent service.

---

## 💾 Data Strategy implementation details

Since we cannot replicate the entire AWS price list (gigabytes of data), we will:
1.  **Seed Minimal Data**: Create a standard set of "Free Tier" or "Standard" prices for core services (S3 Standard, EC2 t3.micro).
2.  **Storage**: Use a new table `pricing_products` in `metadata.db`.
3.  **Schema**:
    ```sql
    CREATE TABLE pricing_products (
        sku TEXT PRIMARY KEY,
        service_code TEXT,
        region TEXT,
        attributes_json TEXT,
        on_demand_price_usd REAL
    );
    ```
