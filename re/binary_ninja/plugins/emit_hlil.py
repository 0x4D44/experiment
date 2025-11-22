"""
Emit HLIL for selected subsystems based on re/artifacts/symbols.json metadata.
Usage (headless example):
    F1GP_SUBSYSTEMS=physics,audio binaryninja-headless \
        f1gp-disasm/GP.EXE out.bndb \
        -e 'run_script(bv)'
"""

import json
import os
from collections import defaultdict
from typing import Dict, Any, List

from binaryninja import BinaryView, log_error, log_info  # type: ignore

SYMBOLS_REL_PATH = os.path.join("re", "artifacts", "symbols.json")
HLIL_DIR = os.path.join("re", "artifacts", "hlil")


def load_symbols(repo_root: str) -> Dict[str, Any]:
    path = os.path.join(repo_root, SYMBOLS_REL_PATH)
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)


def hlil_text(function) -> str:
    try:
        return function.hlil.source  # type: ignore[attr-defined]
    except AttributeError:
        try:
            return str(function.hlil)
        except Exception:
            return f"// HLIL unavailable for {function.name}\n"


def emit_for_subsystems(bv: BinaryView, subsystems: List[str]):
    repo_root = os.getenv("F1GP_REPO_ROOT", os.getcwd())
    symbols = load_symbols(repo_root)
    os.makedirs(os.path.join(repo_root, HLIL_DIR), exist_ok=True)

    grouped = defaultdict(list)
    for fn in symbols.get("functions", []):
        grouped[fn.get("subsystem", "unknown")].append(fn)

    targets = subsystems or list(grouped.keys())
    for subsystem in targets:
        entries = grouped.get(subsystem)
        if not entries:
            log_info(f"No symbols for subsystem {subsystem}")
            continue
        outfile = os.path.join(repo_root, HLIL_DIR, f"{subsystem}.c")
        with open(outfile, "w", encoding="utf-8") as fh:
            fh.write(f"/* HLIL export for subsystem: {subsystem} */\n\n")
            for entry in entries:
                addr_hex = entry.get("entry_linear")
                if not addr_hex:
                    continue
                addr = int(addr_hex, 16)
                function = bv.get_function_at(addr)
                if not function:
                    fh.write(f"// Missing function at {addr_hex} ({entry.get('name')})\n\n")
                    continue
                fh.write(f"// {entry.get('name')} @ {addr_hex}\n")
                fh.write(hlil_text(function))
                fh.write("\n\n")
        log_info(f"Wrote HLIL for {subsystem} -> {outfile}")


def run_script(bv: BinaryView):
    env = os.getenv("F1GP_SUBSYSTEMS", "")
    subsystems = [s.strip() for s in env.split(",") if s.strip()]
    emit_for_subsystems(bv, subsystems)


if "bv" in globals():  # type: ignore[name-defined]
    run_script(bv)
else:  # pragma: no cover
    log_error("Binary Ninja 'bv' not found; run through headless CLI or scripting console.")
