#!/usr/bin/env bash
# IP release gate — fail if tracked release surface contains upstream fingerprints.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

fail=0

echo "== IP fingerprint grep (tracked release surface) =="
if rg -i 'fluent|mui|lemmy|@mui|@fluentui|mui\.com|fluentui\.com|canonical_fluent|DataGrid|Mui[A-Z]' \
  --glob '!docs/internal/**' \
  --glob '!vendor/**' \
  --glob '!target/**' \
  --glob '!target-site-preview/**' \
  --glob '!scripts/maintainer/ip_release_gate.sh' \
  --glob '!scripts/maintainer/audit_product_couplings.py' \
  --glob '!scripts/maintainer/product_coupling_gate.sh' \
  --glob '!end2end/node_modules/**' \
  --glob '!end2end/playwright-report/**' \
  --glob '!end2end/test-results/**' \
  --glob '!end2end/blob-report/**' \
  README.md docs/ orbital-*/src/ orbital/ end2end/tests/ scripts/ 2>/dev/null; then
  echo "FAIL: upstream fingerprint matches above"
  fail=1
else
  echo "PASS: no upstream fingerprint matches"
fi

echo ""
echo "== Legacy Fluent CSS vars in shipped source =="
if rg 'var\(--color[A-Z]|--colorBrand|--strokeWidth' orbital-*/src/ 2>/dev/null; then
  echo "FAIL: legacy CSS token vars above"
  fail=1
else
  echo "PASS: no legacy CSS token vars"
fi

echo ""
echo "== Tracked research / design artifacts =="
if git ls-files '**/docs/research/**' '**/docs/design.md' 'orbital/COMPONENT_REGISTRY.md' 2>/dev/null | grep -q .; then
  echo "FAIL: tracked paths that should be internal-only:"
  git ls-files '**/docs/research/**' '**/docs/design.md' 'orbital/COMPONENT_REGISTRY.md'
  fail=1
else
  echo "PASS: no tracked research/design/registry artifacts"
fi

echo ""
echo "== Product-family CSS tokens (orbital-only theme) =="
PRODUCT_FAMILIES=(
  valence gluon nucleus chronon boson photon spectra neutrino higgs phonon polaron magnon
  pion gauge lepton continuum soliton parton tag counter setup-wizard
)
if rg -e '--orb-color-family-' orbital-*/src/ orbital/src/ 2>/dev/null; then
  echo "FAIL: --orb-color-family-* tokens must not ship from orbital"
  fail=1
else
  echo "PASS: no --orb-color-family-* tokens"
fi

palette_fail=0
for family in "${PRODUCT_FAMILIES[@]}"; do
  if rg -e "--orb-color-palette-${family}-" orbital-*/src/ orbital/src/ 2>/dev/null; then
    echo "FAIL: product-named palette token --orb-color-palette-${family}-* above"
    palette_fail=1
    fail=1
  fi
done
if [[ "$palette_fail" -eq 0 ]]; then
  echo "PASS: no product-named --orb-color-palette-* tokens"
fi

if rg 'PlatformFamilyBrand|BrandTone::Family|write_family_palette' orbital-*/src/ orbital/src/ 2>/dev/null; then
  echo "FAIL: product-family brand API above"
  fail=1
else
  echo "PASS: no PlatformFamilyBrand / family palette API"
fi

echo ""
echo "== Product coupling audit script =="
python3 scripts/maintainer/audit_product_couplings.py --fail-on-hits

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi

echo ""
echo "IP release gate passed."
