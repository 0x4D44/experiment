# Telemetry Harness Specification (Stage 4 Draft)

## Goals
- Capture deterministic gameplay traces using DOSBox-X for regression comparisons between original DOS binary and modern port.
- Link each captured trace to disassembled functions via `source_recording_id` fields.
- Provide reproducible scripts that CI can run headlessly for smoke tests (short scenarios only).

## Components
1. `re/scripts/capture_telemetry.py`
 - CLI stub (to be extended) that will invoke DOSBox-X with a supplied automation script.
  - Reads macro files from `re/scripts/telemetry_macros/`, issues each line as a `-c` command to DOSBox-X, and writes metadata to `f1gp-data/dos_traces/<run_id>/metadata.json`.
  - Supports `--dry-run` (no DOSBox invocation) and `--timeout` for CI robustness.
2. `re/scripts/dosbox-x.conf`
   - Base configuration (memsize, mount commands). Update with final VGA/SB settings and automation script loading once tested.
3. `f1gp-data/dos_traces/`
   - Stores run directories with metadata, raw traces, optional video captures.
4. `re/artifacts/telemetry/`
   - Stores summarized metadata referencing `run_id`, hashed artifacts, and scenario descriptions; `link_traces.py` emits `index.json` for CICD consumption.

## Planned Workflow
1. Author a macro (DOS command list) under `re/scripts/telemetry_macros/` – sample provided in `sample_boot.txt`.
2. Run `python3 re/scripts/capture_telemetry.py RUN_ID re/scripts/telemetry_macros/sample_boot.txt` (or add `--capture-run` / `--capture-macro` flags to `refresh_artifacts.py` to inline the step).
3. Script invokes DOSBox-X (once wired) to launch GP.EXE, execute macro, and save telemetry to `trace.bin`.
4. After capture, update Ghidra function comments with `@trace:RUN_ID` and re-run exporters so metadata references this trace.
5. Commit sanitized sample metadata (`sample_boot`) for CI smoke tests; keep real traces private until licensing cleared.
6. The parity harness consumes both the DOS trace and HLIL/Rust output to assert equivalence.

## Open Items
- Decide on telemetry format (binary struct vs. Cap'n Proto vs. JSON). Suggest Cap'n Proto for determinism + size.
- Determine automation scripting mechanism (DOSBox-X keymapper vs. built-in debugger). Consider using `INPUT.TXT` playback.
- Integrate with CI by bundling a small trace archive (<5 MB) into repository or release artifacts to avoid large downloads.
