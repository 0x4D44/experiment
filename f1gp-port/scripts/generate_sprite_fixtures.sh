#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT_DIR"

DEST=${1:-data/fixtures/sprites}

cargo run -p sprite_atlas_cli --quiet -- generate-fixtures --dest "$DEST"

cargo run -p sprite_atlas_cli --quiet -- pack \
  --source "$DEST" \
  --dest "$DEST" \
  --width 256 --height 256

echo "Sprite fixtures generated under $DEST"
