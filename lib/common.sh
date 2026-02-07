#!/usr/bin/env bash
# lib/common.sh — shared helpers for cloud repo bash scripts

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# ── Color support ────────────────────────────────────────────────────
if [ -t 1 ]; then
  GREEN=$'\033[0;32m'  RED=$'\033[0;31m'  YELLOW=$'\033[1;33m'
  BOLD=$'\033[1m'      NC=$'\033[0m'
else
  GREEN=''  RED=''  YELLOW=''  BOLD=''  NC=''
fi

# ── Platform detection ───────────────────────────────────────────────
detect_platform() {
  if grep -qi microsoft /proc/version 2>/dev/null; then
    echo "wsl"
  else
    echo "linux"
  fi
}

# ── Preflight checks ────────────────────────────────────────────────
preflight() {
  if ! command -v cargo &>/dev/null; then
    echo "${RED}ERROR: cargo not found. Install Rust via rustup.${NC}" >&2
    exit 1
  fi
}

# ── Require a command ────────────────────────────────────────────────
require_cmd() {
  if ! command -v "$1" &>/dev/null; then
    echo "${RED}ERROR: $1 not found in PATH.${NC}" >&2
    return 1
  fi
}

# ── Load .env ────────────────────────────────────────────────────────
load_env() {
  if [ -f "$REPO_ROOT/.env" ]; then
    set -a
    source "$REPO_ROOT/.env"
    set +a
  fi
}

# ── Subproject paths ─────────────────────────────────────────────────
UI_DIR="$REPO_ROOT/apps/swe-cloud-ui"
IAC_DIR="$REPO_ROOT/iac"
CLOUDEMU_DIR="$REPO_ROOT/cloudemu"
CLOUDKIT_DIR="$REPO_ROOT/cloudkit"
