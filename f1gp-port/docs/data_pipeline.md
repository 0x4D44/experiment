# Data & Telemetry Pipeline

_Last updated: 2025-11-19_

This document captures the end-to-end workflow for working with original F1GP assets, the sanitized fixtures we
ship in-repo, the driver database, and the telemetry streams that power the parity program. Follow these steps before
running tooling, CI, or parity validations.

## 1. Track Asset Extraction & Verification

1. Extract the original DOS game files (e.g., by mounting the CD/ISO and copying `HARDDISK/` onto your workstation).
2. Run the Rust asset extractor to copy the required `F1CTxx.DAT` files and verify their checksum footer:

   ```bash
   cargo run -p asset_extractor -- \
     --source /path/to/HARDDISK \
     --dest   assets/original/tracks \
     --force
   ```

   - `--dry-run` lets you validate checksums without copying.
   - On success the tool writes `asset_manifest.json` with file sizes, stored/computed checksums,
     and validation status for all 16 tracks. Persist this JSON with the extracted assets so CI/preflight scripts
     can assert we are using the expected revision.
   - Run `cargo run -p asset_extractor -- --help` for the full CLI reference; the crate's unit tests
     (`tools/asset_extractor/src/main.rs`) capture expected behavior.

3. Commit only the manifest; never add the proprietary DAT files to the repo.
4. For AI/racing-line research, export structured metadata via:
   ```bash
   cargo run -p racing_line_cli -- export --inputs assets/original/F1CT01.DAT --output exports/tracks/ --pretty
   ```
   This writes JSON describing sections, flags, and racing-line segments for downstream tools.

## 2. Sanitized Fixtures & CI Hook

We ship lightweight fixtures so parser/loader tests can run without the proprietary tracks.

- Generate/update fixtures via either command below. Both commands create `data/fixtures/track_stub.bin` and copy the
  sanitized `driver_db.json` sample:

  ```bash
  cargo run --quiet --bin generate_fixtures
  ./scripts/build_fixtures.sh            # wraps the same binary and prints file sizes
  ```

- Wire `scripts/build_fixtures.sh` into CI before `cargo test` so tests always see the fixture outputs.
- `scripts/run_ci.sh` already calls `cargo run --bin generate_fixtures` before formatting/clippy/tests; use it locally and in CI to keep the workflow consistent.
- Tests/tools should reference `data/fixtures/*` to remain legally clean. The fixtures are git-ignored—regenerate them
  locally whenever parser code changes.

## 3. Driver / Car Database

- Authoritative data lives in `data/samples/driver_db.json` (sanitized 1991 roster). The runtime now loads this file by
  default via `CarDatabase::load_from_disk()`.
- Override the path with the `F1GP_DRIVER_DB_PATH` environment variable if you maintain external variants, e.g.:

  ```bash
  export F1GP_DRIVER_DB_PATH=$HOME/f1gp-data/driver_db.json
  ```

- The game logs whether it loaded from disk or fell back to the baked-in sample, making provenance explicit in QA logs.
- To publish updated rosters, run `cargo run --bin generate_fixtures` so the sanitized copy under `data/fixtures` stays in sync.

## 4. Audio & UI Asset Conversion

- Run `cargo run -p audio_ui_extractor -- pcm --source <HARDDISK/SAMPLES> --dest build/audio` to convert the raw PCM
  blobs into standard WAV files. The tool writes `pcm_manifest.json` with byte counts, hashes, and configuration so we
  can track provenance without checking the PCM payloads into git.
- Use the `--extensions` flag to cover alternate suffixes (e.g., `--extensions pcm,raw`). The converter assumes unsigned
  8-bit mono by default; override `--sample-rate`/`--channels` per bank if needed.
- Convert VGA font binaries via `cargo run -p audio_ui_extractor -- font --input FONT.BIN --glyph-width 8 --glyph-height 16 \
  --glyph-count 256 --dest build/fonts`. This produces `<name>_atlas.png` plus a manifest referencing hashes and glyph
  layout. Pass `--include-metadata` to embed glyph placements for runtime loaders.
- Store the generated WAV/PNG outputs outside the repo (e.g., `assets/original/audio/`) and keep only the manifests in
  version control so QA can confirm we are using the right revisions.

## 5. Telemetry Capture

- Telemetry recording now starts automatically whenever a track is loaded (practice or race). Samples are captured every
  frame for the player plus all AI cars using the `telemetry::TelemetryRecording` schema.
- Recordings are written to `telemetry/<track>-<session>-<timestamp>.bin` on disk when a race ends, when a new track is
  loaded, or when the game shuts down. Files are bincode-encoded and ready for the parity harness.
- Set `F1GP_TELEMETRY=off` (or `0`/`false`) to disable capture for lightweight development sessions. Leave it unset in
  parity/CI environments so we always have fresh data.
- Telemetry folders are ignored by git; archive interesting captures manually if needed.
- Use `cargo run -p telemetry_cli -- summary --input telemetry/<file>.bin --verbose` to inspect capture metadata,
  `export-json`/`export-csv` to feed downstream analysis, or `diff --reference <dos.csv> --candidate <rust.csv>` to get
  baseline parity metrics. Schema details live in `wrk_docs/2025.11.19 - Telemetry Schema Spec.md`.

## 6. DOS Capture Preparation

We rely on DOSBox/PCem to capture golden telemetry/visual references from the original game.

1. Install **DOSBox Staging 0.80+** (preferred for deterministic cycles) or PCem 17 if you need cycle-perfect behavior.
2. Prepare a clean F1GP installation inside the emulator and copy the exact tracks you extracted earlier.
3. Use the following baseline scenarios for parity runs:
   - Phoenix, Monaco, Monza – dry conditions, default car setup.
   - Montreal, Silverstone – wet conditions to benchmark rain logic once implemented.
4. Capture gameplay footage plus serial telemetry logs:
   - Configure DOSBox `serial1=raw file dos_capture.log` and run the included `capture.com` helper (see `docs/track_binary_format.md`).
   - Keep a manifest of ISO hash, DOSBox commit, and config used for every capture.
5. Store the resulting logs outside the repo. Use `cargo run -p dos_capture_parser -- export-csv --input captures/<track>/dos_capture.log --output captures/<track>/dos.csv`
   to convert the serial output into CSV, then diff against the Rust capture via `telemetry_cli diff`. See
   `docs/dos_capture_playbook.md` for a step-by-step walkthrough and manifest checklist.

## 7. Sprite Pipeline (Phase 2 Preview)

- Generate sanitized placeholder sprites with `./scripts/generate_sprite_fixtures.sh` (wraps the CLI commands).
- Pack actual sprites (once legally extracted) with `cargo run -p sprite_atlas_cli -- pack --source assets/original/sprites --dest build/sprites`.
- The tool outputs `sprite_atlas.png` plus `sprite_manifest.json` documenting sprite UVs for runtime loaders.

## 7. Next Steps

- Audio/UI extractor tooling (task 1.5) will live beside `tools/asset_extractor`; it should reuse the manifest pattern
  above so PCM/fonts share the same provenance trail.
- Update this document whenever the pipeline changes so downstream teams (rendering, physics, QA) can rely on a single
  source of truth.
