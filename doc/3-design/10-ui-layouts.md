# 10 - UI Layouts and ASCII Diagrams

## Document Information

| Field | Value |
|-------|-------|
| **Version** | 1.0.0 |
| **Status** | Design Complete |
| **Last Updated** | 2025-12-26 |

---

## 1. System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Application Code                             │
│   storage.put_object("bucket", "key", data).await?              │
├─────────────────────────────────────────────────────────────────┤
│                         CloudKit                                 │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │  ObjectStorage │ KeyValueStore │ MessageQueue │ PubSub  │   │
│   └─────────────────────────────────────────────────────────┘   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐    │   │
│   │  │   AWS   │  │  Azure  │  │   GCP   │  │ Oracle  │    │   │
│   │  └─────────┘  └─────────┘  └─────────┘  └─────────┘    │   │
│   └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. SEA Layer Stack

```
┌─────────────────────────────────────────────────────────────────┐
│ LAYER 5: FACADE    │ CloudKit entry point, prelude              │
├────────────────────┼────────────────────────────────────────────┤
│ LAYER 4: CORE      │ CloudContext, OperationExecutor            │
├────────────────────┼────────────────────────────────────────────┤
│ LAYER 3: API       │ ObjectStorage, KeyValueStore, etc.         │
├────────────────────┼────────────────────────────────────────────┤
│ LAYER 2: SPI       │ AuthProvider, RetryPolicy, Metrics         │
├────────────────────┼────────────────────────────────────────────┤
│ LAYER 1: COMMON    │ CloudError, Region, Credentials            │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. SDK vs CLI Comparison

```
SDK (Used):                          CLI (Not Used):
┌─────────────┐                      ┌─────────────┐
│ Application │                      │ Application │
└──────┬──────┘                      └──────┬──────┘
       │ Function call                       │ Spawn process
       ▼                                     ▼
┌─────────────┐                      ┌─────────────┐
│  CloudKit   │                      │ aws/az/gcloud│
└──────┬──────┘                      └──────┬──────┘
       │ Method call                         │ Parse stdout
       ▼                                     ▼
┌─────────────┐                      ┌─────────────┐
│ Provider SDK│                      │ Cloud API   │
└──────┬──────┘                      └─────────────┘
       │ HTTPS
       ▼
┌─────────────┐
│  Cloud API  │
└─────────────┘

✅ Fast, type-safe, async           ❌ Slow, text parsing
```

---

## 4. Request Flow

```
Application
    │
    ▼ storage.put_object("bucket", "key", data)
┌─────────────────────────────────────────────────┐
│                CloudKit                          │
│  1. Validate │ 2. Auth │ 3. Create context      │
└────────────────────────┬────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────┐
│            OperationExecutor                     │
│  4. Metrics │ 5. Execute │ 6. Retry │ 7. Record │
└────────────────────────┬────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────┐
│           Provider SDK (AWS/Azure/GCP)          │
│  8. Serialize │ 9. Sign │ 10. HTTPS │ 11. Parse │
└────────────────────────┬────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────┐
│              Cloud Provider API                  │
└─────────────────────────────────────────────────┘
```

---

## 5. Error Flow

```
Provider Error (AWS NoSuchKey, Azure 404, etc.)
         │
         ▼
┌────────────────────────────────────────┐
│       Conversion (impl From<...>)       │
│                                         │
│  AWS NoSuchKey  ──► NotFound           │
│  Azure 404      ──► NotFound           │
│  AccessDenied   ──► Auth::Permission   │
└────────────────────────┬───────────────┘
                         │
                         ▼
┌────────────────────────────────────────┐
│             CloudError                  │
│                                         │
│  Unified error type for application    │
└────────────────────────────────────────┘
```

---

## 6. Retry Flow

```
Request ──► Execute ──► Success? ──► Yes ──► Return
                │
                ▼ No
            Retryable? ──► No ──► Return Error
                │
                ▼ Yes
            Max Attempts? ──► Yes ──► Return Error
                │
                ▼ No
            Sleep(backoff) ──► Execute (retry)
