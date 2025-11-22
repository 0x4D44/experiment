#!/usr/bin/env bash
set -euo pipefail

echo "Generating fixtures via Rust binary..."
cargo run --quiet --bin generate_fixtures

ls -lh data/fixtures
