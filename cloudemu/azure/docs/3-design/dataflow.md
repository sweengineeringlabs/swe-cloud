# Azure Dataflow

## WHAT
Data flow for Azure operations.

## WHY
Visualize request processing for Azure services.

## HOW

### Blob Upload Flow
1. **Ingress**: PUT request to `http://localhost:10000/account/container/blob`.
2. **Auth**: Checks SAS token or Shared Key (mocked).
3. **Storage**:
   - Blob metadata -> SQLite `azure_blobs`.
   - Content -> Filesystem.
4. **Response**: 201 Created.
