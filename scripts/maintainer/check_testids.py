#!/usr/bin/env python3
"""Check literal data-testid values in Rust sources against the naming rules in
docs/component-testing.md (lowercase ASCII kebab-case)."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SKIP_DIRS = {"target", "node_modules", ".git"}

TESTID_RE = re.compile(r'data-testid="([^"]*)"')
KEBAB_RE = re.compile(r"[a-z0-9]+(-[a-z0-9]+)*")

# Legacy ids that predate the rules. Do not add new entries.
LEGACY_RE = re.compile(
    r"|".join(
        [
            r"[A-Z]{2,3}-\d{2}(-[A-Za-z]+)?",  # test-case ids: CA-02, DP-09-TEXT, TP-08-query
            r"[a-z0-9-]+-(VALUE|LABEL)",  # date-pickers readouts
        ]
    )
)


def rust_files() -> list[Path]:
    return [
        p
        for p in ROOT.rglob("*.rs")
        if not SKIP_DIRS.intersection(p.relative_to(ROOT).parts)
    ]


def main() -> int:
    failures: list[str] = []
    for path in rust_files():
        text = path.read_text(encoding="utf-8", errors="replace")
        for match in TESTID_RE.finditer(text):
            value = match.group(1)
            if KEBAB_RE.fullmatch(value) or LEGACY_RE.fullmatch(value):
                continue
            line = text.count("\n", 0, match.start()) + 1
            failures.append(f"{path.relative_to(ROOT)}:{line}: {value!r}")
    if failures:
        print("data-testid values that are not lowercase kebab-case:")
        print("\n".join(failures))
        return 1
    print("data-testid naming OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
