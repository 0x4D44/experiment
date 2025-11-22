#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT_DIR"

cargo run --bin generate_fixtures >/dev/null

mkdir -p data/fixtures/telemetry

cargo run -p telemetry_cli -- export-csv \
  --input data/fixtures/telemetry/synthetic_monaco.bin \
  --output data/fixtures/telemetry/synthetic_monaco.csv >/dev/null

echo "Fixture telemetry CSV written to data/fixtures/telemetry/synthetic_monaco.csv"
