# Glossary

Terminology used throughout the ZeroCloud project.

---

**Compute Driver** - A platform-specific implementation (Docker, Hyper-V, KVM) for managing virtual workloads.

**Control Core** - The orchestration layer that receives API requests and coordinates work across drivers.

**Data Plane** - The layer responsible for interacting with physical resources (engine, storage, network).

**Node** - A physical host machine part of the ZeroCloud cluster.

**SPI (Service Provider Interface)** - A set of traits that define how drivers must behave (e.g., `ComputeDriver`).

**Workload** - A generic term for a Virtual Machine or Container managed by ZeroCloud.

**ZeroEngine** - The main entry point in the data plane that aggregates compute, storage, and networking capabilities.

**ZeroStore** - Local high-performance object storage service.

**ZeroDB** - Distributed-ready document and key-value database service.

**ZeroFunc** - Event-driven serverless function execution service.

**ZeroQueue** - Asynchronous message queuing service with visibility timeouts.

**ZeroID** - Identity and Access Management (IAM) service for users and groups.

**ZeroLB** - Layer 7 and Layer 4 load balancing service with reverse proxy capabilities.

**ZeroNet** - Software-defined networking layer for workload isolation.

**Zero SDK** - The native Rust client library for interacting with the ZeroCloud Control Plane.
