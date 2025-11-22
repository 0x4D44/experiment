# Parity Harness (Phase 1 Outline)

_Last updated: 2025-11-19_

This document describes how to compare sanitized telemetry captures between the original DOS version (once converted to
CSV) and the Rust port. The goal is to provide a repeatable pipeline ahead of full automation.

## Prerequisites
- DOS capture CSV (`captures/<track>/<timestamp>/dos.csv`) generated via the DOS serial parser.
- Rust capture CSV exported via `telemetry_cli`.
- `telemetry_cli diff` command available via the workspace (`cargo run -p telemetry_cli -- diff ...`).

## Workflow
1. Export both captures to CSV (see data pipeline guide).
2. Invoke the diff subcommand:
   ```bash
   cargo run -p telemetry_cli -- diff \
     --reference captures/monaco/20251118/dos.csv \
     --candidate exports/monaco_rust.csv \
     --car 0
   ```
3. Review the output:
   - `Average speed delta` must be < 2 km/h for parity acceptance.
   - The script currently aligns samples by index; future iterations will resample on timestamps.
4. Record the delta plus supporting metadata in `captures/parity_report.md`.

## Future Enhancements
- Timestamp-based interpolation (nearest-neighbor, optionally resampling to 10 ms).
- Additional metrics: lap time delta, throttle/brake correlation, steering RMS error.
- GitHub Action invoking the diff for designated fixtures.

Use the synthetic fixture (`data/fixtures/telemetry/synthetic_monaco.csv`) to validate the pipeline end-to-end without DOS assets:
```bash
./scripts/generate_fixture_csvs.sh
cargo run -p telemetry_cli -- diff \
  --reference data/fixtures/telemetry/synthetic_monaco.csv \
  --candidate data/fixtures/telemetry/synthetic_monaco.csv
```

## Integration with Disassembly Artifacts
- `re/artifacts/symbols.json` now includes `source_recording_id`; log the corresponding run ID in parity reports so issues can be traced back to the DOS capture.
- `re/scripts/refresh_artifacts.py` exposes `--capture-run/--capture-macro` flags, allowing parity jobs to regenerate traces automatically before comparisons.
- Capture metadata lives under `f1gp-data/dos_traces/<run_id>/metadata.json` with DOSBox config + macro references; cite this path whenever filing regressions.
- Sample metadata included in-repo (`f1gp-data/dos_traces/sample_boot/metadata.json` and `f1gp-port/tests/data/sample_capture_meta.json`) keeps CI parity checks green until real traces are wired in.
