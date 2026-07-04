#!/usr/bin/env bash
# Optional local preview release gate (IP check, workspace build, full E2E).
# PR CI already runs the full Playwright suite on every pull request; this script
# is a maintainer convenience before tagging. TagPush only deploys GitHub Pages.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

echo "== IP release gate =="
"$ROOT/scripts/maintainer/ip_release_gate.sh"

echo ""
echo "== Workspace build (deny rustc warnings) =="
RUSTFLAGS='-D warnings' cargo build --workspace
echo "PASS: cargo build --workspace without warnings"

echo ""
echo "== Playwright prerequisites =="
if [[ ! -d end2end/node_modules ]]; then
  (cd end2end && npm ci)
fi
if ! command -v chromium >/dev/null 2>&1 && [[ ! -d "$HOME/.cache/ms-playwright" ]]; then
  (cd end2end && npx playwright install --with-deps chromium)
fi

echo ""
echo "== Full preview E2E (release, CI parity) =="
fuser -k 3010/tcp 2>/dev/null || true
sleep 1
export LEPTOS_BASE_PATH=/orbital
export LEPTOS_ENV=PROD
export LEPTOS_OUTPUT_NAME=orbital-preview
export ORBITAL_PREVIEW_BASE_PATH=/orbital
export COMPONENT_PREVIEW_BASE_URL=http://127.0.0.1:3010/orbital

cargo leptos end-to-end --release --project orbital-preview

echo ""
echo "Preview release gate passed. Safe to tag and push (Pages deploy only on TagPush)."
