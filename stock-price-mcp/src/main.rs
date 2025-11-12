use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use tokio::runtime::Runtime;

/// JSON-RPC 2.0 Request structure
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

/// JSON-RPC 2.0 Response structure
#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

/// JSON-RPC 2.0 Error structure
#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

/// MCP Server implementation
struct McpServer {
    name: String,
    version: String,
    protocol_version: String,
}

impl McpServer {
    fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            protocol_version: "2025-03-26".to_string(),
        }
    }

    /// Handle incoming JSON-RPC requests
    fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let result = match request.method.as_str() {
            "initialize" => self.handle_initialize(request.params),
            "tools/list" => self.handle_tools_list(),
            "tools/call" => self.handle_tools_call(request.params),
            _ => Err(anyhow!("Method not found: {}", request.method)),
        };

        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(value),
                error: None,
            },
            Err(e) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32603,
                    message: e.to_string(),
                    data: None,
                }),
            },
        }
    }

    /// Handle initialize method
    fn handle_initialize(&self, _params: Option<Value>) -> Result<Value> {
        Ok(json!({
            "protocolVersion": self.protocol_version,
            "serverInfo": {
                "name": self.name,
                "version": self.version
            },
            "capabilities": {
                "tools": {}
            }
        }))
    }

    /// Handle tools/list method
    fn handle_tools_list(&self) -> Result<Value> {
        Ok(json!({
            "tools": [
                {
                    "name": "get_stock_price",
                    "description": "Fetches the current stock price from Yahoo Finance for a given ticker symbol",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "symbol": {
                                "type": "string",
                                "description": "Stock ticker symbol (e.g., AAPL, GOOGL, MSFT, TSLA)"
                            }
                        },
                        "required": ["symbol"]
                    }
                },
                {
                    "name": "get_stock_info",
                    "description": "Fetches detailed stock information including price, market cap, and volume",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "symbol": {
                                "type": "string",
                                "description": "Stock ticker symbol (e.g., AAPL, GOOGL, MSFT, TSLA)"
                            }
                        },
                        "required": ["symbol"]
                    }
                }
            ]
        }))
    }

    /// Handle tools/call method
    fn handle_tools_call(&self, params: Option<Value>) -> Result<Value> {
        let params = params.ok_or_else(|| anyhow!("Missing params"))?;

        let tool_name = params["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing tool name"))?;

        let arguments = params["arguments"]
            .as_object()
            .ok_or_else(|| anyhow!("Missing or invalid arguments"))?;

        let symbol = arguments
            .get("symbol")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing symbol parameter"))?;

        // Create a runtime for async operations
        let rt = Runtime::new()?;

        match tool_name {
            "get_stock_price" => {
                let price = rt.block_on(fetch_stock_price(symbol))?;
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": format!("Stock {} current price: ${:.2}", symbol.to_uppercase(), price)
                    }]
                }))
            }
            "get_stock_info" => {
                let info = rt.block_on(fetch_stock_info(symbol))?;
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": format!(
                            "Stock Information for {}:\n\
                             Price: ${:.2}\n\
                             Previous Close: ${:.2}\n\
                             Open: ${:.2}\n\
                             Day Range: ${:.2} - ${:.2}\n\
                             Market Cap: {}",
                            symbol.to_uppercase(),
                            info.price,
                            info.previous_close,
                            info.open,
                            info.day_low,
                            info.day_high,
                            info.market_cap
                        )
                    }]
                }))
            }
            _ => Err(anyhow!("Unknown tool: {}", tool_name)),
        }
    }

    /// Start the MCP server and process STDIO messages
    fn start(&self) -> Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut stderr = io::stderr();

        writeln!(stderr, "Stock Price MCP Server v{} started", self.version)?;
        writeln!(stderr, "Protocol version: {}", self.protocol_version)?;
        stderr.flush()?;

        for line in stdin.lock().lines() {
            let line = line?;

            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }

            writeln!(stderr, "Received: {}", line)?;
            stderr.flush()?;

            // Parse JSON-RPC request
            let request: JsonRpcRequest = match serde_json::from_str(&line) {
                Ok(req) => req,
                Err(e) => {
                    let error_response = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: None,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32700,
                            message: format!("Parse error: {}", e),
                            data: None,
                        }),
                    };
                    let response = serde_json::to_string(&error_response)?;
                    writeln!(stdout, "{}", response)?;
                    stdout.flush()?;
                    continue;
                }
            };

            // Handle request
            let response = self.handle_request(request);
            let response_json = serde_json::to_string(&response)?;

            writeln!(stderr, "Sending: {}", response_json)?;
            stderr.flush()?;

            writeln!(stdout, "{}", response_json)?;
            stdout.flush()?;
        }

        Ok(())
    }
}

