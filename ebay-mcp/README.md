# eBay Search MCP Server

An MCP (Model Context Protocol) server that enables AI assistants to search eBay using headless browser automation. Built in Rust for performance and safety.

## Features

- 🔍 **Intelligent eBay Search**: Automated searching with comprehensive filtering
- 💾 **Saved Search Phrases**: Store and reuse frequent searches
- ⚡ **Smart Caching**: Results caching with configurable TTL
- 🤖 **Anti-Detection**: User agent rotation, random delays, fingerprint masking
- 📊 **Search History**: Track all searches with statistics
- 🎯 **MCP Compliant**: Full Model Context Protocol implementation

## Status

**Current Phase**: Implementation - Phase 1 (Foundation)
**Progress**: ~70% of Phase 1 complete

### Completed
- ✅ Project structure and configuration
- ✅ Error types and data models
- ✅ Configuration management (TOML)
- ✅ Database layer (SQLite)
- ✅ Cache system
- ✅ Logging infrastructure

### In Progress
- 🔨 Initial build and testing

### Upcoming
- Browser pool implementation
- eBay scraper with DOM parsing
- Search manager orchestration
- MCP protocol layer

## Prerequisites

### Rust

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Rust 1.75+ required
rustc --version
```

## Quick Start

```bash
# Clone repository
cd ebay-mcp

# Build the project
cargo build --release

# Run the server
cargo run -- --config config/config.toml

# Or with custom log level
cargo run -- --log-level debug
```

## Configuration

Configuration is managed through TOML files:

### Main Configuration (`config/config.toml`)

- Server settings
- Browser pool configuration
- Database and cache settings
- Logging preferences
- Scraper options

### Saved Phrases (`config/search_phrases.toml`)

- Saved search queries
- Filter presets
- Tag-based organization

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              eBay Search MCP Server (Rust)                   │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │              MCP Protocol Layer                     │    │
│  └────────────────────────────────────────────────────┘    │
│  ┌────────────────────────────────────────────────────┐    │
│  │              Search Management Layer                │    │
│  └────────────────────────────────────────────────────┘    │
│  ┌────────────────────────────────────────────────────┐    │
│  │           Headless Browser Layer                    │    │
│  └────────────────────────────────────────────────────┘    │
│  ┌────────────────────────────────────────────────────┐    │
│  │              Data Persistence Layer                 │    │
│  └────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Project Structure

```
ebay-mcp-server/
├── src/
│   ├── server/       # MCP protocol implementation
│   ├── search/       # Search management
│   ├── browser/      # Headless browser pool
│   ├── scraper/      # eBay scraping logic
│   ├── storage/      # Database and cache
│   ├── config/       # Configuration management
│   ├── models/       # Data models
│   └── utils/        # Utility functions
├── config/           # Configuration files
├── wrk_docs/         # Design documents (HLD)
├── wrk_journals/     # Implementation journal
└── tests/            # Integration tests
```

## Development

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Run with logs

```bash
RUST_LOG=debug cargo run
```

### Documentation

```bash
cargo doc --open
```

## MCP Tools

The server will provide these MCP tools (when complete):

- `search_ebay` - Execute eBay search
- `search_by_phrase` - Use saved search phrase
- `save_search_phrase` - Save new phrase
- `list_search_phrases` - View all saved phrases
- `update_search_phrase` - Modify phrase
- `delete_search_phrase` - Remove phrase
- `get_search_history` - View search history
- `clear_cache` - Cache management

## MCP Resources

- `ebay://config` - Server configuration
- `ebay://phrases` - All saved phrases
- `ebay://phrases/{id}` - Specific phrase
- `ebay://history` - Search history
- `ebay://stats` - Server statistics

## Documentation

- [High-Level Design](wrk_docs/2025.11.13%20-%20HLD%20-%20eBay%20Search%20MCP%20Server.md)
- [Implementation Journal](wrk_journals/2025.11.13%20-%20JRN%20-%20eBay%20MCP%20Implementation.md)
- [MCP Guide](mcp-server-comprehensive-guide.md)

## License

MIT

## Contributing

This project is under active development. See the implementation journal for current status and next steps.
