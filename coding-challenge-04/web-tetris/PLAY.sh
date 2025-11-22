#!/bin/bash
# Simple launcher script for Tetris game

echo "======================================"
echo "  TETRIS - Classic Block Puzzle Game"
echo "======================================"
echo ""
echo "Opening game in your default browser..."
echo ""

# Try to open in browser
if command -v xdg-open > /dev/null; then
    xdg-open index.html
elif command -v open > /dev/null; then
    open index.html
elif command -v start > /dev/null; then
    start index.html
else
    echo "Could not detect browser launcher."
    echo "Please open 'index.html' manually in your browser."
    echo ""
    echo "File location: $(pwd)/index.html"
fi

echo ""
echo "Controls:"
echo "  ← → : Move piece left/right"
echo "  ↑   : Rotate piece"
echo "  ↓   : Soft drop"
echo "  SPACE : Hard drop"
echo "  P   : Pause/Resume"
echo ""
echo "Have fun playing Tetris!"