/// Stock information structure
#[derive(Debug)]
struct StockInfo {
    price: f64,
    previous_close: f64,
    open: f64,
    day_low: f64,
    day_high: f64,
    market_cap: String,
}

/// Fetch stock price from Yahoo Finance using HTTP scraping
async fn fetch_stock_price(symbol: &str) -> Result<f64> {
    let url = format!("https://finance.yahoo.com/quote/{}", symbol);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let response = client.get(&url).send().await?;
    let html = response.text().await?;

    let document = scraper::Html::parse_document(&html);

    // Try multiple selectors for robustness
    let selectors = vec![
        format!(r#"fin-streamer[data-symbol="{}"][data-field="regularMarketPrice"]"#, symbol),
        r#"[data-test="qsp-price"]"#.to_string(),
        r#"[data-testid="qsp-price"]"#.to_string(),
    ];

    for selector_str in &selectors {
        if let Ok(selector) = scraper::Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                // Try data-value attribute first
                if let Some(value) = element.value().attr("data-value") {
                    if let Ok(price) = value.parse::<f64>() {
                        return Ok(price);
                    }
                }

                // Try text content
                let text = element.text().collect::<String>();
                let cleaned = text.trim().replace(',', "");
                if let Ok(price) = cleaned.parse::<f64>() {
                    return Ok(price);
                }
            }
        }
    }

    Err(anyhow!("Could not find stock price for symbol: {}", symbol))
}

/// Fetch detailed stock information from Yahoo Finance
async fn fetch_stock_info(symbol: &str) -> Result<StockInfo> {
    let url = format!("https://finance.yahoo.com/quote/{}", symbol);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let response = client.get(&url).send().await?;
    let html = response.text().await?;

    let document = scraper::Html::parse_document(&html);

    // Helper function to extract value
    let extract_value = |field: &str| -> Option<f64> {
        let selector_str = format!(r#"fin-streamer[data-symbol="{}"][data-field="{}"]"#, symbol, field);
        let selector = scraper::Selector::parse(&selector_str).ok()?;
        let element = document.select(&selector).next()?;

        if let Some(value) = element.value().attr("data-value") {
            value.parse::<f64>().ok()
        } else {
            let text = element.text().collect::<String>();
            text.trim().replace(',', "").parse::<f64>().ok()
        }
    };

    let price = extract_value("regularMarketPrice")
        .ok_or_else(|| anyhow!("Could not find price"))?;

    let previous_close = extract_value("regularMarketPreviousClose").unwrap_or(0.0);
    let open = extract_value("regularMarketOpen").unwrap_or(0.0);
    let day_low = extract_value("regularMarketDayLow").unwrap_or(0.0);
    let day_high = extract_value("regularMarketDayHigh").unwrap_or(0.0);

    // Extract market cap
    let market_cap = extract_value("marketCap")
        .map(|v| format_market_cap(v))
        .unwrap_or_else(|| "N/A".to_string());

    Ok(StockInfo {
        price,
        previous_close,
        open,
        day_low,
        day_high,
        market_cap,
    })
}

/// Format market cap for display
fn format_market_cap(value: f64) -> String {
    if value >= 1_000_000_000_000.0 {
        format!("${:.2}T", value / 1_000_000_000_000.0)
    } else if value >= 1_000_000_000.0 {
        format!("${:.2}B", value / 1_000_000_000.0)
    } else if value >= 1_000_000.0 {
        format!("${:.2}M", value / 1_000_000.0)
    } else {
        format!("${:.2}", value)
    }
}

fn main() -> Result<()> {
    let server = McpServer::new("Stock Price MCP Server", "1.0.0");
    server.start()
}
