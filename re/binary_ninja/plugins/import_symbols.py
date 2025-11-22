"""
Binary Ninja helper: import symbols/types from re/artifacts/symbols.json.
Run via: `binaryninja -s re/binary_ninja/plugins/import_symbols.py f1gp-disasm/GP.EXE`
"""

import json
import os
from typing import Dict, Any

from binaryninja import (  # type: ignore
    BinaryView,
    Symbol,
    SymbolType,
    log_error,
    log_info,
)

SYMBOLS_REL_PATH = os.path.join("re", "artifacts", "symbols.json")


def load_symbols(repo_root: str) -> Dict[str, Any]:
    path = os.path.join(repo_root, SYMBOLS_REL_PATH)
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)


def apply_symbols(bv: BinaryView):
    repo_root = os.getenv("F1GP_REPO_ROOT", os.getcwd())
    try:
        payload = load_symbols(repo_root)
    except Exception as exc:  # pragma: no cover
        log_error(f"Failed to load symbols.json: {exc}")
        return

    funcs = payload.get("functions", [])
    created = 0
    for fn in funcs:
        name = fn.get("name")
        addr = fn.get("entry_linear")
        if not name or not addr:
            continue
        linear = int(addr, 16)
        symbol = Symbol(SymbolType.FunctionSymbol, linear, name)
        bv.define_user_symbol(symbol)
        bv.add_function(linear)
        created += 1
    log_info(f"Imported {created} symbols from artifacts")


def run_script(bv: BinaryView):
    apply_symbols(bv)


if "bv" in globals():
    run_script(bv)  # type: ignore[name-defined]
else:  # pragma: no cover
    log_error("Binary Ninja 'bv' not found; run inside BN scripting console or headless session.")
