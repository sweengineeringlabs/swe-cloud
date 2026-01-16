# AWS Installation Guide

## WHAT
Instructions for setting up the AWS emulator.

## WHY
Developers need a local AWS environment.

## HOW

### Prerequisites
- Rust 1.70+

### Installation
The AWS module is bundled with CloudEmu Server.

```bash
cargo build --release -p cloudemu-server
```

### Configuration
- **Port**: Defaults to 4566.
- **Data Dir**: `.cloudemu/aws`.

### Usage
```bash
cargo run --bin cloudemu-server
export AWS_ENDPOINT_URL=http://localhost:4566
aws s3 ls
```
