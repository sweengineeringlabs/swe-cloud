# Terraform Flow Diagrams

Visual representations of request flow through the IAC framework's SEA layers.

## Terraform Plan/Apply Flow

```mermaid
sequenceDiagram
    participant User
    participant TF as Terraform CLI
    participant Facade as Facade Module<br/>(storage, compute)
    participant Core as Core Module
    participant Provider as Provider Module<br/>(aws, azure, gcp)
    participant Cloud as Cloud API

    User->>TF: terraform plan
    TF->>Facade: Load facade module
    Facade->>Core: Reference core resources
    Core->>Provider: Select provider
    Provider->>Cloud: Query existing state
    Cloud-->>Provider: Current resources
    Provider-->>Core: State data
    Core-->>Facade: Computed values
    Facade-->>TF: Plan output
    TF-->>User: Show changes
    
    User->>TF: terraform apply
    TF->>Facade: Apply configuration
    Facade->>Core: Create resources
    Core->>Provider: Provider-specific calls
    Provider->>Cloud: Create/Update/Delete
    Cloud-->>Provider: Success
    Provider-->>Core: Resource IDs
    Core-->>Facade: Outputs
    Facade-->>TF: State update
    TF-->>User: Apply complete
```

## Multi-Cloud Provider Selection

```mermaid
flowchart TD
    A[Terraform Config] --> B[Facade Module<br/>variable: provider]
    
    B --> C{provider value}
    
    C -->|aws| D[aws/ module]
    C -->|azure| E[azure/ module]
    C -->|gcp| F[gcp/ module]
    
    D --> G[AWS Resources:<br/>S3, EC2, RDS]
    E --> H[Azure Resources:<br/>Blob, VM, SQL]
    F --> I[GCP Resources:<br/>GCS, Compute, SQL]
    
    G --> J[Unified Outputs:<br/>endpoint, id, arn]
    H --> J
    I --> J
    
    J --> K[Application Code<br/>Provider-Agnostic]
```

## SEA Layer Dependency Flow

```mermaid
flowchart LR
    FACADE[Facade Layer<br/>facade/*] --> CORE[Core Layer<br/>core/*]
    
    CORE --> API[API Layer<br/>api/*]
    
    API --> COMMON[Common Layer<br/>common/*]
    
    COMMON --> SPI[SPI Layer<br/>spi/*]
    
    SPI --> AWS[aws/ provider]
    SPI --> AZURE[azure/ provider]
    SPI --> GCP[gcp/ provider]
    
    style FACADE fill:#e1f5ff
    style CORE fill:#b3e5fc
    style API fill:#81d4fa
    style COMMON fill:#4fc3f7
    style SPI fill:#29b6f6
```

## Variable and Output Flow

```mermaid
sequenceDiagram
    participant User as User Config
    participant Facade
    participant Core
    participant Provider
    
    User->>Facade: var.provider = "aws"<br/>var.bucket_name = "my-bucket"
    Facade->>Core: Pass normalized vars
    Core->>Core: Validate + transform
    Core->>Provider: Provider-specific vars
    Provider->>Provider: Create resources
    Provider-->>Core: Resource outputs
    Core->>Core: Normalize outputs
    Core-->>Facade: Unified outputs
    Facade-->>User: output.endpoint<br/>output.id
```

## Storage Facade Example

```mermaid
flowchart TD
    A[User: terraform apply] --> B[facade/storage/main.tf]
    
    B --> C{var.provider}
    
    C -->|aws| D[module.aws_storage]
    C -->|azure| E[module.azure_storage]
    C -->|gcp| F[module.gcp_storage]
    
    D --> G[core/storage]
    E --> G
    F --> G
    
    G --> H[api/storage]
    
    H --> I{Call Provider}
    
    I -->|aws| J[aws_s3_bucket]
    I -->|azure| K[azurerm_storage_account]
    I -->|gcp| L[google_storage_bucket]
    
    J --> M[Outputs]
    K --> M
    L --> M
    
    M --> N[endpoint = ...]
    M --> O[bucket_id = ...]
    
    N --> P[Application]
    O --> P
```

## Testing Flow with Terratest

