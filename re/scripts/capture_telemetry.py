#!/usr/bin/env python3
"""Drive DOSBox-X to capture deterministic telemetry runs."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path
from typing import List

REPO_ROOT = Path(__file__).resolve().parents[2]
DOSBOX_CONFIG = REPO_ROOT / "re" / "scripts" / "dosbox-x.conf"
TRACE_DIR = REPO_ROOT / "f1gp-data" / "dos_traces"


def read_macro_lines(path: Path) -> List[str]:
    lines = []
    for raw in path.read_text(encoding="utf-8").splitlines():
        stripped = raw.strip()
        if not stripped or stripped.startswith("#") or stripped.upper().startswith("REM"):
            continue
        lines.append(stripped)
    if not lines:
        raise ValueError(f"macro {path} produced no executable lines")
    return lines


def run_dosbox(macro_lines: List[str], env, dry_run: bool, timeout: int) -> int:
    dosbox = env.get("DOSBOX_X", os.environ.get("DOSBOX_X", "dosbox-x"))
    cmd = [dosbox, "-conf", str(DOSBOX_CONFIG)]
    for line in macro_lines:
        cmd.extend(["-c", line])
    if macro_lines[-1].lower() != "exit":
        cmd.extend(["-c", "exit"])
    if dry_run:
        print("[dry-run]", " ".join(cmd))
        return 0
    try:
        subprocess.run(cmd, check=True, env=env, timeout=timeout)
    except subprocess.TimeoutExpired as exc:
        print(f"dosbox-x timed out: {exc}", file=sys.stderr)
        return 124
    except subprocess.CalledProcessError as exc:
        print(f"dosbox-x failed: {exc}", file=sys.stderr)
        return exc.returncode
    return 0


def capture(run_id: str, macro_path: Path, dry_run: bool, timeout: int):
    macro_lines = read_macro_lines(macro_path)
    TRACE_DIR.mkdir(parents=True, exist_ok=True)
    out_dir = TRACE_DIR / run_id
    out_dir.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    env.setdefault("F1GP_REPO_ROOT", str(REPO_ROOT))
    status = run_dosbox(macro_lines, env, dry_run, timeout)
    metadata = {
        "run_id": run_id,
        "macro": str(macro_path.relative_to(REPO_ROOT)),
        "binary": "f1gp-disasm/GP.EXE",
        "dosbox_conf": str(DOSBOX_CONFIG.relative_to(REPO_ROOT)),
        "status": status,
    }
    with open(out_dir / "metadata.json", "w", encoding="utf-8") as fh:
        json.dump(metadata, fh, indent=2)
    if dry_run:
        print(f"[dry-run] metadata stub written to {out_dir}")
    else:
        print(f"Capture complete (exit={status}) -> {out_dir}")
    trace_file = out_dir / "trace.bin"
    if not trace_file.exists():
        trace_file.write_bytes(b"TODO: replace with real telemetry")


def main():
    parser = argparse.ArgumentParser(description="Capture DOS telemetry via DOSBox-X")
    parser.add_argument("run_id")
    parser.add_argument("macro", help="Path to macro text file")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--timeout", type=int, default=300, help="Timeout seconds for DOSBox-X run")
    args = parser.parse_args()
    capture(args.run_id, Path(args.macro), args.dry_run, args.timeout)


if __name__ == "__main__":
    main()
