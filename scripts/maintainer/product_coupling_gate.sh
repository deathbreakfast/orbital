#!/usr/bin/env bash
# Fail if orbital ships product-family theme tokens or brand API.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

fail=0

PRODUCT_FAMILIES=(
  valence gluon nucleus chronon boson photon spectra neutrino higgs phonon polaron magnon
  pion gauge lepton continuum soliton parton tag counter setup-wizard
)

echo "== Product-family CSS tokens (orbital-only theme) =="
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
echo "Product coupling gate passed."
