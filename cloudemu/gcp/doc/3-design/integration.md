# GCP Integration Guide

## WHAT
Integration details for GCP SDKs and tools.

## WHY
Allows standard Google Cloud tools to talk to CloudEmu.

## HOW

### 1. Unified Server Integration
The `gcp-control-facade` crate provides the routing logic.

**Wiring:**
```rust
use gcp_data_api::router::create_router;

let app = create_router(); // Listen on 4567
```

### 2. Emulator Environment Variables
GCP SDKs automatically switch to emulators when specific env vars are set.

**Variables:**
- `STORAGE_EMULATOR_HOST`: `http://localhost:4567`
- `FIRESTORE_EMULATOR_HOST`: `localhost:4567` (Note: no http://)
- `PUBSUB_EMULATOR_HOST`: `localhost:4567`

### 3. Terraform Integration
Use the `google` provider with custom endpoints.

```hcl
provider "google" {
  project = "test-project"
  region  = "us-central1"
}
# Note: Terraform GCP provider emulation support varies.
```

### 4. gcloud CLI
```bash
gcloud storage ls --project=test --endpoint=http://localhost:4567
```
