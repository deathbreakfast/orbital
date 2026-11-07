#!/usr/bin/env python3
"""Audit #[component_doc] rustdoc coverage for props and live example descriptions."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

CRATE_PATHS: dict[str, list[str]] = {
    "orbital-core-components": ["orbital-core-components/src"],
    "orbital": ["orbital/src"],
    "orbital-datatable": ["orbital-datatable/src"],
    "orbital-charts": ["orbital-charts/src"],
    "orbital-scheduler": ["orbital-scheduler/src"],
    "orbital-graph": ["orbital-graph/src"],
    "orbital-date-pickers": ["orbital-date-pickers/src"],
    "orbital-primitives": ["orbital-primitives/src"],
}

PRODUCTION_CRATES = ("orbital-core-components", "orbital")


def get_slug(text: str) -> str | None:
    match = re.search(r'preview_slug\s*=\s*"([^"]+)"', text)
    return match.group(1) if match else None


def is_manual_preview(text: str) -> bool:
    return bool(re.search(r'preview\s*=\s*"manual"', text))


def get_doc_comment(text: str) -> str:
    match = re.search(r"((?:///.*\n)+)\s*#\[component_doc\(", text)
    if not match:
        return ""
    lines: list[str] = []
    for line in match.group(1).split("\n"):
        if line.startswith("///"):
            lines.append(line[4:].lstrip())
        elif line.strip() == "///":
            lines.append("")
    return "\n".join(lines)


def parse_examples(doc: str) -> list[dict]:
    match = re.search(r"#\s+Examples?\s*\n(.*)", doc, re.S)
    if not match:
        return []
    body = match.group(1)
    variants: list[dict] = []
    for part in re.split(r"\n##\s+", body):
        if not part.strip():
            continue
        title = part.split("\n", 1)[0].strip()
        pre = part.split("```")[0]
        desc_lines = [
            line.strip()
            for line in pre.split("\n")[1:]
            if line.strip() and not line.strip().startswith("<!--")
        ]
        description = " ".join(desc_lines).strip()
        code_only = "<!-- code-only -->" in pre
        preview = "<!-- preview -->" in pre and not code_only
        variants.append(
            {
                "title": title,
                "description": description,
                "preview": preview,
            }
        )
    return variants


def parse_props(text: str) -> list[tuple[str, bool]]:
    # Bind props to the #[component] immediately after #[component_doc], not helper fns above.
    match = re.search(
        r"#\[component_doc\([^)]*\)\]\s*\n#\[component\]\s*\n(?:pub\s+)?fn\s+\w+\s*\((.*?)\)\s*->",
        text,
        re.S,
    )
    if not match:
        return []
    params = match.group(1)
    props: list[tuple[str, bool]] = []
    for chunk in re.split(r",\n\s*(?=#\[prop|///|children|\w)", params):
        chunk = chunk.strip()
        if not chunk:
            continue
        doc_match = re.search(r"///\s*(.+)", chunk)
        documented = bool(doc_match and doc_match.group(1).strip())
        if "children:" in chunk or chunk.startswith("children:"):
            name = "children"
        else:
            name_match = re.search(r"(\w+)\s*:\s*", chunk)
            name = name_match.group(1) if name_match else "?"
        if name != "?":
            props.append((name, documented))
    return props


def iter_component_files(crate_names: list[str]) -> list[Path]:
    files: list[Path] = []
    for crate in crate_names:
        for rel in CRATE_PATHS.get(crate, []):
            base = ROOT / rel
            if not base.exists():
                continue
            files.extend(sorted(base.rglob("*.rs")))
    return files


def audit_file(path: Path) -> dict | None:
    text = path.read_text(encoding="utf-8", errors="ignore")
    slug = get_slug(text)
    if not slug:
        return None
    manual = is_manual_preview(text)
    doc = get_doc_comment(text)
    examples = parse_examples(doc)
    props = parse_props(text)
    undoc_props = [name for name, documented in props if not documented]
    missing_example_desc = [
        ex["title"]
        for ex in examples
        if ex["preview"] and not ex["description"]
    ]
    no_examples = not examples and not manual
    return {
        "slug": slug,
        "path": str(path.relative_to(ROOT)),
        "manual": manual,
        "undoc_props": undoc_props,
        "missing_example_desc": missing_example_desc,
        "no_examples": no_examples,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--crates",
        default=",".join(PRODUCTION_CRATES),
        help="Comma-separated crate names to audit (default: production crates)",
    )
    parser.add_argument(
        "--warn",
        action="store_true",
        help="Print violations but exit 0",
    )
    args = parser.parse_args()
    crate_names = [c.strip() for c in args.crates.split(",") if c.strip()]

    rows: list[dict] = []
    for path in iter_component_files(crate_names):
        row = audit_file(path)
        if row:
            rows.append(row)

    prop_gaps = [r for r in rows if r["undoc_props"]]
    example_gaps = [r for r in rows if r["missing_example_desc"]]
    no_examples = [r for r in rows if r["no_examples"]]

    violations = 0
    if prop_gaps:
        violations += len(prop_gaps)
        print("UNDOCUMENTED PROPS:")
        for row in sorted(prop_gaps, key=lambda r: r["slug"]):
            print(f"  {row['slug']}: {row['undoc_props']} ({row['path']})")
        print()

    if example_gaps:
        violations += len(example_gaps)
        print("MISSING LIVE EXAMPLE DESCRIPTIONS:")
        for row in sorted(example_gaps, key=lambda r: r["slug"]):
            titles = ", ".join(row["missing_example_desc"][:4])
            extra = "..." if len(row["missing_example_desc"]) > 4 else ""
            print(
                f"  {row['slug']}: {len(row['missing_example_desc'])} variant(s) "
                f"— {titles}{extra} ({row['path']})"
            )
        print()

    if no_examples:
        violations += len(no_examples)
        print("MISSING # Examples SECTION:")
        for row in sorted(no_examples, key=lambda r: r["slug"]):
            print(f"  {row['slug']}: {row['path']}")
        print()

    if violations:
        print(f"{violations} component(s) with documentation gaps.")
        print("See docs/component-testing.md (Component authoring).")
        return 0 if args.warn else 1

    print(
        f"All {len(rows)} audited component(s) in {', '.join(crate_names)} "
        "have prop docs and live example descriptions."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
