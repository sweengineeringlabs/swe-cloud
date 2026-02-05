# ADR 001: ZeroCloud Private Cloud Architecture

## Status
Proposed

## Context
We need a private cloud provider implementation (ZeroCloud) that allows for local resource management (SWE Engineering Labs) while maintaining compatibility with the multi-cloud architecture used by AWS, Azure, and GCP emulators.

## Decision
We will implement ZeroCloud using the SEA (Service-Engine-Adapter) pattern with a plugin-based driver architecture to allow for flexible backend choices.

### 1. Compute
- **Containers**: Default to **Docker** (using `bollard`) for lightweight isolation. Support for **Podman** as a drop-in alternative.
- **Virtual Machines**: 
  - **Windows Hosts**: Use native **Hyper-V** (orchestrated via PowerShell commands) for full hardware virtualization.
  - **Linux Hosts**: Use **KVM/Libvirt** for hardware virtualization.

### 2. Storage
- **Local Resources**: Support direct mounting of **Local NVMe** drives for high-performance block storage.
- **Shared Resources**: Support **NAS (Samba/NFS)** backends to emulate S3-like distributed storage for the private lab.
- **Interface**: Exposed via the `StorageDriver` SPI for block and file operations.

### 3. Networking
- **Virtualization**: Use **Linux Bridge** for basic internal networking.
- **Advanced Networking**: Implement **Open vSwitch (OVS)** to manage local VLANs and provide advanced SDN (Software Defined Networking) capabilities within the ZeroCloud environment.

## Consequences
- **Extensibility**: The driver-based approach allows adding new hardware or virtualization backends without changing the Control Plane logic.
- **Consistency**: ZeroCloud will be manageable via the same IAC tools used for public clouds.
- **Performance**: Direct hardware mapping (NVMe) provides near-native performance for local workloads.
