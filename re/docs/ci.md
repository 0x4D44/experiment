# Reverse Engineering CI Overview

This document explains the three CI workflows added in Stage 4 and the secrets/config they require.

## Workflows

1. **re-ghidra.yml**
   - Triggers on changes to Ghidra scripts/artifacts.
   - Installs Temurin JDK 17, downloads Ghidra (URL override via `GHIDRA_URL`).
  - Runs `re/scripts/refresh_artifacts.py --skip-binja` to regenerate symbols/segments, rebuild the telemetry index, and emit the coverage summary, then re-runs in dry-run mode for validation.
  - Optional: pass `--capture-run <id> --capture-macro <path>` when telemetry capture should precede exports (requires DOSBox-X on runner).
   - Uploads `symbols.json`, `segments.yml`, type headers, and logs as build artifacts.

2. **re-binja.yml**
   - Requires secrets:
     - `BINJA_LICENSE` – contents of Binary Ninja headless license file.
     - `BINJA_DOWNLOAD_URL` – pre-authorized URL to download the headless build.
   - Runs Binary Ninja export only, filtering subsystems via `F1GP_SUBSYSTEMS`.
   - Publishes HLIL outputs + logs.

3. **re-parity.yml**
   - Builds/runs the `parity_harness` binary, validating sample metadata stored at `f1gp-data/dos_traces/sample_boot/metadata.json` against `f1gp-port/tests/data/sample_capture_meta.json`.
   - Future work: replace sample metadata with real telemetry diffs once HLIL + Rust integration lands.

## Secrets & Environment Variables
| Name | Workflow | Purpose |
|------|----------|---------|
| `GHIDRA_URL` (optional) | re-ghidra | Override default download mirror. |
| `BINJA_LICENSE` | re-binja | Binary Ninja headless license contents. |
| `BINJA_DOWNLOAD_URL` | re-binja | Authenticated download for BN. |
| `F1GP_SUBSYSTEMS` | re-binja / refresh script | Comma-separated subsystem filters for HLIL export. |
| `DOSBOX_X` (optional) | re-ghidra (capture step) | Path to DOSBox-X binary if telemetry capture is enabled. |

## Adding to GitHub
1. Navigate to repository Settings → Secrets → Actions.
2. Create repository secrets matching table above.
3. For Binary Ninja, host the headless zip in a private storage bucket; ensure the token embedded in `BINJA_DOWNLOAD_URL` rotates periodically.
4. Optionally set environment variables in workflow dispatch inputs for temporary overrides.

## Future Enhancements
- Replace `curl` downloads with cached GitHub Releases for reproducible builds.
- Add matrix strategy to run both Linux and Windows headless sessions.
- Combine parity workflow with telemetry capture verification once Stage 5 HLIL exports land.
