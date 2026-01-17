# 🚀 ZeroCloud CLI User Manual

`zero-cli` is the command-line interface for **ZeroCloud**, a private cloud orchestrator designed for high-performance lab environments. It allows you to manage compute workloads, storage volumes, and virtual networks directly on your local hardware.

---

## 🛠 Installation

To use the CLI from the workspace root:

```powershell
cargo build -p zero-cli
# The binary will be available at:
./target/debug/zero-cli.exe
```

---

## 🌍 Environment Detection (Smart Drivers)

ZeroCloud is environment-agnostic. By default, it follows this priority:
1.  **Docker**: If a local Docker daemon is detected, it uses container-based orchestration.
2.  **Native Hypervisor**: If Docker is missing, it falls back to **Hyper-V** (Windows) or **KVM** (Linux).
3.  **Mock**: If no virtualization is available, it uses an in-memory emulator for development and CI.

### Forcing Native Drivers
Use the global `--native` (or `-n`) flag to ignore Docker and force the use of the OS-native hypervisor:

```powershell
zero --native workload up --id "my-vm" --image "ubuntu-22.04"
```

---

## 📋 Command Reference

### 1. Workload Management (`workload`)
Manage virtual machines or containers.

*   **Spin up a workload**:
    `zero workload up --id <ID> --image <IMAGE>`
    *Example:* `zero workload up -i "web-srv" --image "nginx:latest"`
*   **Terminate a workload**:
    `zero workload down --id <ID>`

### 2. Networking (`network`)
Manage isolated virtual subnets.

*   **Create a network**:
    `zero network create --id <ID> --cidr <CIDR>`
    *Example:* `zero network create -i "lab-net" --cidr "172.16.0.0/24"`
    *Note: On Windows, this creates a Hyper-V Internal Virtual Switch.*

### 3. Storage (`volume`)
Manage local block/file storage.

*   **Provision a volume**:
    `zero volume create --id <ID> --size <GB>`
    *Example:* `zero volume create -i "db-data" --size 100`

### 4. Node Management (`node`)
View the health of your local compute resources.

*   **List nodes**:
    `zero node list`

---

## 🔑 Permissions & Troubleshooting

### Windows (Hyper-V)
*   **Administrator Rights**: Real hardware orchestration (`--native`) requires running your terminal as **Administrator**.
*   **Module Not Found**: Ensure the Hyper-V feature is enabled:
    `Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All`

### Docker
*   If running via the Docker driver, ensure **Docker Desktop** is open and the service is active.

---

## 🎨 Global Options

| Flag | Short | Description |
| :--- | :--- | :--- |
| `--native` | `-n` | Force use of native OS drivers (Hyper-V / KVM) |
| `--help` | `-h` | Show help information |
