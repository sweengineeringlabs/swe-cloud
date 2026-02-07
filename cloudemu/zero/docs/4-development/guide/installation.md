# 🛠️ ZeroCloud CLI Installation Guide

Follow these steps to build and install the ZeroCloud CLI (`zero`) on your local machine.

## 📋 Prerequisites

Before you begin, ensure you have the following installed:
- **Rust Toolchain**: [Install Rust](https://rustup.rs/) (Stable version 1.75+ recommended).
- **Git**: To clone the repository.
- **System Dependencies**:
  - **Windows**: Hyper-V and PowerShell (for native mode).
  - **Linux**: libvirt and KVM (for native mode).
  - **Docker**: Optional, for container-based orchestration.

## 🔧 Building from Source

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/sweengineeringlabs/swe-cloud.git
    cd swe-cloud
    ```

2.  **Build the CLI binary**:
    ```bash
    cargo build -p zero-cli --release
    ```

3.  **Locate the Binary**:
    The compiled binary will be located at:
    `./target/release/zero-cli.exe` (Windows) or `./target/release/zero-cli` (Linux).

## 🚀 Installation (Optional)

To use `zero` from anywhere, add the binary to your system PATH or move it to a standard bin directory.

### Windows (PowerShell)
```powershell
# Move to a bin folder (example)
mkdir C:\tools\bin -Force
cp ./target/release/zero-cli.exe C:\tools\bin\zero.exe

# Add to PATH (Permanent)
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\tools\bin", [EnvironmentVariableTarget]::User)
```

### Linux / macOS
```bash
# Move to /usr/local/bin
sudo cp ./target/release/zero-cli /usr/local/bin/zero
sudo chmod +x /usr/local/bin/zero
```

## ✅ Verifying Installation

Run the following command to verify that the CLI is installed and working correctly:

```bash
zero --help
```

## 🧪 Quick Test (Mock Mode)

To test the CLI without setting up any hardware or Docker:

```bash
# Run the node list command using the mock engine
zero node list
```

---

*For detailed usage and troubleshooting, see the [User Manual](./docs/user-manual.md).*
