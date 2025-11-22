#!/bin/bash

# Physics Sandbox - Quick Start Script

echo "======================================"
echo "  Physics Sandbox - Starting Server  "
echo "======================================"
echo ""

# Check if Python 3 is available
if command -v python3 &> /dev/null; then
    echo "Starting local server on http://localhost:8000"
    echo "Press Ctrl+C to stop the server"
    echo ""
    python3 -m http.server 8000
# Check if Python 2 is available
elif command -v python &> /dev/null; then
    echo "Starting local server on http://localhost:8000"
    echo "Press Ctrl+C to stop the server"
    echo ""
    python -m SimpleHTTPServer 8000
# Check if npx is available
elif command -v npx &> /dev/null; then
    echo "Starting local server on http://localhost:8000"
    echo "Press Ctrl+C to stop the server"
    echo ""
    npx http-server -p 8000
else
    echo "ERROR: No suitable HTTP server found!"
    echo "Please install Python or Node.js, or open index.html directly in your browser."
    exit 1
fi
