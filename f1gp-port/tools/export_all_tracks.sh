#!/bin/bash
#
# Export all 16 F1GP tracks to JSON

set -e

echo "F1GP Track Batch Exporter"
echo "========================="
echo

# Track names (1991 F1 season)
declare -a TRACK_NAMES=(
    "Phoenix"           # F1CT01
    "Interlagos"       # F1CT02
    "Imola"            # F1CT03
    "Monaco"           # F1CT04
    "Montreal"         # F1CT05
    "Mexico"           # F1CT06
    "Magny-Cours"      # F1CT07
    "Silverstone"      # F1CT08
    "Hockenheim"       # F1CT09
    "Hungaroring"      # F1CT10
    "Spa"              # F1CT11
    "Monza"            # F1CT12
    "Estoril"          # F1CT13
    "Barcelona"        # F1CT14
    "Suzuka"           # F1CT15
    "Adelaide"         # F1CT16
)

# Create output directory
OUTPUT_DIR="data/tracks_json"
mkdir -p "$OUTPUT_DIR"

# Build the tool if needed
echo "Building track_inspector..."
cargo build --release -p track_inspector --quiet
echo

# Export each track
for i in {1..16}; do
    TRACK_NUM=$(printf "%02d" $i)
    TRACK_FILE="assets/original/HARDDISK/F1CT${TRACK_NUM}.DAT;1"
    TRACK_NAME="${TRACK_NAMES[$((i-1))]}"
    OUTPUT_FILE="$OUTPUT_DIR/track_${TRACK_NUM}_${TRACK_NAME,,}.json"

    if [ -f "$TRACK_FILE" ]; then
        echo "Exporting F1CT${TRACK_NUM}.DAT → ${TRACK_NAME}..."
        RUST_LOG=warn target/release/track_inspector \
            -i "$TRACK_FILE" \
            -n "$TRACK_NAME" \
            -o "$OUTPUT_FILE"
        echo "  ✓ Written to: $OUTPUT_FILE"
    else
        echo "  ✗ Skipping: $TRACK_FILE not found"
    fi
done

echo
echo "Export complete!"
echo "JSON files written to: $OUTPUT_DIR/"
ls -lh "$OUTPUT_DIR/"
