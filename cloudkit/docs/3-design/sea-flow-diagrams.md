# SEA Layer Flow Diagrams

Visual representations of data flow through CloudKit's Stratified Encapsulation Architecture (SEA).

## Request Flow Through SEA Layers

```mermaid
sequenceDiagram
    participant App as Application Code
    participant Facade as Facade Layer<br/>(cloudkit)
    participant API as API Layer<br/>(cloudkit_api)
    participant Core as Core Layer<br/>(cloudkit_core)
    participant SPI as SPI Layer<br/>(cloudkit_spi)
    participant Provider as Cloud Provider<br/>(AWS/Azure/GCP)

    App->>Facade: cloud.storage().put_object()
    Facade->>API: StorageTrait::put_object()
    API->>Core: CloudContext::execute()
    Core->>Core: Apply retry policy
    Core->>SPI: Provider::upload()
    SPI->>Provider: AWS SDK call
    Provider-->>SPI: Response
    SPI-->>Core: Result
    Core->>Core: Collect metrics
    Core-->>API: Result
    API-->>Facade: Result
    Facade-->>App: Result<()>
```

## Multi-Cloud Provider Selection

```mermaid
flowchart TD
    A[Application] --> B{CloudKit::builder<br/>select provider}
    
    B -->|aws| C[AWS Configuration]
    B -->|azure| D[Azure Configuration]
    B -->|gcp| E[GCP Configuration]
    
    C --> F[AwsProvider]
    D --> G[AzureProvider]
    E --> H[GcpProvider]
    
    F --> I[CloudContext]
    G --> I
    H --> I
    
    I --> J[Unified API:<br/>storage, queues,<br/>database, messaging]
    
    J --> K[Application Logic<br/>Provider-Agnostic]
```

## Retry and Error Handling

```mermaid
sequenceDiagram
    participant App
    participant Core as Core Layer
    participant Provider
    
    App->>Core: execute(operation)
    
    loop Retry Logic
        Core->>Provider: attempt operation
        alt Success
            Provider-->>Core: Ok(result)
            Core-->>App: Ok(result)
        else Retriable Error
            Provider-->>Core: Err (transient)
            Core->>Core: wait + backoff
        else Non-Retriable
            Provider-->>Core: Err (permanent)
            Core-->>App: Err (mapped)
        end
    end
```

## Storage Operation Data Flow

```mermaid
flowchart LR
    A[put_object<br/>request] --> B[API Trait]
    B --> C[Core Context]
    C --> D{Provider?}
    
    D -->|AWS| E[S3 SDK]
    D -->|Azure| F[Blob SDK]
    D -->|GCP| G[GCS SDK]
    
    E --> H[AWS S3]
    F --> I[Azure Blob]
    G --> J[GCP Storage]
    
    H --> K[Result]
    I --> K
    J --> K
    
    K --> L[Metrics<br/>Collection]
    L --> M[Application]
```

## Configuration and Initialization

```mermaid
sequenceDiagram
    participant App
    participant Builder as CloudKitBuilder
    participant Config as Configuration
    participant Context as CloudContext
    participant Provider

    App->>Builder: CloudKit::aws()
    Builder->>Config: Load env vars
    Config-->>Builder: AWS config
    Builder->>Provider: Initialize AwsProvider
    Provider-->>Builder: Provider instance
    Builder->>Context: Create CloudContext
    Context-->>Builder: Context
    Builder-->>App: CloudKit instance
    
    Note over App: Ready to make API calls
```

## Observer Pattern for Metrics

```mermaid
sequenceDiagram
    participant Core
    participant Observer as MetricsObserver
    participant Collector as MetricsCollector
    
    Core->>Core: Start operation
    Core->>Observer: notify(OperationStart)
    Observer->>Collector: record_start()
    
    Core->>Core: Execute
    
    alt Success
        Core->>Observer: notify(OperationSuccess)
        Observer->>Collector: record_success()<br/>record_duration()
    else Failure
        Core->>Observer: notify(OperationFailure)
        Observer->>Collector: record_failure()<br/>record_error_type()
    end
```

## WASM Compatibility Layer

```mermaid
flowchart TD
    A[WASM Application] --> B[cloudkit_spi<br/>no_std core]
    
    B --> C{Target}
    
    C -->|wasm32| D[WASM Runtime<br/>Browser/Edge]
    C -->|native| E[Native Runtime<br/>Server/CLI]
    
    D --> F[fetch API]
    E --> G[reqwest/hyper]
    
    F --> H[Cloud Provider<br/>HTTP API]
    G --> H
    
    H --> I[Response]
    I --> B
    B --> A
```

## Dependency Flow (Layer Strictness)

```mermaid
flowchart TD
    APP[Application Code] --> FACADE[cloudkit<br/>Facade Layer]
    
    FACADE --> API[cloudkit_api<br/>API Layer]
    FACADE --> CORE[cloudkit_core<br/>Core Layer]
    
    API --> SPI[cloudkit_spi<br/>SPI Layer]
    
    CORE --> API
    CORE --> SPI
    
    CORE --> PROVIDER1[aws-sdk-*]
    CORE --> PROVIDER2[azure_*]
    CORE --> PROVIDER3[google-cloud-*]
    
    style FACADE fill:#e1f5ff
    style API fill:#b3e5fc
    style CORE fill:#81d4fa
    style SPI fill:#4fc3f7
    
    Note1[Facade depends on<br/>API + Core only]
    Note2[Core depends on<br/>API + SPI + providers]
    Note3[SPI is no_std<br/>foundation]
```

## Cross-Cloud Request Normalization

```mermaid
sequenceDiagram
    participant App
    participant Core
    participant Normalizer
    participant AWS as AWS Provider
    participant Azure as Azure Provider
    
    App->>Core: put_object("bucket", "key", data)
    Core->>Normalizer: normalize_request()
    
    alt AWS Selected
        Normalizer->>AWS: s3::PutObjectRequest
        AWS-->>Normalizer: s3::PutObjectOutput
    else Azure Selected
        Normalizer->>Azure: BlobClient::upload
        Azure-->>Normalizer: BlobProperties
    end
    
    Normalizer->>Normalizer: unify_response()
    Normalizer-->>Core: CloudResult<ObjectMeta>
    Core-->>App: Ok(ObjectMeta)
```

## Key Architectural Principles

### 1. Unidirectional Dependency Flow

```
App → Facade → API → SPI
         ↓      ↓     ↑
       Core ────┘     │
         ↓            │
      Providers ──────┘
```

Upper layers never depend on lower implementation details.

### 2. Provider Isolation

Each provider (AWS, Azure, GCP) is:
- Implemented in Core layer
- Hidden from API consumers
- Swappable via configuration

### 3. No_std Foundation

`cloudkit_spi` is no_std compatible, enabling:
- WASM deployment
- Embedded systems
- Edge computing

### 4. Retry at Core

Retry logic centralized in Core layer ensures:
- Consistent behavior across providers
- Policy enforcement
- Metrics collection

---

**Last Updated**: 2026-01-14
