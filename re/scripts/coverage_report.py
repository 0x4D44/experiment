#!/usr/bin/env python3
"""Generate coverage summary from symbols.json."""

from __future__ import annotations

import json
from collections import Counter, defaultdict
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
SYMBOLS_PATH = REPO_ROOT / "re" / "artifacts" / "symbols.json"
OUT_PATH = REPO_ROOT / "re" / "artifacts" / "coverage_summary.md"


def main():
    if not SYMBOLS_PATH.exists():
        raise SystemExit(f"Missing {SYMBOLS_PATH}")
    data = json.loads(SYMBOLS_PATH.read_text(encoding="utf-8"))
    functions = data.get("functions", [])
    by_subsystem = defaultdict(list)
    int_usage = Counter()
    named = 0
    for fn in functions:
        subsystem = fn.get("subsystem", "unknown") or "unknown"
        by_subsystem[subsystem].append(fn)
        if fn.get("name") and not fn.get("name", "").lower().startswith("FUN_"):
            named += 1
        for intr in fn.get("int_usage", []):
            int_usage[intr] += 1

    lines = []
    lines.append("# Reverse Engineering Coverage Summary\n")
    lines.append(f"Total functions: {len(functions)}")
    lines.append(f"Named functions: {named}")
    lines.append("\n## Breakdown by subsystem\n")
    for subsystem, items in sorted(by_subsystem.items(), key=lambda kv: kv[0]):
        lines.append(f"- **{subsystem}**: {len(items)} functions")
    lines.append("\n## INT usage counts\n")
    if int_usage:
        for intr, count in sorted(int_usage.items()):
            lines.append(f"- {intr}: {count}")
    else:
        lines.append("- (none recorded)")

    OUT_PATH.write_text("\n".join(lines), encoding="utf-8")
    print(f"Wrote coverage summary to {OUT_PATH}")


if __name__ == "__main__":
    main()
