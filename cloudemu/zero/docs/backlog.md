# ZeroCloud Backlog

Planned features and technical debt for the ZeroCloud project.

## P0: Core Stability & Orchestration
-   [x] Basic Compute SPI (Docker/Hyper-V).
-   [x] Basic Storage SPI (FileSystem).
-   [x] Basic Network SPI (Internal Switches/Bridges).
-   [x] Management CLI (`zero-cli`).
-   [x] HTTP API Facade (`zero-control-facade`).

## P1: Advanced Networking & Storage
-   [ ] **Open vSwitch (OVS)** integration for advanced SDN.
-   [ ] **NVMe Pass-through** support for high-performance storage.
-   [ ] **VLAN Management** for workload isolation across physical nodes.
-   [ ] **DHCP/DNS** service managed by the orchestrator.

## P2: Refinement & Developer UX
-   [x] **WSL 2 Pre-flight Check** for nested virtualization.
-   [ ] **Workload Monitoring**: Real-time stats (CPU/RAM) via API.
-   [ ] **Interactive Dashboard**: Full CRUD for all resources in the web UI.

## P3: Multi-Node Scaling
-   [ ] **Remote Node Agent**: Manage resources on remote Linux/Windows machines.
-   [ ] **Cluster Scheduler**: Simple round-robin placement of workloads across nodes.
