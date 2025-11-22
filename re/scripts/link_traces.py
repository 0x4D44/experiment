#!/usr/bin/env python3
"""Builds re/artifacts/telemetry/index.json from f1gp-data/dos_traces."""

from __future__ import annotations

import argparse
import json
import hashlib
from pathlib import Path
from typing import Dict, Any

REPO_ROOT = Path(__file__).resolve().parents[2]
TRACE_DIR = REPO_ROOT / "f1gp-data" / "dos_traces"
OUT_FILE = REPO_ROOT / "re" / "artifacts" / "telemetry" / "index.json"


def hash_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def build_index() -> Dict[str, Any]:
    entries = []
    if not TRACE_DIR.exists():
        return {"traces": entries}
    for run_dir in sorted(p for p in TRACE_DIR.iterdir() if p.is_dir()):
        meta_path = run_dir / "metadata.json"
        if not meta_path.exists():
            continue
        with meta_path.open("r", encoding="utf-8") as fh:
            metadata = json.load(fh)
        entry: Dict[str, Any] = {
            "run_id": metadata.get("run_id", run_dir.name),
            "metadata": str(meta_path.relative_to(REPO_ROOT)),
            "macro": metadata.get("macro"),
            "binary": metadata.get("binary"),
            "status": metadata.get("status"),
        }
        trace_path = run_dir / "trace.bin"
        if trace_path.exists():
            entry["trace"] = {
                "path": str(trace_path.relative_to(REPO_ROOT)),
                "sha256": hash_file(trace_path),
                "size": trace_path.stat().st_size,
            }
        entries.append(entry)
    return {"traces": entries}


def main():
    parser = argparse.ArgumentParser(description="Build telemetry index")
    parser.add_argument("--output", default=str(OUT_FILE))
    args = parser.parse_args()
    payload = build_index()
    out_path = Path(args.output)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    with out_path.open("w", encoding="utf-8") as fh:
        json.dump(payload, fh, indent=2)
    print(f"Wrote telemetry index with {len(payload['traces'])} entries -> {out_path}")


if __name__ == "__main__":
    main()
