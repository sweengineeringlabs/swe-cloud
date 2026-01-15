# CloudEmu Server

The **CloudEmu Server** (`cloudemu_server`) is the unified runtime entry point for the CloudEmu platform. It runs a local multi-cloud emulator that simulates AWS, Azure, and GCP services simultaneously.

## Features

*   **Multi-Cloud Support**: Runs AWS (`:4566`), Azure (`:4567`), and GCP (`:4568`) emulators simultaneously.
*   **Unified Runtime**: Single binary powered by `Axum` and `Tokio`.
*   **Facade Architecture**: Routes requests to provider-specific control planes.
*   **Persistence**: Uses the shared `StorageEngine` to persist data to `.cloudemu/` (configurable).

## Usage

Run the server using Cargo:

```bash
cargo run -p cloudemu_server
```

## Configuration

The server can be configured via command-line arguments or environment variables.

| Environment Variable | CLI Argument | Default | Description |
|---------------------|--------------|---------|-------------|
| `CLOUDEMU_AWS_PORT` | `--aws-port` | `4566` | Port for AWS services |
| `CLOUDEMU_AZURE_PORT` | `--azure-port` | `4567` | Port for Azure services |
| `CLOUDEMU_GCP_PORT` | `--gcp-port` | `4568` | Port for GCP services |
| `CLOUDEMU_DATA_DIR` | `--data-dir` | `.cloudemu` | Directory for persistent data |
| `CLOUDEMU_ENABLE_AWS` | `--enable-aws` | `true` | Enable AWS provider |
| `CLOUDEMU_ENABLE_AZURE` | `--enable-azure` | `true` | Enable Azure provider |
| `CLOUDEMU_ENABLE_GCP` | `--enable-gcp` | `true` | Enable GCP provider |

## Architecture

The server initializes three asynchronous tasks, one for each provider. It uses the `control-facade` crates (`aws-control-facade`, `azure-control-facade`, `gcp-control-facade`) to handle protocol-specific request parsing and routing.

For development details, see [Toolchain Documentation](doc/3-design/toolchain.md).
