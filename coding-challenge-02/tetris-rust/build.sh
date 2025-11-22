#!/bin/bash
# Build and test script for Tetris Champion

set -e

echo "================================================"
echo "  Tetris Champion - Build & Test Script"
echo "================================================"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "[1/4] Running tests..."
cargo test --quiet
echo "✓ All tests passed!"
echo ""

echo "[2/4] Building debug version..."
cargo build --quiet
echo "✓ Debug build completed!"
echo ""

echo "[3/4] Building release version..."
cargo build --release --quiet
echo "✓ Release build completed!"
echo ""

echo "[4/4] Verifying binary..."
if [ -f "target/release/tetris-rust" ]; then
    SIZE=$(du -h target/release/tetris-rust | cut -f1)
    echo "✓ Binary created: target/release/tetris-rust ($SIZE)"
else
    echo "✗ Binary not found!"
    exit 1
fi

echo ""
echo "================================================"
echo "  Build Complete!"
echo "================================================"
echo ""
echo "To run the game:"
echo "  ./target/release/tetris-rust"
echo "  or"
echo "  cargo run --release"
echo ""
echo "Have fun playing Tetris! 🎮"
