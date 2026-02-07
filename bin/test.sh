#!/usr/bin/env bash
# bin/test.sh — run tests for the cloud workspace
source "$(cd "$(dirname "$0")/.." && pwd)/lib/common.sh"

SUITE="${1:-all}"

if [ "$SUITE" = "scripts" ]; then
  exec bash "$REPO_ROOT/bin/tests/runner.sh"
fi

preflight

run_cloudemu_tests() {
  echo "==> Testing cloudemu..."
  cargo test --manifest-path "$REPO_ROOT/Cargo.toml" \
    -p aws-control-spi -p aws-control-api -p aws-control-core -p aws-control-facade \
    -p aws-data-spi -p aws-data-api -p aws-data-core -p aws-data-facade \
    -p azure-control-spi -p azure-control-api -p azure-control-core -p azure-control-facade \
    -p azure-data-spi -p azure-data-api -p azure-data-core -p azure-data-facade \
    -p gcp-control-spi -p gcp-control-api -p gcp-control-core -p gcp-control-facade \
    -p gcp-data-spi -p gcp-data-api -p gcp-data-core -p gcp-data-facade \
    -p oracle-control-spi -p oracle-control-core -p oracle-data-core
}

run_cloudkit_tests() {
  echo "==> Testing cloudkit..."
  cargo test --manifest-path "$REPO_ROOT/Cargo.toml" \
    -p cloudkit_spi -p cloudkit_api -p cloudkit_core -p cloudkit
}

run_crate_tests() {
  echo "==> Testing shared crates..."
  cargo test --manifest-path "$REPO_ROOT/Cargo.toml" \
    -p rsc-lint -p rsc-security
}

case "$SUITE" in
  cloudemu) run_cloudemu_tests ;;
  cloudkit) run_cloudkit_tests ;;
  crates)   run_crate_tests ;;
  all)
    run_cloudemu_tests
    run_cloudkit_tests
    run_crate_tests
    ;;
  *)
    echo "Usage: ./cloud test [cloudemu|cloudkit|crates|scripts|all]"
    exit 1
    ;;
esac

echo "==> Tests complete ($SUITE)"
