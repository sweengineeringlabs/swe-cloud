# ☁️ ZeroCloud: Functional Private Cloud Platform

**ZeroCloud** is a high-performance, environment-agnostic private cloud platform that transforms local hardware into a fully functional cloud region. It goes beyond emulation to provide persistent, native cloud services.

## ✨ Key Features
-   **Full-Stack Cloud**: Native implementation of Compute, Storage, Database, Functions, Queues, and Identity.
-   **Native Performance**: Direct integration with Hyper-V, KVM, and local file systems.
-   **Environment Agnostic**: Support for Docker, Podman, and Mock modes.
-   **Stratified Architecture**: Clean separation between Control Plane and Data Drivers.
-   **Unified CLI**: Command-line management tool for all local resources.

## 📦 Zero Services
-   **ZeroCompute** (EC2-like): VM and Container management.
-   **ZeroStore** (S3-like): Blob and Object storage.
-   **ZeroDB** (DynamoDB-like): NoSQL Document database.
-   **ZeroFunc** (Lambda-like): Serverless Function execution.
-   **ZeroQueue** (SQS-like): Message Queuing service.
-   **ZeroID** (IAM-like): Identity and Access Management.

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
