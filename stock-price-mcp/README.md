# Stock Price MCP Server

A Model Context Protocol (MCP) server for fetching real-time stock prices from Yahoo Finance. Built in Rust with a clean, simple interface.

## Overview

This MCP server provides tools for AI assistants to fetch stock market data from Yahoo Finance. It implements the MCP protocol over STDIO transport using JSON-RPC 2.0 for communication.

### Features

- **Real-time Stock Prices**: Fetch current stock prices for any ticker symbol
- **Detailed Stock Information**: Get comprehensive stock data including market cap, day range, and more
- **MCP Protocol Compliant**: Fully implements the MCP 2025-03-26 specification
- **Web Scraping**: Uses HTTP scraping with robust selectors for reliable data extraction
- **Simple Interface**: Clean, easy-to-understand Rust implementation

## Available Tools

### 1. `get_stock_price`
Fetches the current stock price from Yahoo Finance for a given ticker symbol.

**Parameters:**
- `symbol` (string, required): Stock ticker symbol (e.g., AAPL, GOOGL, MSFT, TSLA)

**Example Response:**
```
Stock AAPL current price: $182.45
```

### 2. `get_stock_info`
Fetches detailed stock information including price, market cap, day range, and volume.

**Parameters:**
- `symbol` (string, required): Stock ticker symbol (e.g., AAPL, GOOGL, MSFT, TSLA)

**Example Response:**
```
Stock Information for AAPL:
Price: $182.45
Previous Close: $181.20
Open: $181.50
Day Range: $180.75 - $183.20
Market Cap: $2.85T
```

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- Internet connection (for fetching stock data)

### Build from Source

```bash
cd stock-price-mcp
cargo build --release
```

The compiled binary will be available at `target/release/stock-price-mcp`.

## Usage

### Running as a Standalone Server

```bash
cargo run
```

The server will start and listen for JSON-RPC messages on stdin, sending responses to stdout and logs to stderr.

### Testing with Manual JSON-RPC Requests

You can test the server by sending JSON-RPC requests via stdin:

#### 1. Initialize the server

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26"}}' | cargo run
```

#### 2. List available tools

```bash
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | cargo run
```

#### 3. Get stock price

```bash
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_stock_price","arguments":{"symbol":"AAPL"}}}' | cargo run
```

#### 4. Get detailed stock info

```bash
echo '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"get_stock_info","arguments":{"symbol":"TSLA"}}}' | cargo run
```

### Using with MCP Clients

#### Claude Desktop Configuration

Add to your Claude Desktop config file (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "stock-price": {
      "command": "/path/to/stock-price-mcp/target/release/stock-price-mcp"
    }
  }
}
```

#### Using with Other MCP Clients

Any MCP-compatible client can use this server by launching it as a subprocess and communicating via STDIO.

## Architecture

### Protocol Implementation

The server implements the MCP protocol specification (version 2025-03-26) with support for:

- **Transport**: STDIO (standard input/output)
- **Protocol**: JSON-RPC 2.0
- **Methods**:
  - `initialize`: Initialize the server connection
  - `tools/list`: List available tools
  - `tools/call`: Execute a specific tool

### Data Fetching Strategy

The server uses HTTP scraping with the following approach:

1. **HTTP Client**: Uses `reqwest` with a browser-like User-Agent
2. **HTML Parsing**: Uses `scraper` library for robust HTML parsing
3. **Multiple Selectors**: Tries several CSS selectors for resilience against page changes
4. **Error Handling**: Provides clear error messages when data cannot be fetched

### Code Structure

```
src/
└── main.rs
    ├── JsonRpcRequest/Response structs
    ├── McpServer implementation
    │   ├── handle_initialize()
    │   ├── handle_tools_list()
    │   ├── handle_tools_call()
    │   └── start()
    ├── fetch_stock_price()
    ├── fetch_stock_info()
    └── format_market_cap()
```

## Dependencies

- **tokio**: Async runtime for handling asynchronous operations
- **serde/serde_json**: JSON serialization and deserialization
- **anyhow**: Error handling
- **reqwest**: HTTP client for fetching web pages
- **scraper**: HTML parsing and CSS selector support
- **thirtyfour**: (Optional) WebDriver protocol for headless browser automation

## Development

### Building for Development

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Checking Code

```bash
cargo clippy
cargo fmt --check
```

## Troubleshooting

### Stock Price Not Found

If the server returns "Could not find stock price for symbol", try:

1. Verify the ticker symbol is correct (e.g., AAPL, not Apple)
2. Check that Yahoo Finance has data for that symbol
3. Check your internet connection
4. Yahoo Finance may have changed their HTML structure (selectors may need updating)

### Connection Issues

If the server doesn't respond:

1. Check that the server is running (check stderr for startup messages)
2. Verify JSON-RPC messages are properly formatted
3. Ensure each message is on a single line (no embedded newlines)

### Build Errors

If you encounter build errors:

```bash
# Clean the build
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build
```

## MCP Protocol Details

This server follows the MCP protocol specification. For detailed protocol documentation, see `MCP_IMPLEMENTATION_GUIDE.md`.

### JSON-RPC Message Format

All messages follow JSON-RPC 2.0 specification:

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "get_stock_price",
    "arguments": {
      "symbol": "AAPL"
    }
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [{
      "type": "text",
      "text": "Stock AAPL current price: $182.45"
    }]
  }
}
```

## Security Considerations

- The server only makes HTTP GET requests to Yahoo Finance
- No sensitive data is stored or transmitted
- Input validation is performed on all parameters
- User-Agent header is set to identify the client properly

## Performance

- Async I/O for efficient request handling
- Lightweight HTTP scraping (no headless browser overhead for basic requests)
- Minimal memory footprint
- Fast startup time

## Limitations

- Relies on Yahoo Finance HTML structure (may break if they change their website)
- No caching (each request fetches fresh data)
- Rate limiting may apply from Yahoo Finance
- Only supports publicly traded stocks available on Yahoo Finance

## Future Enhancements

Potential improvements:

- [ ] Add caching for frequently requested symbols
- [ ] Support for historical price data
- [ ] Support for multiple stock symbols in a single request
- [ ] Add support for cryptocurrency prices
- [ ] Implement rate limiting protection
- [ ] Add configuration file support
- [ ] Full headless browser option for JavaScript-rendered pages

## Contributing

This is a simple, standalone MCP server. Feel free to:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - see LICENSE file for details

## Resources

- [MCP Specification](https://spec.modelcontextprotocol.io)
- [MCP Documentation](https://modelcontextprotocol.io)
- [Yahoo Finance](https://finance.yahoo.com)
- [Rust MCP SDK Documentation](https://docs.rs/mcpr)

## Version History

### v1.0.0 (2025-01-12)
- Initial release
- Basic stock price fetching
- Detailed stock information
- MCP 2025-03-26 protocol support
- STDIO transport implementation

---

**Note**: This server fetches data from Yahoo Finance through web scraping. Yahoo Finance's terms of service should be reviewed for commercial use. This tool is intended for personal, educational, and development purposes.