```mermaid
sequenceDiagram
    participant Test as Go Test<br/>(Terratest)
    participant TF as Terraform
    participant Module as IAC Module
    participant Cloud as Cloud Provider
    
    Test->>TF: terraform init
    TF-->>Test: Initialized
    
    Test->>TF: terraform apply -auto-approve
    TF->>Module: Deploy resources
    Module->>Cloud: Create infrastructure
    Cloud-->>Module: Resources created
    Module-->>TF: Outputs
    TF-->>Test: Apply complete + outputs
    
    Test->>Test: Validate outputs
    Test->>Cloud: Query resources directly
    Cloud-->>Test: Resource details
    Test->>Test: Assert expectations
    
    Test->>TF: terraform destroy -auto-approve
    TF->>Module: Destroy resources
    Module->>Cloud: Delete infrastructure
    Cloud-->>Module: Deleted
    Module-->>TF: Destroy complete
    TF-->>Test: Success
```

## State Management Flow

```mermaid
flowchart TD
    A[terraform apply] --> B{Backend Type}
    
    B -->|local| C[Local File<br/>terraform.tfstate]
    B -->|s3| D[AWS S3 Bucket<br/>+ DynamoDB lock]
    B -->|azurerm| E[Azure Blob Storage]
    
    C --> F[Read Current State]
    D --> F
    E --> F
    
    F --> G[Compute Diff]
    G --> H[Apply Changes]
    
    H --> I[Update State]
    
    I --> J{Write State}
    
    J -->|local| K[terraform.tfstate]
    J -->|s3| L[S3 Bucket]
    J -->|azurerm| M[Blob Storage]
```

## Size Normalization Pattern

```mermaid
flowchart LR
    A[var.size = small] --> B[common/sizing.tf]
    
    B --> C{Provider}
    
    C -->|AWS| D[t3.small<br/>db.t3.small]
    C -->|Azure| E[Standard_B1s<br/>Basic]
    C -->|GCP| F[e2-small<br/>db-f1-micro]
    
    D --> G[Unified Output:<br/>size_class = small]
    E --> G
    F --> G
```

## Dependency Graph Example

```mermaid
flowchart TD
    VPC[VPC/VNet] --> SUBNET[Subnet]
    SUBNET --> SEC[Security Group]
    SEC --> COMPUTE[Compute Instance]
    SEC --> DB[Database]
    
    COMPUTE --> LB[Load Balancer]
    
    IAM[IAM Role] --> COMPUTE
    IAM --> DB
    
    STORE[Storage Bucket] --> COMPUTE
    
    style VPC fill:#ffebee
    style SUBNET fill:#ffcdd2
    style SEC fill:#ef9a9a
    style COMPUTE fill:#e57373
    style DB fill:#ef5350
    style LB fill:#f44336
    style IAM fill:#e53935
    style STORE fill:#d32f2f
```

## CI/CD Integration

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant Git as GitHub
    participant CI as GitHub Actions
    participant TF as Terraform Cloud
    participant Cloud
    
    Dev->>Git: git push
    Git->>CI: Trigger workflow
    CI->>CI: terraform fmt -check
    CI->>CI: terraform validate
    CI->>CI: go test (Terratest)
    
    alt PR
        CI->>TF: terraform plan
        TF-->>CI: Plan output
        CI->>Git: Comment plan on PR
    else Main Branch
        CI->>TF: terraform apply
        TF->>Cloud: Deploy infrastructure
        Cloud-->>TF: Success
        TF-->>CI: Apply complete
        CI-->>Git: Update status
    end
```

## Key Architectural Patterns

### 1. Provider Abstraction

Each facade module accepts a `provider` variable:
- **Input**: Unified parameters (size, environment, region)
- **Output**: Normalized attributes (endpoint, id, arn)
- **Implementation**: Provider-specific resources hidden

### 2. Module Composition

```
facade/storage
  ├── main.tf (entry)
  ├── core/
  │   └── Normalized logic
  └── providers/
      ├── aws/
      ├── azure/
      └── gcp/
```

### 3. State Isolation

- **Dev**: Local state
- **Staging**: S3 backend
- **Prod**: S3 backend + DynamoDB locking

### 4. Testing Pyramid

```
        E2E (Terraform Apply)
          ↑
       Integration (Go Tests)
          ↑
     Unit (Validation)
```

---

**Last Updated**: 2026-01-14
