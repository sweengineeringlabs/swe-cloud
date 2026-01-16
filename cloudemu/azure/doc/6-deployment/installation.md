# Azure Installation Guide

## WHAT
Setup instructions for Azure emulation.

## WHY
Run Azure locally.

## HOW

### Usage
Start the server and point client tools to port 10000.

```bash
cargo run --bin cloudemu-server
export AZURE_STORAGE_CONNECTION_STRING="..."
```
