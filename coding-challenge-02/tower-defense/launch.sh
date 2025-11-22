#!/bin/bash

# Tower Defense Game Launcher
# Simple HTTP server to run the game

echo "🏰 Tower Defense: Epic Battle"
echo "=============================="
echo ""
echo "Starting local web server..."
echo ""

# Check if Python 3 is available
if command -v python3 &> /dev/null; then
    echo "✅ Server running at: http://localhost:8000"
    echo "✅ Open your browser and navigate to the URL above"
    echo ""
    echo "Press Ctrl+C to stop the server"
    echo ""
    python3 -m http.server 8000
elif command -v python &> /dev/null; then
    echo "✅ Server running at: http://localhost:8000"
    echo "✅ Open your browser and navigate to the URL above"
    echo ""
    echo "Press Ctrl+C to stop the server"
    echo ""
    python -m SimpleHTTPServer 8000
else
    echo "❌ Python not found. Please install Python or open index.html directly in your browser."
    exit 1
fi
