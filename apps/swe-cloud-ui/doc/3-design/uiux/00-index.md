# SWE Cloud UI - Screen Designs

ASCII wireframes and architecture diagrams for the SWE Cloud UI application.

## Screens

| File | Description |
|------|-------------|
| [01-landing-page.ascii](./01-landing-page.ascii) | Main dashboard with stats and feature cards |
| [02-cloudemu-landing.ascii](./02-cloudemu-landing.ascii) | CloudEmu feature landing with provider cards |
| [03-cloudemu-service.ascii](./03-cloudemu-service.ascii) | Service resource view (e.g., S3 buckets) |
| [04-iac.ascii](./04-iac.ascii) | Infrastructure as Code / Terraform modules |
| [05-cloudkit.ascii](./05-cloudkit.ascii) | API Explorer for cloud services |
| [06-context-dropdowns.ascii](./06-context-dropdowns.ascii) | Provider/Environment switchers, User menu |
| [07-404-not-found.ascii](./07-404-not-found.ascii) | Not found error page |
| [08-wiremock-architecture.ascii](./08-wiremock-architecture.ascii) | WireMock/CloudEmu backend architecture |

## Layout Structure

```
┌──────────────────────────────────────┐
│            Context Bar               │  ← Provider + Environment selectors
├──────────────────────────────────────┤
│              Header                  │  ← Brand, Search, Notifications, User
├─────────┬────────────────────────────┤
│         │                            │
│ Sidebar │       Main Content         │  ← Feature-specific content
│         │                            │
│         ├────────────────────────────┤
│         │      Bottom Panel          │  ← Console, Output, Problems tabs
├─────────┴────────────────────────────┤
│            Status Bar                │  ← Environment, stats, version
└──────────────────────────────────────┘
```

## Color Scheme

### Providers
- AWS: `#FF9900` (Orange)
- Azure: `#0078D4` (Blue)
- GCP: `#EA4335` (Red)
- ZeroCloud: `#333333` (Gray)

### Environments
- Local: `#10B981` (Green) - Safe
- Development: `#3B82F6` (Blue) - Safe
- Staging: `#F59E0B` (Yellow) - Confirm required
- Production: `#EF4444` (Red) - Confirm required, read-only default

## Features

1. **CloudEmu** - Cloud service emulation (WireMock-style)
2. **CloudKit** - API explorer and operation executor
3. **IAC** - Infrastructure as Code / Terraform management
