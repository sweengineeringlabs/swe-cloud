# Azure Module Overview

## WHAT
This module provides local emulation for Microsoft Azure core services. It implements the Azure REST API to ensure compatibility with Azure SDKs and tools.

### Supported Services
| Service | Type | Status |
|---------|------|--------|
| **Blob Storage** | Object Storage | ✅ Active |
| **Cosmos DB** | NoSQL Database | ✅ Active |
| **Service Bus** | Queue | ✅ Active |
| **Functions** | Compute | ✅ Active |
| **Key Vault** | Security | ✅ Active |
| **Retail Prices** | FinOps | ✅ Active |

## WHY
- **Consistency**: Test Azure logic alongside AWS/GCP in one tool.
- **Speed**: Eliminate cloud deployment latency.
- **Privacy**: Keep test data strictly local.

## HOW

### 1. Prerequisites
- **Azure CLI** (optional)
- **Azurite** (compatible client)

### 2. Configuration
Set the connection string to point to the local emulator (Port 10000):

```bash
export AZURE_STORAGE_CONNECTION_STRING="DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;AccountKey=Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==;BlobEndpoint=http://127.0.0.1:10000/devstoreaccount1;"
```

### 3. Usage Example (Python SDK)

```python
from azure.storage.blob import BlobServiceClient
import os

connect_str = os.getenv('AZURE_STORAGE_CONNECTION_STRING')
service_client = BlobServiceClient.from_connection_string(connect_str)

# Create container
container_client = service_client.create_container("my-container")
```

### 4. Examples and Tests
- **Integration Tests**: `cloudemu/azure/control-plane/azure-control-facade/tests/integration.rs`
- **Unit Tests**: Run `cargo test -p azure-control-core`
