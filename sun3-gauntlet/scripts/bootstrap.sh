#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VENDOR="$ROOT/vendor/m68k-rs"
FIRMWARE_DIR="$ROOT/firmware"
PROM="$FIRMWARE_DIR/sun3-60-3.0.1.bin"
M68K_COMMIT="ef5dc486bc05410608545da1f83ec49d088af1f1"
PROM_URL="https://oldsilicon.com/technologies/sun-rom-images/ROMS/3.60_v3.0.1.zip"
PROM_SHA1="6e48414ce2139282e69f57612b20f7d5c475e74c"
PROM_SHA256="b562aa5d7bc51eed732fbafde1fd6ea1340977d2b04fb826201c079f699212c6"

mkdir -p "$ROOT/vendor" "$FIRMWARE_DIR"
if [[ ! -f "$VENDOR/.sun3-function-code-patch" ]]; then
    rm -rf "$VENDOR"
    git clone --filter=blob:none https://github.com/benletchford/m68k-rs.git "$VENDOR"
    git -C "$VENDOR" checkout "$M68K_COMMIT"
    rm -rf "$VENDOR/.git"
    python3 "$ROOT/scripts/patch_m68k.py" "$VENDOR"
fi

if [[ ! -f "$PROM" ]]; then
    work="$(mktemp -d)"
    trap 'rm -rf "$work"' EXIT
    curl --fail --location --retry 3 --output "$work/prom.zip" "$PROM_URL"
    unzip -q "$work/prom.zip" -d "$work/prom"
    cp "$work/prom/3.60_v3.0.1_rom" "$PROM"
fi

actual_sha1="$(sha1sum "$PROM" | awk '{print $1}')"
actual_sha256="$(sha256sum "$PROM" | awk '{print $1}')"
[[ "$actual_sha1" == "$PROM_SHA1" ]] || { echo "wrong PROM SHA-1: $actual_sha1" >&2; exit 1; }
[[ "$actual_sha256" == "$PROM_SHA256" ]] || { echo "wrong PROM SHA-256: $actual_sha256" >&2; exit 1; }
[[ "$(stat -c %s "$PROM")" == "65536" ]] || { echo "wrong PROM size" >&2; exit 1; }

cat <<REPORT
m68k-rs commit: $M68K_COMMIT
PROM: $PROM
PROM SHA-1: $actual_sha1
PROM SHA-256: $actual_sha256
REPORT
