# Subsystem Lifting Playbook (Stage 5)

This guide explains how to execute the Stage 5 sprints (Physics, Input, Graphics, Audio, AI) when Ghidra/Binary Ninja are available.

## 1. Prep Checklist
- `f1gp-disasm/GP.EXE` hash matches `re/SOURCE_MANIFEST.md`.
- `re/scripts/refresh_artifacts.py` runs successfully with telemetry capture and HLIL publishing enabled.
- `re/artifacts/coverage_summary.md` shows baseline counts (use to track progress).

## 2. Sprint Workflow
1. **Target Selection**: choose subsystem (e.g., Physics). Record sprint goal in `wrk_journals/…`.
2. **Ghidra Session**:
   - Load latest project template.
   - Annotate functions (naming, comments, `@subsystem:<name>`, `@trace:<run_id>`).
   - Run `TagInterruptUsage`, `ExportSymbolsToJson`, `ExportSegmentsToYaml`, `EmitTypeDecls`.
3. **Telemetry Alignment**:
   - Capture trace (DOSBox macro) targeting subsystem behavior.
   - Update metadata/`source_recording_id` references.
4. **Binary Ninja HLIL**:
   - Load updated artifacts, run `emit_hlil.py` for the subsystem.
   - Review HLIL readability; add TODO comments where manual cleanup needed.
5. **Export & Publish**:
   - Run `refresh_artifacts.py` (no `--skip-*`).
   - Verify `re/artifacts/hlil/<subsystem>.c` copied into `f1gp-port/docs/re/`.
6. **Parity Harness Impact**:
   - Update `f1gp-port/tests/data/<subsystem>_meta.json` (if needed).
   - Record new coverage numbers in journal.

## 3. Definition of Done per Sprint
- ≥10 new named functions or 20% increase in subsystem coverage (whichever first).
- HLIL export committed and published to `f1gp-port/docs/re/`.
- Telemetry trace metadata captured under `f1gp-data/dos_traces/<run_id>/` + referenced via `@trace:` tags.
- Journal entry summarizing findings + blockers.
- Parity harness note indicating whether new trace affects diff thresholds.

## 4. Blocking Issues
- If Ghidra/Binary Ninja scripts fail, capture logs from `re/artifacts/logs/` and update `wrk_journals` with error detail.
- Licensing issues (Binary Ninja headless) should be escalated before sprint start.

## 5. Reporting
- After each sprint, rerun `coverage_report.py` and attach diff of `coverage_summary.md` in PR.
- Use `re/docs/ci.md` as reference for required secrets/config before launching CI runs.

Future revisions of this playbook can add subsystem-specific heuristics once we have real data.
