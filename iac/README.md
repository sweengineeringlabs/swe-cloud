# Multi-Cloud IAC Framework (SEA)

**Unified Cloud Infrastructure** - Deploy serverless, compute, and data resources to AWS, Azure, and GCP using a single, provider-agnostic interface.

## Features

- ✅ **Provider Abstraction**: Single interface for 8+ resource types across **AWS, Azure, GCP, and ZeroCloud**.
- ✅ **ZeroCloud Support**: Full local emulation support via CloudEmu (wire-compatible AWS Shim).
- ✅ **SEA Architecture**: 5-layer design (Common, SPI, API, Core, Facade) mirrored from CloudKit SDK.
- ✅ **Unified Identity**: Capability-based role assignment (`storage_read`, `admin`) that works across all clouds.
- ✅ **Standardized Sizing**: Unified `small`, `medium`, `large` mappings across all providers.
- ✅ **Quality Driven**: Integrated static validation and Go-based Terrates suite.

## Quick Start

```hcl
module "storage" {
  source      = "./facade/storage"
  provider_name = "aws" # Switch to "azure" or "gcp" with zero code changes
  bucket_name = "my-secure-data-123"
  environment = "prod"
}
```

## Documentation

Full documentation is available in the **[Documentation Hub](./doc/overview.md)**.

- **[Installation Guide](./doc/6-deployment/installation-guide.md)**
- **[Architecture Specification](./doc/3-design/architecture.md)**
- **[Testing Strategy](./doc/5-testing/testing-strategy.md)**

## Testing

Run the full platform validation suite:

```bash
go test -v ./...
```

## License

MIT - See [LICENSE](LICENSE) for details.
