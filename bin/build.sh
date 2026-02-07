#!/usr/bin/env bash
# bin/build.sh — build the cloud workspace
source "$(cd "$(dirname "$0")/.." && pwd)/lib/common.sh"

BUILD_MODE="--release"
for arg in "$@"; do
  case "$arg" in
    --debug) BUILD_MODE="" ;;
  esac
done

PROFILE_LABEL="${BUILD_MODE:+release}"
PROFILE_LABEL="${PROFILE_LABEL:-debug}"

preflight

echo "==> Building workspace ($PROFILE_LABEL)..."
cargo build --manifest-path "$REPO_ROOT/Cargo.toml" $BUILD_MODE

echo "==> Build complete ($PROFILE_LABEL)"