```

---

## 7. Credential Chain

```
┌───────────────────────────────────────────┐
│  1. Environment Variables                  │
│     AWS_ACCESS_KEY_ID, AZURE_STORAGE_KEY  │
└─────────────────────┬─────────────────────┘
                      │ if not found
                      ▼
┌───────────────────────────────────────────┐
│  2. Config Files                           │
│     ~/.aws/credentials, ~/.oci/config     │
└─────────────────────┬─────────────────────┘
                      │ if not found
                      ▼
┌───────────────────────────────────────────┐
│  3. Instance Metadata (IAM)                │
│     EC2, ECS, Lambda, GCE, Azure VM       │
└─────────────────────┬─────────────────────┘
                      │ if not found
                      ▼
┌───────────────────────────────────────────┐
│  4. Custom AuthProvider (SPI)              │
│     Vault, SSO, Custom                     │
└───────────────────────────────────────────┘
```

---

## 8. Message Queue Lifecycle

```
Producer                  Queue                Consumer
    │                       │                      │
    │ send("msg")          │                      │
    ├──────────────────────►│                      │
    │                       │◄── stored            │
    │                       │                      │
    │                       │      receive()       │
    │                       │◄─────────────────────┤
    │                       │                      │
    │          message ─────┼─────────────────────►│
    │        (invisible)    │                      │
    │                       │      (processing)    │
    │                       │                      │
    │                       │      delete()        │
    │                       │◄─────────────────────┤
    │          deleted ─────┼                      │
```

---

## 9. Provider Comparison

```
╔═══════════════╦═══════════╦═══════════╦═══════════╦═══════════╗
║   Service     ║    AWS    ║   Azure   ║    GCP    ║  Oracle   ║
╠═══════════════╬═══════════╬═══════════╬═══════════╬═══════════╣
║ Storage       ║    S3     ║   Blob    ║    GCS    ║  Object   ║
║ KV Store      ║ DynamoDB  ║  Cosmos   ║ Firestore ║   NoSQL   ║
║ Queue         ║   SQS     ║ Svc. Bus  ║   Tasks   ║  Stream   ║
║ Pub/Sub       ║   SNS     ║ Evt. Grid ║  Pub/Sub  ║    -      ║
║ Functions     ║  Lambda   ║ Functions ║  Cloud Fn ║ Functions ║
╚═══════════════╩═══════════╩═══════════╩═══════════╩═══════════╝
```

---

## 10. Project Structure

```
cloudkit/
├── Cargo.toml               # Workspace
├── README.md
├── CHANGELOG.md
├── CONTRIBUTING.md
│
├── crates/
│   ├── cloudkit/            # Core library
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── prelude.rs
│   │       ├── common/      # Layer 1
│   │       ├── spi/         # Layer 2
│   │       ├── api/         # Layer 3
│   │       ├── core/        # Layer 4
│   │       └── facade/      # Layer 5
│   │
│   ├── cloudkit-aws/        # AWS provider
│   ├── cloudkit-azure/      # Azure provider
│   ├── cloudkit-gcp/        # GCP provider
│   └── cloudkit-oracle/     # Oracle provider
│
├── doc/
│   └── 3-design/            # This documentation
│
├── docs/                    # User documentation
│   ├── README.md
│   ├── getting-started.md
│   ├── providers/
│   └── spi/
│
└── examples/
    ├── basic_usage.rs
    ├── multi_cloud.rs
    └── testing.rs
```

---

## 11. WASM Architecture (Future)

```
Current (Native):                 Future (Dual Platform):

┌─────────────┐                   ┌─────────────────────┐
│  CloudKit   │                   │   cloudkit (core)   │
│             │                   │  (platform-agnostic)│
│  tokio      │                   └──────────┬──────────┘
│  reqwest    │                              │
└─────────────┘                   ┌──────────┼──────────┐
                                  ▼          ▼          ▼
                            ┌─────────┐ ┌────────┐ ┌────────┐
                            │ Native  │ │  WASM  │ │  WASI  │
                            │ tokio   │ │ gloo   │ │        │
                            │ reqwest │ │ fetch  │ │        │
                            └─────────┘ └────────┘ └────────┘
```
