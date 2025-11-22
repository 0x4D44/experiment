#!/usr/bin/env python3
"""Convenience runner for the Ghidra/Binary Ninja export pipeline."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List

try:
    import yaml  # type: ignore
except ImportError:  # pragma: no cover
    yaml = None

try:
    import jsonschema  # type: ignore
except ImportError:  # pragma: no cover
    jsonschema = None

REPO_ROOT = Path(__file__).resolve().parents[2]
ARTIFACT_DIR = REPO_ROOT / "re" / "artifacts"
LOG_DIR = ARTIFACT_DIR / "logs"
SCRIPT_DIR = REPO_ROOT / "re" / "ghidra" / "scripts"
SCHEMA_DIR = REPO_ROOT / "re" / "scripts" / "schemas"
CAPTURE_SCRIPT = REPO_ROOT / "re" / "scripts" / "capture_telemetry.py"
TRACE_INDEX_SCRIPT = REPO_ROOT / "re" / "scripts" / "link_traces.py"
COVERAGE_SCRIPT = REPO_ROOT / "re" / "scripts" / "coverage_report.py"
HLIL_PUBLISH_SCRIPT = REPO_ROOT / "re" / "scripts" / "publish_hlil.py"

SYMBOLS_PATH = ARTIFACT_DIR / "symbols.json"
SEGMENTS_PATH = ARTIFACT_DIR / "segments.yml"


def run_cmd(cmd: List[str], env: Dict[str, str], dry_run: bool, name: str, log_steps: List[Dict[str, Any]]):
    record: Dict[str, Any] = {"name": name, "command": cmd, "status": "skipped" if dry_run else "pending"}
    start = time.perf_counter()
    if dry_run:
        log_steps.append(record)
        return
    try:
        subprocess.run(cmd, check=True, env=env)
        record["status"] = "ok"
    except subprocess.CalledProcessError as exc:
        record["status"] = f"failed:{exc.returncode}"
        record["stderr"] = getattr(exc, "stderr", None)
        log_steps.append(record)
        raise
    finally:
        record["duration_s"] = round(time.perf_counter() - start, 3)
        log_steps.append(record)


def run_capture(args, log_steps):
    if not args.capture_run or not args.capture_macro:
        if args.capture_run or args.capture_macro:
            log_steps.append({"name": "capture", "status": "skipped", "reason": "missing run or macro"})
        return
    cmd = [
        sys.executable,
        str(CAPTURE_SCRIPT),
        args.capture_run,
        args.capture_macro,
    ]
    if args.capture_dry_run:
        cmd.append("--dry-run")
    if args.capture_timeout:
        cmd.extend(["--timeout", str(args.capture_timeout)])
    env = os.environ.copy()
    env.setdefault("F1GP_REPO_ROOT", str(REPO_ROOT))
    run_cmd(cmd, env, args.dry_run, "capture", log_steps)


def run_trace_index(args, log_steps):
    if args.skip_trace_index:
        log_steps.append({"name": "trace_index", "status": "skipped"})
        return
    cmd = [sys.executable, str(TRACE_INDEX_SCRIPT)]
    env = os.environ.copy()
    run_cmd(cmd, env, args.dry_run, "trace_index", log_steps)


def run_coverage(args, log_steps):
    if args.skip_coverage:
        log_steps.append({"name": "coverage", "status": "skipped"})
        return
    cmd = [sys.executable, str(COVERAGE_SCRIPT)]
    env = os.environ.copy()
    run_cmd(cmd, env, args.dry_run, "coverage", log_steps)


def run_hlil_publish(args, log_steps):
    if args.skip_hlil_publish:
        log_steps.append({"name": "hlil_publish", "status": "skipped"})
        return
    cmd = [sys.executable, str(HLIL_PUBLISH_SCRIPT)]
    env = os.environ.copy()
    run_cmd(cmd, env, args.dry_run, "hlil_publish", log_steps)


def run_ghidra(args, log_steps):
    if args.skip_ghidra:
        log_steps.append({"name": "ghidra", "status": "skipped"})
        return
    ghidra_headless = os.environ.get("GHIDRA_HEADLESS", "analyzeHeadless")
    project_dir = (REPO_ROOT / "re" / "ghidra" / "project_template").resolve()
    gp_path = (REPO_ROOT / "f1gp-disasm" / "GP.EXE").resolve()
    cmd = [
        ghidra_headless,
        str(project_dir),
        "F1GP_Auto",
        "-import",
        str(gp_path),
        "-overwrite",
        "-scriptPath",
        str(SCRIPT_DIR),
        "-postScript",
        "TagInterruptUsage.py",
        "-postScript",
        "ExportSymbolsToJson.java",
        "-postScript",
        "ExportSegmentsToYaml.java",
        "-postScript",
        "EmitTypeDecls.py",
    ]
    env = os.environ.copy()
    env["F1GP_REPO_ROOT"] = str(REPO_ROOT)
    run_cmd(cmd, env, args.dry_run, "ghidra", log_steps)


def run_binary_ninja(args, log_steps):
    if args.skip_binja:
        log_steps.append({"name": "binary_ninja", "status": "skipped"})
        return
    bn_headless = os.environ.get("BINARY_NINJA_HEADLESS", "binaryninja-headless")
    gp_path = (REPO_ROOT / "f1gp-disasm" / "GP.EXE").resolve()
    tmp_bndb = ARTIFACT_DIR / "tmp_refresh.bndb"
    loader = REPO_ROOT / "re" / "binary_ninja" / "plugins" / "f1gp_loader.py"
    importer = REPO_ROOT / "re" / "binary_ninja" / "plugins" / "import_symbols.py"
    hlil = REPO_ROOT / "re" / "binary_ninja" / "plugins" / "emit_hlil.py"
    cmd = [
        bn_headless,
        str(gp_path),
        str(tmp_bndb),
        "-t",
        "F1GP",
        "-s",
        str(loader),
        "-s",
        str(importer),
        "-s",
        str(hlil),
    ]
    env = os.environ.copy()
    env.setdefault("F1GP_REPO_ROOT", str(REPO_ROOT))
    env.setdefault("F1GP_SUBSYSTEMS", args.subsystems or "")
    run_cmd(cmd, env, args.dry_run, "binary_ninja", log_steps)
    if tmp_bndb.exists():
        tmp_bndb.unlink()


def load_yaml(path: Path) -> Any:
    if not path.exists():
        raise FileNotFoundError(path)
    if yaml is None:
        raise RuntimeError("PyYAML not installed; cannot validate YAML artifacts")
    with open(path, "r", encoding="utf-8") as fh:
        return yaml.safe_load(fh)


def validate_artifact(path: Path, schema_path: Path):
    if jsonschema is None:
        return {"status": "skipped", "reason": "jsonschema unavailable"}
    if path.suffix in {".json", ".JSON"}:
        with open(path, "r", encoding="utf-8") as fh:
            payload = json.load(fh)
    else:
        payload = load_yaml(path)
    with open(schema_path, "r", encoding="utf-8") as fh:
        schema = json.load(fh) if schema_path.suffix == ".json" else yaml.safe_load(fh)
    jsonschema.validate(payload, schema)
    return {"status": "ok"}


def validate_artifacts(args, log_steps):
    if args.skip_validate:
        log_steps.append({"name": "validate", "status": "skipped"})
        return
    try:
        sym_result = validate_artifact(SYMBOLS_PATH, SCHEMA_DIR / "symbols.schema.json")
        seg_result = validate_artifact(SEGMENTS_PATH, SCHEMA_DIR / "segments.schema.yml")
        log_steps.append({"name": "validate", "status": "ok", "symbols": sym_result, "segments": seg_result})
    except Exception as exc:
        log_steps.append({"name": "validate", "status": f"failed:{exc}"})
        raise


def write_log(log_steps: List[Dict[str, Any]]):
    LOG_DIR.mkdir(parents=True, exist_ok=True)
    payload = {
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "steps": log_steps,
    }
    out_path = LOG_DIR / f"refresh_{datetime.now().strftime('%Y%m%dT%H%M%S')}.json"
    with open(out_path, "w", encoding="utf-8") as fh:
        json.dump(payload, fh, indent=2)
    print(f"Wrote log to {out_path}")


def parse_args():
    parser = argparse.ArgumentParser(description="Refresh RE artifacts")
    parser.add_argument("--dry-run", action="store_true", help="Print commands without executing")
    parser.add_argument("--skip-ghidra", action="store_true")
    parser.add_argument("--skip-binja", action="store_true")
    parser.add_argument("--skip-validate", action="store_true")
    parser.add_argument("--subsystems", default="", help="Comma-separated subsystem filter for HLIL export")
    parser.add_argument("--capture-run", help="Capture telemetry before exports (run id)")
    parser.add_argument("--capture-macro", help="Macro path for telemetry capture")
    parser.add_argument("--capture-timeout", type=int, default=300)
    parser.add_argument("--capture-dry-run", action="store_true")
    parser.add_argument("--skip-trace-index", action="store_true")
    parser.add_argument("--skip-coverage", action="store_true")
    parser.add_argument("--skip-hlil-publish", action="store_true")
    return parser.parse_args()


def main():
    args = parse_args()
    log_steps: List[Dict[str, Any]] = []
    try:
        run_capture(args, log_steps)
        run_trace_index(args, log_steps)
        run_ghidra(args, log_steps)
        run_binary_ninja(args, log_steps)
        validate_artifacts(args, log_steps)
        run_coverage(args, log_steps)
        run_hlil_publish(args, log_steps)
    finally:
        write_log(log_steps)


if __name__ == "__main__":
    try:
        main()
    except Exception as exc:
        print(f"refresh_artifacts failed: {exc}", file=sys.stderr)
        sys.exit(1)
