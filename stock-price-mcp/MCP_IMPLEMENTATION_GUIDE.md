# Comprehensive MCP (Model Context Protocol) Implementation Guide

## Table of Contents
1. [Overview](#overview)
2. [Core Concepts](#core-concepts)
3. [Protocol Specification](#protocol-specification)
4. [Implementation in Rust](#implementation-in-rust)
5. [Best Practices](#best-practices)
6. [Example Implementations](#example-implementations)

---

## Overview

### What is MCP?

The Model Context Protocol (MCP) is an open-source standard for connecting AI applications to external systems. Think of MCP like a USB-C port for AI applications—providing a universal interface for bidirectional communication between AI systems and various data sources.

**Protocol Revision:** 2025-03-26 (latest)

### Key Benefits

- **Dynamic Discovery:** Runtime tool discovery reduces hallucinations through structured validation
- **Unified Security:** OAuth 2.1 implementation for consistent authentication
- **Modularity:** Single interface accessing diverse resources (filesystems, databases, APIs)
- **Multi-turn Interactions:** Supports conversational agent workflows with real-time context fetching
- **Efficiency:** Reduces token usage by up to 98.7% compared to traditional tool-calling approaches

### Stakeholder Benefits

- **Developers:** Streamlines building and integrating AI applications
- **AI Systems:** Access to expanded data and tool ecosystems
- **End-Users:** More capable AI assistants that can act on their behalf

---

## Core Concepts

### Architecture

MCP follows a client-server architecture:

```
┌─────────────┐         MCP Protocol          ┌─────────────┐
│             │◄────────────────────────────►  │             │
│  MCP Client │    (JSON-RPC over Transport)   │  MCP Server │
│             │                                │             │
└─────────────┘                                └─────────────┘
      │                                              │
      │                                              │
      ▼                                              ▼
  AI Application                            External Resources
  (Claude, etc.)                           (Data, Tools, APIs)
```

### Key Components

1. **Transport Layer:** Communication mechanism (STDIO, HTTP/2, WebSocket, SSE)
2. **Protocol Layer:** JSON-RPC 2.0 message format
3. **Server Configuration:** Tool definitions, capabilities, metadata
4. **Tool Handlers:** Functions that process tool invocations
5. **Security Layer:** Authentication and authorization

### MCP vs. OpenAPI

| Feature | OpenAPI | MCP |
|---------|---------|-----|
| API Definition | Static REST contracts | Dynamic tool discovery |
| Validation | Client-side (model generates calls) | Server-side (validates before execution) |
| Context | Stateless | Supports stateful conversations |
| Security | Various patterns | Standardized OAuth 2.1 |

---

## Protocol Specification

### Message Format

MCP uses **JSON-RPC 2.0** for all messages:

- All messages MUST be UTF-8 encoded
- Messages MUST follow JSON-RPC 2.0 specification
- Messages can be requests, responses, notifications, or batches

### Transport Mechanisms

#### STDIO Transport (Primary for Server Subprocesses)

**How it works:**
1. Client launches MCP server as a subprocess
2. Server reads JSON-RPC messages from stdin
3. Server writes responses to stdout
4. Server MAY write logs to stderr

**Message Delimiting:**
- Messages are delimited by newlines
- Messages MUST NOT contain embedded newlines
- Each line is a complete JSON-RPC message

**Example Message Flow:**
```
Client → Server (stdin):
{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}

Server → Client (stdout):
{"jsonrpc":"2.0","id":1,"result":{"tools":[...]}}
```

#### Other Transports

- **HTTP/1.1 & HTTP/2:** RESTful or long-lived connections
- **WebSocket:** Bidirectional real-time communication
- **SSE (Server-Sent Events):** Unidirectional streaming

### Core Protocol Methods

#### Server Information
```json
// Request
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26"}}

// Response
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-03-26","serverInfo":{"name":"my-server","version":"1.0.0"}}}
```

#### List Tools
```json
// Request
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}

// Response
{
  "jsonrpc":"2.0",
  "id":2,
  "result":{
    "tools":[
      {
        "name":"get_stock_price",
        "description":"Fetches current stock price from Yahoo Finance",
        "inputSchema":{
          "type":"object",
          "properties":{
            "symbol":{"type":"string","description":"Stock ticker symbol"}
          },
          "required":["symbol"]
        }
      }
    ]
  }
}
```

#### Call Tool
```json
// Request
{
  "jsonrpc":"2.0",
  "id":3,
  "method":"tools/call",
  "params":{
    "name":"get_stock_price",
    "arguments":{"symbol":"AAPL"}
  }
}

// Response
{
  "jsonrpc":"2.0",
  "id":3,
  "result":{
    "content":[
      {
        "type":"text",
        "text":"Apple Inc. (AAPL): $182.45"
      }
    ]
  }
}
```

---

## Implementation in Rust

### Available Rust SDKs

1. **mcpr** (Official Rust MCP SDK)
   - Crate: `mcpr` (v0.2.3+)
   - Documentation: docs.rs/mcpr
   - Features: Schema definitions, transport layer, server/client implementations
   - Coverage: 65.8% documented

2. **prism-mcp-rs** (Enterprise-Grade)
   - Production-ready with advanced features
   - Circuit breakers, adaptive retry policies
   - Hot-swappable plugin support
   - OpenTelemetry integration

### Basic Server Implementation with mcpr

#### 1. Project Setup

```toml
# Cargo.toml
[package]
name = "stock-price-mcp"
version = "0.1.0"
edition = "2021"

[dependencies]
mcpr = "0.2"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
```

#### 2. Server Structure

```rust
use mcpr::{Server, ServerConfig, StdioTransport};
use serde_json::{json, Value};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Configure server
    let server_config = ServerConfig::new()
        .with_name("Stock Price MCP Server")
        .with_version("1.0.0")
        .with_tool(create_tool_definition());

    // 2. Create server with STDIO transport
    let mut server: Server<StdioTransport> = Server::new(server_config);

    // 3. Register tool handler
    server.register_tool_handler("get_stock_price", handle_stock_price)?;

    // 4. Start server
    server.start().await?;

    Ok(())
}

fn create_tool_definition() -> Value {
    json!({
        "name": "get_stock_price",
        "description": "Fetches current stock price from Yahoo Finance",
        "inputSchema": {
            "type": "object",
            "properties": {
                "symbol": {
                    "type": "string",
                    "description": "Stock ticker symbol (e.g., AAPL, GOOGL)"
                }
            },
            "required": ["symbol"]
        }
    })
}

async fn handle_stock_price(params: Value) -> Result<Value> {
    let symbol = params["symbol"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing symbol parameter"))?;

    // Implement stock price fetching logic here
    let price = fetch_stock_price(symbol).await?;

    Ok(json!({
        "content": [{
            "type": "text",
            "text": format!("Stock {} price: ${}", symbol, price)
        }]
    }))
}
```

#### 3. Tool Handler Best Practices

```rust
// Input validation
fn validate_params(params: &Value) -> Result<()> {
    if !params["symbol"].is_string() {
        return Err(anyhow::anyhow!("symbol must be a string"));
    }
    Ok(())
}

// Error handling
async fn handle_tool_with_error(params: Value) -> Result<Value> {
    match process_request(params).await {
        Ok(result) => Ok(json!({
            "content": [{"type": "text", "text": result}]
        })),
        Err(e) => Ok(json!({
            "content": [{
                "type": "text",
                "text": format!("Error: {}", e)
            }],
            "isError": true
        }))
    }
}

// Structured response
fn create_response(data: Vec<(String, String)>) -> Value {
    json!({
        "content": [{
            "type": "text",
            "text": serde_json::to_string_pretty(&data).unwrap()
        }]
    })
}
```

---

## Best Practices

### 1. Security

- **Authentication:** Implement OAuth 2.1 for production servers
- **Authorization:** Enforce granular permissions per tool
- **Sandboxing:** Restrict filesystem and network access
- **Input Validation:** Always validate and sanitize user inputs
- **Rate Limiting:** Prevent abuse through request throttling

### 2. Performance

- **Connection Pooling:** Reuse HTTP clients and database connections
- **Caching:** Cache expensive operations (API calls, database queries)
- **Async Operations:** Use async/await for I/O-bound operations
- **Resource Limits:** Set timeouts and memory limits
- **Batch Operations:** Support batch requests where applicable

### 3. Observability

- **Structured Logging:** Use correlation IDs to trace requests
- **Metrics:** Track request counts, latencies, error rates
- **Health Checks:** Implement health endpoints for monitoring
- **Error Forensics:** Log detailed error context for debugging
- **Distributed Tracing:** Use OpenTelemetry for multi-service architectures

### 4. Reliability

- **Circuit Breakers:** Prevent cascading failures
- **Retry Logic:** Implement exponential backoff with jitter
- **Graceful Degradation:** Provide fallback responses
- **Idempotency:** Design tools to be safely retryable
- **Health Monitoring:** Multi-level health checks (shallow, deep)

### 5. Developer Experience

- **Clear Documentation:** Document each tool's purpose and parameters
- **Type Safety:** Use strong typing for input/output schemas
- **Error Messages:** Provide actionable error messages
- **Examples:** Include usage examples in tool descriptions
- **Versioning:** Version your server and tools appropriately

---

## Example Implementations

### Example 1: Simple Calculator Server

```rust
use mcpr::{Server, ServerConfig, StdioTransport};
use serde_json::{json, Value};
use anyhow::Result;

fn create_calculator_tools() -> Vec<Value> {
    vec![
        json!({
            "name": "add",
            "description": "Adds two numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number"},
                    "b": {"type": "number"}
                },
                "required": ["a", "b"]
            }
        }),
        json!({
            "name": "multiply",
            "description": "Multiplies two numbers",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number"},
                    "b": {"type": "number"}
                },
                "required": ["a", "b"]
            }
        })
    ]
}

async fn handle_add(params: Value) -> Result<Value> {
    let a = params["a"].as_f64().unwrap_or(0.0);
    let b = params["b"].as_f64().unwrap_or(0.0);
    Ok(json!({
        "content": [{"type": "text", "text": (a + b).to_string()}]
    }))
}

async fn handle_multiply(params: Value) -> Result<Value> {
    let a = params["a"].as_f64().unwrap_or(0.0);
    let b = params["b"].as_f64().unwrap_or(0.0);
    Ok(json!({
        "content": [{"type": "text", "text": (a * b).to_string()}]
    }))
}
```

### Example 2: Web Scraping Server

```rust
async fn handle_fetch_webpage(params: Value) -> Result<Value> {
    let url = params["url"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing url parameter"))?;

    // Use headless browser (e.g., headless_chrome)
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    tab.navigate_to(url)?
        .wait_until_navigated()?;

    let content = tab.get_content()?;

    Ok(json!({
        "content": [{
            "type": "text",
            "text": content
        }]
    }))
}
```

### Example 3: Database Query Server

```rust
async fn handle_query(params: Value) -> Result<Value> {
    let query = params["query"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing query parameter"))?;

    // Validate query (e.g., only allow SELECT)
    if !query.trim_start().to_uppercase().starts_with("SELECT") {
        return Err(anyhow::anyhow!("Only SELECT queries are allowed"));
    }

    let pool = get_connection_pool();
    let rows = sqlx::query(query)
        .fetch_all(&pool)
        .await?;

    let result = format_rows(rows)?;

    Ok(json!({
        "content": [{
            "type": "text",
            "text": result
        }]
    }))
}
```

---

## Resources

### Official Documentation
- MCP Main Site: https://modelcontextprotocol.io
- MCP Specification: https://spec.modelcontextprotocol.io
- GitHub Organization: https://github.com/modelcontextprotocol

### Rust-Specific Resources
- mcpr Documentation: https://docs.rs/mcpr
- prism-mcp-rs: https://github.com/prismworks-ai/prism-mcp-rs
- mcp-client-rs: https://github.com/tim-schultz/mcp-client-rs

### Community Resources
- MCP Servers Collection: GitHub modelcontextprotocol/servers (72.5k stars)
- Developer Guides: Zep.ai, Medium articles, engineering blogs
- Integration Examples: Claude Desktop, LangChain, OpenAI Agents SDK

### Key Articles
- "Code execution with MCP: Building more efficient AI agents" - Anthropic Engineering
- "A Developer's Guide to the MCP" - Zep Software
- "Building Better Rust Code with AI: Introducing the Rust MCP Server" - Medium

---

## Appendix: Protocol Revision History

- **2025-03-26:** Latest specification revision
- Includes updates to transport mechanisms, message formats, and security requirements
- Backward compatibility considerations documented in specification

---

## Quick Reference

### Common JSON-RPC Methods
- `initialize` - Initialize connection and protocol version
- `tools/list` - List available tools
- `tools/call` - Invoke a specific tool
- `notifications/initialized` - Notification after successful init

### HTTP Status Codes (HTTP Transport)
- 200 - Success
- 400 - Bad Request (invalid JSON-RPC)
- 401 - Unauthorized
- 500 - Internal Server Error

### Error Codes (JSON-RPC)
- `-32700` - Parse error
- `-32600` - Invalid Request
- `-32601` - Method not found
- `-32602` - Invalid params
- `-32603` - Internal error

---

*Last Updated: 2025-01-12*
*Protocol Version: 2025-03-26*
