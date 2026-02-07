# SWE Cloud

Multi-cloud development platform providing emulation, SDK, and infrastructure-as-code in a unified Rust workspace.

## Sub-Projects

| Project | Description | Status |
|---------|-------------|--------|
| [CloudEmu](cloudemu/) | Local cloud emulator (AWS, Azure, GCP, Oracle) | Active |
| [CloudKit](cloudkit/) | Rust multi-cloud SDK | Active |
| [IAC](iac/) | Infrastructure as Code (Terraform) | Active |
| [CloudCost](apps/cloudcost/) | FinOps cost analysis engine | Alpha |
| [SWE Cloud UI](apps/swe-cloud-ui/) | Web dashboard | Active |

## Quick Start

```bash
# Start the cloud emulator
cargo run --bin cloudemu-server

# Run tests
cargo test --workspace
```

## Documentation

See [docs/README.md](docs/README.md) for complete documentation.

## License

MIT OR Apache-2.0
