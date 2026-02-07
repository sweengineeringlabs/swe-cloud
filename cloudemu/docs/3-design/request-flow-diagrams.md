# Request Flow Diagrams

Visual representations of request flows through CloudEmu's multi-cloud architecture.

## AWS Request Flow

```mermaid
sequenceDiagram
    participant Client
    participant Server as CloudEmu Server<br/>(Port 4566)
    participant AWS as AWS Provider
    participant Router as Axum Router
    participant Service as Service Handler<br/>(S3/DynamoDB/etc)
    participant Storage as Storage Engine<br/>(SQLite + FS)

    Client->>Server: HTTP Request<br/>(e.g., PUT /bucket/key)
    Server->>AWS: Route by port
    AWS->>Router: Convert to Axum Request
    Router->>Service: Dispatch by path
    Service->>Storage: Persist data
    Storage-->>Service: Success
    Service-->>Router: HTTP Response
    Router-->>AWS: Response
    AWS-->>Server: Response
    Server-->>Client: HTTP 200 OK
```

## Azure Request Flow

```mermaid
sequenceDiagram
    participant Client
    participant Server as CloudEmu Server<br/>(Port 4567)
    participant Azure as Azure Provider
    participant Blob as Blob Service
    participant Storage as Storage Engine

    Client->>Server: HTTP Request<br/>/devstoreaccount1/container/blob
    Server->>Azure: Route by port
    Azure->>Azure: Strip account name
    Azure->>Blob: Dispatch to service
    Blob->>Blob: Parse request<br/>(container vs blob op)
    Blob->>Storage: Store blob data
    Storage-->>Blob: Success
    Blob-->>Azure: XML Response
    Azure-->>Server: HTTP Response
    Server-->>Client: HTTP 200 + XML
```

## GCP Request Flow (Skeleton)

```mermaid
sequenceDiagram
    participant Client
    participant Server as CloudEmu Server<br/>(Port 4568)
    participant GCP as GCP Provider
    
    Client->>Server: HTTP Request
    Server->>GCP: Route by port
    GCP-->>Server: 404 Not Implemented
    Server-->>Client: HTTP 404
    
    Note over GCP: Skeleton implementation<br/>No services yet
```

## Multi-Cloud Server Startup

```mermaid
sequenceDiagram
    participant Main as main.rs
    participant Config as AppConfig
    participant AWS as AWS Task
    participant Azure as Azure Task
    participant GCP as GCP Task

    Main->>Config: Parse CLI args
    Config-->>Main: Ports + flags
    
    par AWS Startup
        Main->>AWS: spawn(start_aws_provider)
        AWS->>AWS: Initialize Emulator
        AWS->>AWS: Bind to port 4566
        AWS-->>Main: Server running
    and Azure Startup
        Main->>Azure: spawn(start_provider_server)
        Azure->>Azure: Initialize AzureProvider
        Azure->>Azure: Bind to port 4567
        Azure-->>Main: Server running
    and GCP Startup
        Main->>GCP: spawn(start_provider_server)
        GCP->>GCP: Initialize GcpProvider
        GCP->>GCP: Bind to port 4568
        GCP-->>Main: Server running
    end

    Note over Main: All providers listening
```

## Generic Provider Server Flow

```mermaid
sequenceDiagram
    participant Axum as Axum Handler
    participant Server as start_provider_server
    participant Provider as CloudProviderTrait<br/>impl
    participant Core as cloudemu_core

    Axum->>Server: handle_request(axum::Request)
    Server->>Server: Convert axum::Request<br/>to core::Request
    Server->>Provider: provider.handle_request()
    Provider->>Provider: Route to service
    Provider->>Core: Use core types
    Core-->>Provider: core::Response
    Provider-->>Server: core::Response
    Server->>Server: Convert core::Response<br/>to axum::Response
    Server-->>Axum: axum::Response
```

## Storage Engine Data Flow

```mermaid
flowchart TD
    A[HTTP Request] --> B{Operation Type}
    B -->|Metadata| C[SQLite Database]
    B -->|Object Data| D[Filesystem]
    
    C --> E[Buckets Table]
    C --> F[Objects Table]
    C --> G[Metadata Table]
    
    D --> H[.cloudemu/data/]
    H --> I[bucket-name/]
    I --> J[object-key]
    
    E --> K[Combine Results]
    F --> K
    G --> K
    J --> K
    
    K --> L[Response]
```

## Error Handling Flow

```mermaid
sequenceDiagram
    participant Client
    participant Service
    participant Storage
    
    Client->>Service: Invalid Request
    Service->>Service: Validate
    Service-->>Client: 400 Bad Request
    
    Client->>Service: Valid Request
    Service->>Storage: Operation
    Storage-->>Service: Error (e.g., NoSuchBucket)
    Service->>Service: Map to AWS error
    Service-->>Client: 404 + XML Error
```

## Key Architectural Decisions

### Port-Based Routing

CloudEmu uses distinct ports to route requests to different cloud providers:
- **4566** → AWS (existing control-plane)
- **4567** → Azure (new generic server)
- **4568** → GCP (new generic server)

This allows clients to target specific clouds without request inspection.

### Two Server Patterns

1. **AWS**: Uses existing `control-plane::gateway::ingress::start()` for backward compatibility
2. **Azure/GCP**: Uses generic `server::start_provider_server()` with `CloudProviderTrait`

This hybrid approach maintains AWS compatibility while enabling new provider architectures.

### Request Conversion

The generic server converts between:
- `axum::Request` (HTTP layer)
- `cloudemu_core::Request` (provider layer)

This abstraction allows providers to be framework-agnostic.

---

**Last Updated**: 2026-01-14
