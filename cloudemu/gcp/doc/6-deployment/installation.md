# GCP Installation Guide

## WHAT
Setup instructions for GCP emulation.

## WHY
Run GCP locally.

## HOW

### Usage
Start the server and point client tools to port 4567.

```bash
cargo run --bin cloudemu-server
export STORAGE_EMULATOR_HOST=http://localhost:4567
```
