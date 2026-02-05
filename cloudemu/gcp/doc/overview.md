# GCP Module Overview

## WHAT
This module provides local emulation for Google Cloud Platform (GCP) services. It implements the standard Google Cloud APIs (REST/gRPC) for compatibility with client libraries.

### Supported Services
| Service | Type | Status |
|---------|------|--------|
| **Cloud Storage** | Object Storage | ✅ Active |
| **Firestore** | NoSQL Database | ✅ Active |
| **Pub/Sub** | Messaging | ✅ Active |
| **Cloud Functions** | Compute | ✅ Active |
| **Secret Manager** | Security | ✅ Active |
| **Billing Catalog** | FinOps | ✅ Active |

## WHY
- **Unified Testing**: Validate multi-cloud architectures locally.
- **Simplicity**: No need for complex IAM service account setups for dev.

## HOW

### 1. Prerequisites
- **gcloud CLI** (optional)
- **Google Cloud SDK**

### 2. Configuration
Set the emulator host environment variables (Port 4567):

```bash
export STORAGE_EMULATOR_HOST=http://localhost:4567
export FIRESTORE_EMULATOR_HOST=localhost:4567
export PUBSUB_EMULATOR_HOST=localhost:4567
```

### 3. Usage Example (gcloud)

```bash
# List buckets
gcloud storage ls --project=test-project

# Topics
gcloud pubsub topics create my-topic --project=test-project
```

### 4. Examples and Tests
- **Integration Tests**: `cloudemu/gcp/control-plane/gcp-control-facade/tests/integration.rs`
- **Unit Tests**: Run `cargo test -p gcp-control-core`
