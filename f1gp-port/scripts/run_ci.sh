#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT_DIR"

log() {
  printf '\n==> %s\n' "$1"
}

log "Ensuring sanitized fixtures"
cargo run --quiet --bin generate_fixtures

log "Generating telemetry fixture CSV"
./scripts/generate_fixture_csvs.sh >/dev/null

log "Generating sprite fixtures"
./scripts/generate_sprite_fixtures.sh >/dev/null

log "Verifying telemetry diff tooling"
cargo run -p telemetry_cli --quiet -- diff \
  --reference data/fixtures/telemetry/synthetic_monaco.csv \
  --candidate data/fixtures/telemetry/synthetic_monaco.csv \
  --car 0

log "Checking formatting"
cargo fmt --all -- --check

log "Running Clippy (warnings are errors)"
cargo clippy --workspace --all-targets -- -D warnings

log "Running tests (workspace)"
cargo test --workspace

log "CI checks completed"
