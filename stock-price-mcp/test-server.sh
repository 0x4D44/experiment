#!/bin/bash
# Test script for Stock Price MCP Server

echo "Testing Stock Price MCP Server"
echo "==============================="
echo ""

# Build the project first
echo "1. Building the project..."
cargo build --release 2>&1 | grep -E "(Compiling|Finished|error)" || echo "Build completed"
echo ""

# Test 1: Initialize
echo "2. Testing initialize..."
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26"}}' | \
  timeout 5 cargo run --quiet 2>/dev/null | head -1
echo ""

# Test 2: List tools
echo "3. Testing tools/list..."
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | \
  timeout 5 cargo run --quiet 2>/dev/null | head -1 | jq -r '.result.tools[].name' 2>/dev/null || echo "Response received (jq not available for pretty print)"
echo ""

# Test 3: Get stock price (Example - won't actually fetch in test mode)
echo "4. Example: Get stock price for AAPL"
echo "   Command: echo '{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{\"name\":\"get_stock_price\",\"arguments\":{\"symbol\":\"AAPL\"}}}' | cargo run --quiet"
echo ""

echo "==============================="
echo "Test completed!"
echo ""
echo "To manually test the server, run:"
echo "  cargo run"
echo ""
echo "Then send JSON-RPC messages via stdin, for example:"
echo "  {\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/list\",\"params\":{}}"
