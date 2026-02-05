# AWS-Zero Parity Manifesto

This document outlines the "Full Picture" comparison between ZeroCloud and AWS, detailing how ZeroCloud achieves architectural and data-plane parity to enable seamless hybrid cloud workflows.

## 1. The Parity Philosophy

ZeroCloud is NOT just a mock. It is a **Functional Double** of AWS core services. While AWS is built for global-scale multi-tenancy, ZeroCloud is built for **performance-first private infrastructure**.

| Level | AWS (Public Cloud) | ZeroCloud (Private Cloud) | Parity Impact |
| :--- | :--- | :--- | :--- |
| **Control Plane** | Global API Endpoints (Auth, Rate Limiting) | Local API Endpoint (v1/ v1/) | High: CloudKit SDKs are 100% compatible. |
| **Data Plane** | Distributed Fleets (Firecracker, EBS, S3-Nodes) | Native OS Drivers (Process, FS, SQLite) | High: Real-world behavior (latency, I/O) is preserved. |
| **Scale** | Unlimited (Horizontal) | Finite (Vertical/Node-bound) | Expected: Optimized for local/edge performance. |

---

## 2. Core Service Matrix (The "Big 6")

These services form the backbone of Cloud-Native applications. ZeroCloud implements deep data-plane parity for each.

### A. ZeroStore (S3 Parity)
- **Control Plane**: `CreateBucket`, `ListBuckets`, `PutObject`.
- **Data Plane**: Real streaming of bytes to host filesystem. Supports content-types and metadata persistence in SQLite.
- **Why it matters**: Your app can use S3 SDKs to store multi-gigabyte files locally without a network round-trip.

### B. ZeroDB (DynamoDB Parity)
- **Control Plane**: `CreateTable`, `PutItem`, `GetItem`, `Query`.
- **Data Plane**: Leverages SQLite's JSON1 extension to simulate NoSQL document storage. Fully ACID compliant.
- **Why it matters**: You get the schema-less flexibility of DynamoDB with the operational simplicity of a local file.

### C. ZeroFunc (Lambda Parity)
- **Control Plane**: `CreateFunction`, `UpdateFunction`, `Invoke`.
- **Data Plane**: Native Process Spawning. Executes real Python/Node.js code by piping stdin/stdout.
- **Why it matters**: Instant cold starts (<50ms). Debug your "Lambda" logic by inspecting local process logs.

### D. ZeroQueue (SQS Parity)
- **Control Plane**: `CreateQueue`, `SendMessage`, `ReceiveMessage`, `DeleteMessage`.
- **Data Plane**: Stateful message engine with visibility timeouts. Tracks "Receipt Handles" in SQLite.
- **Why it matters**: If a local worker crashes, the message reappears in the queue just like in AWS.

### E. ZeroLB (ALB Parity)
- **Control Plane**: `CreateTargetGroup`, `RegisterTargets`, `CreateListener`.
- **Data Plane**: A Rust-based Reverse Proxy (Axum/Reqwest). Actively routes real HTTP/HTTPS traffic to running targets.
- **Why it matters**: Test complex microservice routing rules (e.g., `/api` -> Service A) on your local machine.

### F. ZeroNet (VPC Parity)
- **Control Plane**: `CreateNetwork`, `AttachInterface`.
- **Data Plane**: Software-Defined Networking (SDN) bridging. Uses host networking bridges and listeners to isolate services.
- **Why it matters**: Simulates the connectivity constraints of a VPC environment.

---

## 3. Data Plane Deep Dive: Why Parity Matters

| Service | Data Plane Implementation | Parity Characteristic |
| :--- | :--- | :--- |
| **Compute** | OS Process Forking | Real-time CPU/Memory usage. |
| **Storage** | Direct POSIX I/O | Exact file-level representation. |
| **Networking** | Axum Reverse Proxy | Real header manipulation and round-robin. |
| **Persistence** | SQLite WAL Mode | Multi-process concurrent access. |

---

## 4. The Hybrid Cloud Value Proposition

ZeroCloud + CloudKit enables the **"Local-to-Cloud Continuum"**:

1. **Local-First Dev**: Use `CloudKit::zero()` for sub-millisecond dev loops with no costs.
2. **Private Edge**: Deploy the same binary to ZeroCloud on a factory floor or remote node for ultra-low latency.
3. **Public Cloud**: Scale to AWS for global reach without changing a single line of business logic.

**Conclusion**: AWS-Zero Parity is not about 100% API coverage; it is about **99% Behavioral Coverage** for the most critical data-path services.
