#!/usr/bin/env python3
"""Audit orbital for web-app-template product-family couplings.

Parses product family names from ~/web-app-template workspace crates, greps orbital
for coupling patterns, and writes a markdown report. Excludes orbital self-refs and
unified-field-dev org URLs from flagged hits.

Usage:
  python3 scripts/maintainer/audit_product_couplings.py [--report PATH]
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WAT_ROOT = Path.home() / "web-app-template"
DEFAULT_REPORT = ROOT / "docs" / "product-coupling-audit-report.md"

# Same release surface as ip_release_gate.sh (exclude maintainer scripts that reference patterns).
AUDIT_GLOBS = ["orbital-*/src", "orbital/src"]

GENERIC_PALETTE = frozenset({"red", "green", "yellow", "orange"})

SUFFIXES = {
    "app",
    "server",
    "core",
    "macros",
    "codegen",
    "e2e",
    "topics",
    "worker",
    "bench",
    "lab",
    "embedded",
    "remote",
    "monolithic",
    "distributed",
    "identity",
    "admin",
    "leptos",
    "telemetry",
    "smtp",
    "preview",
    "integrations",
    "notifications",
    "auth",
    "search",
    "ssr",
    "welcome",
    "apps",
    "marketing",
    "history",
    "control",
    "plane",
    "handoff",
    "fleet",
    "ui",
    "platform",
    "store",
    "bootstrap",
    "reconcile",
    "quality",
    "wizard",
    "latency",
    "soak",
    "zone",
    "a",
    "lib",
    "datagrid",
    "charts",
    "pickers",
    "scheduler",
    "tree",
    "paging",
    "motion",
    "primitives",
    "style",
    "base",
    "components",
    "data",
    "shell",
    "theme",
    "frontend",
    "backend",
    "runtime",
    "axum",
    "testkit",
}


def family_from_crate(name: str) -> str:
    if name.startswith("setup-wizard"):
        return "setup-wizard"
    if name.startswith("record-history"):
        return "record-history"
    if name.startswith("counter-app"):
        return "counter"
    if name.startswith("unified-field"):
        return "unified-field"
    if name.startswith("lepton-auth"):
        return "lepton"
    for prefix in (
        "orbital-",
        "continuum-",
        "photon-",
        "gluon-",
        "valence-",
        "chronon-",
        "boson-",
        "spectra-",
        "neutrino-",
        "nucleus-",
        "higgs-",
        "pion-",
        "gauge-",
    ):
        if name.startswith(prefix):
            return prefix.rstrip("-")
    parts = name.split("-")
    while len(parts) > 1 and parts[-1] in SUFFIXES:
        parts.pop()
    return parts[0]


def load_families() -> list[str]:
    cargo = WAT_ROOT / "Cargo.toml"
    if not cargo.is_file():
        print(f"warning: {cargo} not found; using built-in family list", file=sys.stderr)
        return sorted(
            {
                "valence",
                "gluon",
                "nucleus",
                "chronon",
                "boson",
                "photon",
                "spectra",
                "neutrino",
                "higgs",
                "phonon",
                "polaron",
                "magnon",
                "pion",
                "gauge",
                "lepton",
                "continuum",
                "soliton",
                "parton",
                "tag",
                "counter",
                "setup-wizard",
            }
        )

    text = cargo.read_text()
    members_match = re.search(r"members\s*=\s*\[(.*?)\]", text, re.DOTALL)
    if not members_match:
        raise SystemExit(f"could not parse members from {cargo}")
    crates = re.findall(r'"([^"]+)"', members_match.group(1))
    all_crates = set(crates)
    for p in WAT_ROOT.rglob("Cargo.toml"):
        if "vendor" in p.parts:
            continue
        m = re.search(r'^\s*name\s*=\s*"([^"]+)"', p.read_text(), re.M)
        if m:
            all_crates.add(m.group(1))
    families = sorted({family_from_crate(c) for c in all_crates})
    families.extend(["phonon", "polaron", "magnon"])
    return sorted(set(families))


def rg(pattern: str) -> list[str]:
    args = ["rg", "-n", "--no-heading", "-e", pattern]
    for glob in AUDIT_GLOBS:
        args.extend(["--glob", glob])
    args.append(str(ROOT))
    proc = subprocess.run(args, capture_output=True, text=True)
    if proc.returncode not in (0, 1):
        raise SystemExit(proc.stderr or proc.stdout)
    return [line for line in proc.stdout.splitlines() if line.strip()]


def should_skip_line(line: str) -> bool:
    if "unified-field-dev" in line:
        return True
    if re.search(r"\borbital-preview\b|\borbital-theme\b|\borbital-shell\b", line):
        return False
    if re.search(r"\bOrbital\b|\borbital/", line) and "family-" not in line:
        return True
    return False


def classify_hit(line: str, family: str) -> str:
    if f"--orb-color-family-{family}" in line:
        return "css_var_family"
    if f"--orb-color-palette-{family}" in line:
        return "css_var_palette"
    if f"orbital-token-family-{family}" in line:
        return "token_class"
    if f"icons/{family}.rs" in line or f"icons::{family.title()}" in line:
        return "icon_module"
    if "PlatformFamilyBrand" in line or f"::{family.title()}" in line:
        return "enum_api"
    if f'"{family}"' in line:
        return "slug_string"
    return "other"


def audit() -> tuple[dict[str, list[tuple[str, str, str]]], list[str]]:
    families = load_families()
    product_families = [
        f
        for f in families
        if f not in {"orbital", "unified-field", "app", "server", "frontend", "quality"}
    ]

    hits: dict[str, list[tuple[str, str, str]]] = defaultdict(list)
    global_hits: list[str] = []

    family_lines = rg(r"--orb-color-family-")
    for line in family_lines:
        if should_skip_line(line):
            continue
        global_hits.append(line)

    for family in product_families:
        for kind, pattern in [
            ("css_var_family", rf"--orb-color-family-{family}"),
            ("css_var_palette", rf"--orb-color-palette-{family}"),
            ("token_class", rf"orbital-token-family-{family}"),
            ("enum_api", rf"PlatformFamilyBrand|::{family.title()}|Self::{family.title()}"),
            ("slug_string", rf'"{family}"'),
        ]:
            for line in rg(pattern):
                if should_skip_line(line):
                    continue
                if f"/icons/{family}.rs" in line or line.endswith(f"icons/{family}.rs"):
                    hits[family].append((line, "icon_file", kind))
                else:
                    hits[family].append((line, classify_hit(line, family), kind))

        icon_path = ROOT / "orbital-shell" / "src" / "icons" / f"{family}.rs"
        if icon_path.is_file():
            hits[family].append((str(icon_path), "icon_file", "icon_file"))

    for line in rg(r"PlatformFamilyBrand|BrandTone::Family|write_family_palette"):
        if not should_skip_line(line):
            global_hits.append(line)

    return hits, global_hits


def write_report(
    hits: dict[str, list[tuple[str, str, str]]],
    global_hits: list[str],
    path: Path,
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    total = sum(len(v) for v in hits.values()) + len(global_hits)
    lines = [
        "# Product coupling audit report",
        "",
        f"Generated by `scripts/maintainer/audit_product_couplings.py`.",
        "",
        f"**Total flagged hits:** {total}",
        "",
    ]

    if global_hits:
        lines.extend(["## Global patterns", ""])
        for line in sorted(set(global_hits)):
            lines.append(f"- `{line}`")
        lines.append("")

    for family in sorted(hits):
        entries = hits[family]
        if not entries:
            continue
        lines.append(f"## {family} ({len(entries)} hits)")
        lines.append("")
        seen = set()
        for line, kind, _ in sorted(entries):
            key = (line, kind)
            if key in seen:
                continue
            seen.add(key)
            lines.append(f"- [{kind}] `{line}`")
        lines.append("")

    path.write_text("\n".join(lines))
    print(f"Wrote {path} ({total} hits)")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--report",
        type=Path,
        default=DEFAULT_REPORT,
        help="Markdown report output path",
    )
    parser.add_argument(
        "--fail-on-hits",
        action="store_true",
        help="Exit 1 if any couplings are found",
    )
    args = parser.parse_args()

    hits, global_hits = audit()
    write_report(hits, global_hits, args.report)

    total = sum(len(v) for v in hits.values()) + len(global_hits)
    if args.fail_on_hits and total:
        print(f"FAIL: {total} product coupling hits", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
