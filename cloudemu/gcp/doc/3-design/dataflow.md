# GCP Dataflow

## WHAT
Data flow for GCP operations.

## WHY
Trace GCP request processing.

## HOW

### Cloud Storage Upload
1. **Ingress**: POST /upload/storage/v1/b/bucket/o.
2. **Storage**:
   - Metadata -> SQLite `gcp_objects`.
   - Content -> Filesystem.
3. **Response**: JSON resource representation.
