# ☁️ ZeroCloud: Private Cloud Orchestrator

**ZeroCloud** is a high-performance, environment-agnostic private cloud orchestrator that abstracts local hardware (VMs, Containers, Networking) into a unified API.

## ✨ Key Features
-   **Native Performance**: Direct integration with Hyper-V (Windows) and KVM (Linux).
-   **Environment Agnostic**: Support for Docker, Podman, and Mock modes.
-   **Stratified Architecture**: Clean separation between Control Plane and Data Drivers.
-   **Unified CLI**: Command-line management tool for all local resources.

## 🚀 Quick Start

1.  **Build the CLI**:
    ```bash
    cargo build -p zero-cli
    ```
2.  **Spin up a workload**:
    ```bash
    zero workload up --id "my-vm" --image "ubuntu-22.04"
    ```

## 📖 Documentation
See [**docs/overview.md**](docs/overview.md) for the complete documentation hub, including Architecture ADRs and Developer Guides.

## 🛠 Installation
For detailed setup instructions, see the [Installation Guide](docs/4-development/guide/installation.md).

## ⚖️ License
MIT
