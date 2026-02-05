# Azure Integration Guide

## WHAT
Details on integrating the Azure emulator with SDKs and tools.

## WHY
Enables developers to replace the real Azure cloud with CloudEmu.

## HOW

### 1. Unified Server Integration
The `azure-control-facade` crate exports a router builder.

**Wiring:**
```rust
use azure_data_api::router::create_router;

let app = create_router(); // Listen on 10000
```

### 2. Azurite Compatibility
We aim for protocol parity with Azurite, allowing existing Azurite configs to work.

**Connection String:**
`DefaultEndpointsProtocol=http;AccountName=devstoreaccount1;AccountKey=Eby8vd...;BlobEndpoint=http://127.0.0.1:10000/devstoreaccount1;`

### 3. Azure CLI
Support is partial. Uses `--connection-string` where possible.

```bash
az storage response-blob list --connection-string "..."
```

### 4. Python SDK
```python
BlobServiceClient.from_connection_string("DefaultEndpointsProtocol=http;...")
```
