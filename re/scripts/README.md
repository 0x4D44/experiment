# Automation Scripts

- `refresh_artifacts.py` – orchestrates headless Ghidra + Binary Ninja exports, validates artifacts, logs execution metadata, runs optional telemetry captures, rebuilds the telemetry index, and emits coverage reports.
- `capture_telemetry.py` – invokes DOSBox-X with macros under `telemetry_macros/` to record deterministic traces into `f1gp-data/dos_traces/` (supports `--dry-run`).
- `link_traces.py` – scans `f1gp-data/dos_traces/` and writes `re/artifacts/telemetry/index.json` (hash + path info).
- `coverage_report.py` – summarizes `re/artifacts/symbols.json` into `re/artifacts/coverage_summary.md` (functions per subsystem, INT usage counts).
- `publish_hlil.py` – copies `re/artifacts/hlil/*.c` into `f1gp-port/docs/re/` to keep gameplay docs in sync.
- `schemas/` – JSON/YAML schemas used by the validation step to ensure artifacts remain well-formed.

Usage example:
```
cd /home/md/language/experiment
# Ensure analyzeHeadless + binaryninja-headless available, jsonschema + PyYAML installed
python3 re/scripts/refresh_artifacts.py \
  --dry-run \
  --subsystems physics,audio \
  --capture-run TEST_RUN \
  --capture-macro re/scripts/telemetry_macros/sample_boot.txt \
  --capture-dry-run \
  --skip-trace-index \
  --skip-hlil-publish
```
