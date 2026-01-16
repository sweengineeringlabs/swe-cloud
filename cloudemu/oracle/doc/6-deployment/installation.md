# Oracle Installation Guide

## WHAT
Setup instructions for Oracle emulation.

## WHY
Run OCI locally.

## HOW

### Usage
Start the server and point client tools to port 4568.

```bash
cargo run --bin cloudemu-server
export CLOUDEMU_ORACLE_PORT=4568
```
