#!/usr/bin/env bash
# bin/setup.sh — one-time development environment setup
source "$(cd "$(dirname "$0")/.." && pwd)/lib/common.sh"

PLATFORM=$(detect_platform)
echo "==> Detected platform: $PLATFORM"

# ── Check prerequisites ─────────────────────────────────────────────
echo "==> Checking prerequisites..."

if ! command -v rustup &>/dev/null; then
  echo "${RED}ERROR: rustup not found. Install from https://rustup.rs${NC}" >&2
  exit 1
fi

if ! command -v cargo &>/dev/null; then
  echo "${RED}ERROR: cargo not found. Install Rust via rustup.${NC}" >&2
  exit 1
fi

echo "  rustup: $(rustup --version 2>&1 | head -1)"
echo "  cargo:  $(cargo --version)"

# ── Check optional tools ────────────────────────────────────────────
echo "==> Checking optional tools..."

for tool in terraform aws dx; do
  if command -v "$tool" &>/dev/null; then
    echo "  ${GREEN}✓${NC} $tool"
  else
    echo "  ${YELLOW}⚠${NC} $tool (not found — some scripts may not work)"
  fi
done

# ── Copy .env.example → .env ────────────────────────────────────────
if [ ! -f "$REPO_ROOT/.env" ]; then
  if [ -f "$REPO_ROOT/.env.example" ]; then
    cp "$REPO_ROOT/.env.example" "$REPO_ROOT/.env"
    echo "==> Copied .env.example → .env (edit before running)"
  else
    echo "==> No .env.example found — skipping .env creation"
  fi
else
  echo "==> .env already exists"
fi

# ── Summary ──────────────────────────────────────────────────────────
echo ""
echo "Setup complete!"
echo "  Platform:  $PLATFORM"
echo "  Workspace: $REPO_ROOT/Cargo.toml"
echo ""
echo "Next steps:"
echo "  1. Edit .env with your configuration"
echo "  2. Run: ./cloud build"
echo "  3. Run: ./cloud test"
