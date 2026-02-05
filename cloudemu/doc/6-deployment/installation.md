# CloudEmu Installation & Deployment

**Audience**: DevOps Engineers and Developers.

## WHAT: Running CloudEmu Locally

CloudEmu is a Rust binary that can be run directly via `cargo run` or compiled into a standalone executable. It reads configuration from environment variables and stores data in a local directory.

**Scope**:
- Local development setup.
- Environment variable configuration.
- Production deployment considerations.
- Docker containerization.

## WHY: Zero-Configuration Local Testing

### Problems Addressed

1. **AWS Cost During Testing**
   - Impact: Every test run against real AWS incurs charges.
   - Consequence: Developers skip integration tests to save money.

2. **Environment Setup Complexity**
   - Impact: Setting up AWS credentials, regions, and permissions.
   - Consequence: High friction for new developers joining a project.

### Benefits
- **Instant Setup**: Run `cargo run` and you have a local AWS environment.
- **No AWS Account Required**: Develop and test without any AWS credentials.
- **Offline Development**: Work on planes, at coffee shops, or in restricted networks.

## HOW: Installation Steps

### 1. Prerequisites

- **Rust** (1.70+): Install via [rustup.rs](https://rustup.rs/)
- **SQLite** (bundled via `rusqlite`)

### 2. Clone and Build

```bash
git clone git@github.com:sweengineeringlabs/swe-cloud.git
cd cloudemu
cargo build --release
```

### 3. Run the Emulator

```bash
cargo run --release -p cloudemu-server
```

Output:
```
   _____ _                 _ ______                
  / ____| |               | |  ____|               
 | |    | | ___  _   _  __| | |__   _ __ ___  _   _ 
 | |    | |/ _ \| | | |/ _` |  __| | '_ ` _ \| | | |
 | |____| | (_) | |_| | (_| | |____| | | | | | |_| |
  \_____|_|\___/ \__,_|\__,_|______|_| |_| |_|\__,_|
                                                    
  CloudEmu Unified Server v1.0.0

  AWS Service   : http://127.0.0.1:4566
  Azure Service : http://127.0.0.1:10000
  GCP Service   : http://127.0.0.1:4567
  Oracle Service: http://127.0.0.1:4568
```

### 4. Configure Your Tools

```bash
export AWS_ENDPOINT_URL=http://localhost:4566
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test
```

### 5. Test with AWS CLI

```bash
aws s3 mb s3://my-bucket
aws s3 cp hello.txt s3://my-bucket/
aws s3 ls s3://my-bucket/
```

---

## Configuration

| Variable | Default | Description |
| :--- | :--- | :--- |
| `CLOUDEMU_HOST` | `127.0.0.1` | Bind address |
| `CLOUDEMU_AWS_PORT` | `4566` | AWS HTTP port |
| `CLOUDEMU_AZURE_PORT` | `10000` | Azure HTTP port |
| `CLOUDEMU_GCP_PORT` | `4567` | GCP HTTP port |
| `CLOUDEMU_ORACLE_PORT` | `4568` | Oracle HTTP port |
| `CLOUDEMU_DATA_DIR` | `.cloudemu` | Data directory |

---

## Docker Deployment (Future)

CloudEmu will support Docker for containerized deployments:

```bash
docker run -p 4566:4566 -v $(pwd)/.cloudemu:/data cloudemu:latest
```

---

## Summary

CloudEmu installation is straightforward: clone, build, and run. For production use, configure via environment variables and mount persistent volumes for the `.cloudemu` data directory.

**Key Takeaways**:
1. No AWS account needed for local development.
2. Use environment variables for all configuration.
3. Data persists in `.cloudemu/` for stateful testing.

---

**Related Documentation**:
- [Overview](../overview.md)
- [Architecture](../3-design/architecture.md)

**Last Updated**: 2026-01-16  
**Version**: 1.0
