# ZeroCloud Backlog

Planned features and technical debt for the ZeroCloud project.

## P0: Core Stability & Orchestration
-   [x] Basic Compute SPI (Docker/Hyper-V).
-   [x] Basic Storage SPI (FileSystem).
-   [x] Basic Network SPI (Internal Switches/Bridges).
-   [x] Management CLI (`zero-cli`).
-   [x] HTTP API Facade (`zero-control-facade`).

## P1: Advanced Services & SDK
-   [x] **ZeroStore** (S3-compatible API).
-   [x] **ZeroDB** (DynamoDB-compatible API).
-   *   [x] **ZeroFunc** (Lambda-compatible API, local execution).
-   *   [x] **ZeroQueue** (SQS-compatible API, visibility timeouts).
-   *   [x] **ZeroID** (IAM-compatible API).
-   *   [x] **ZeroLB** (ALB-compatible API, Reverse Proxy Data Plane).
-   *   [x] **Zero SDK Rust**: Native client library.

## P2: Multi-Cloud Integration
-   [x] **CloudKit Integration**: ZeroCloud as a first-class provider in CloudKit.
-   [x] **CloudKit Integration Tests**: E2E verification for ZeroCloud in CloudKit.
-   [ ] **Open vSwitch (OVS)** integration for advanced SDN.
-   [ ] **NVMe Pass-through** support for high-performance storage.

## P3: Refinement & Scaling
-   [x] **WSL 2 Pre-flight Check** for nested virtualization.
-   [ ] **Workload Monitoring**: Real-time stats (CPU/RAM) via API.
-   [ ] **Interactive Dashboard**: Full CRUD for all resources in the web UI.
-   [ ] **Remote Node Agent**: Manage resources on remote nodes.
-   [ ] **Cluster Scheduler**: Simple round-robin placement.
