# Comprehensive Guide to Building Model Context Protocol (MCP) Servers

## Table of Contents

1. [Introduction to MCP](#introduction-to-mcp)
2. [Core Concepts and Architecture](#core-concepts-and-architecture)
3. [Protocol Specification](#protocol-specification)
4. [MCP Primitives](#mcp-primitives)
5. [Transport Mechanisms](#transport-mechanisms)
6. [Building MCP Servers](#building-mcp-servers)
7. [Best Practices](#best-practices)
8. [Testing and Debugging](#testing-and-debugging)
9. [Real-World Examples and Use Cases](#real-world-examples-and-use-cases)
10. [Advanced Topics](#advanced-topics)
11. [Security Considerations](#security-considerations)
12. [Resources and References](#resources-and-references)

---

## Introduction to MCP

### What is the Model Context Protocol?

The Model Context Protocol (MCP) is an open standard introduced by Anthropic on November 25, 2024, that enables developers to build secure, two-way connections between their data sources and AI-powered tools. MCP standardizes how applications provide context to Large Language Models (LLMs), solving a fundamental problem: AI models are often constrained by their isolation from data—trapped behind information silos and legacy systems.

Think of MCP like a USB-C port for AI applications. Just as USB-C provides a standardized way to connect your devices to various peripherals and accessories, MCP provides a standardized way to connect AI models to different data sources and tools. Rather than building custom integrations for each data source and AI application combination, MCP provides a unified approach that works across multiple platforms.

### Why MCP Matters

Before MCP, developers faced several challenges:

- **Data Isolation**: AI models couldn't easily access data trapped in various systems, repositories, and tools
- **Custom Integrations**: Each data source required a separate, custom integration for each AI application
- **Maintenance Burden**: Multiple integrations meant more code to maintain and update
- **Lack of Standardization**: No consistent way to expose data, tools, and prompts to LLMs

MCP addresses these challenges by providing:

- **Universal Connectivity**: One MCP server works across multiple AI applications (Claude, Cursor, Replit, Zed, etc.)
- **Standardized Interface**: A common protocol for exposing context to LLMs
- **Reduced Complexity**: Build once, use everywhere
- **Open Standard**: Community-driven development with contributions from multiple organizations

### The MCP Ecosystem

The MCP ecosystem consists of several key components:

1. **Specification and SDKs**: Open-source specifications and software development kits available on GitHub in multiple languages (Python, TypeScript, Java, Kotlin, C#, Go, PHP, Ruby, Rust, and Swift)

2. **MCP Servers**: Programs that expose data, tools, and prompts to AI applications through the standardized MCP interface

3. **MCP Clients**: Built-in bridges within host applications that connect to MCP servers

4. **Host Applications**: AI applications that integrate MCP support, including:
   - Claude Desktop
   - Claude.ai (for Teams)
   - Messages API (via MCP connector)
   - Claude Code
   - Development tools (Zed, Replit, Codeium, Sourcegraph)

5. **Official MCP Servers**: Pre-built servers for popular enterprise systems including:
   - Google Drive
   - Slack
   - GitHub
   - Git
   - Postgres
   - Puppeteer
   - GitLab
   - Sentry
   - Atlassian (Confluence, Jira)

### Early Adoption and Industry Impact

Since its release, MCP has gained significant traction:

- Companies like Block and Apollo integrated MCP early on
- Development tools including Zed, Replit, Codeium, and Sourcegraph enhanced their platforms with MCP support
- OpenAI officially adopted MCP in March 2025
- Over 5,000 active MCP servers were running as of May 2025
- A vibrant community has emerged with contributions from developers worldwide

---

## Core Concepts and Architecture

### Client-Server Architecture

MCP implements a client-server architecture with clearly defined roles:

#### 1. MCP Host
The host is the AI application the user interacts with directly. Examples include:
- Claude Desktop
- Claude.ai
- Cursor IDE
- Custom AI applications

The host provides the user interface and manages the overall user experience.

#### 2. MCP Client
The client is a component built into the host application that:
- Manages connections to one or more MCP servers
- Handles protocol communication
- Negotiates capabilities with servers
- Enforces security policies (user consent, rate limits)
- Maintains session state

#### 3. MCP Server
The server is an external program that:
- Exposes capabilities (tools, resources, prompts) to clients
- Processes requests from clients
- Returns structured responses
- Maintains its own internal state
- Connects to data sources or external APIs

#### 4. Data Sources
These are the underlying systems the MCP server connects to:
- Local databases
- File systems
- REST APIs
- Third-party services
- Legacy systems
- Cloud platforms

### Communication Flow

The typical communication flow in an MCP system:

1. **User Interaction**: User interacts with the host application (e.g., types a message in Claude)
2. **Intent Detection**: The AI model identifies the need for external data or tool execution
3. **Client Request**: The MCP client sends a request to the appropriate server
4. **Server Processing**: The server processes the request, accesses data sources, and prepares a response
5. **Response Delivery**: The server returns structured data to the client
6. **Context Integration**: The client provides the data to the AI model as context
7. **Response Generation**: The AI model generates a natural language response incorporating the context
8. **User Display**: The host application displays the response to the user

### Capability-Based System

MCP uses a capability-based negotiation system where clients and servers explicitly declare their supported features during initialization. This ensures both parties understand what operations are available and prevents errors from unsupported features.

Capabilities are exchanged during the initialization phase and determine:
- Which protocol features are available
- What types of primitives (tools, resources, prompts) can be used
- Whether advanced features (sampling, roots) are supported
- Communication patterns and constraints

### Data Abstraction

A key principle of MCP is data abstraction. The host application doesn't need to know:
- Where data comes from
- How it's retrieved
- Implementation details of the server
- Authentication mechanisms with external systems

The server handles all these details and simply provides structured, contextualized data that the AI model can use effectively.

---

## Protocol Specification

### JSON-RPC 2.0 Foundation

MCP is built on JSON-RPC 2.0, providing a stateful session protocol focused on context exchange and sampling coordination between clients and servers. All messages in MCP must follow the JSON-RPC 2.0 specification, ensuring reliable and predictable interactions.

### Message Types

The protocol defines three fundamental message types:

#### 1. Requests

Requests are bidirectional messages that initiate operations and expect a response. They include:

**Required fields:**
- `jsonrpc`: Protocol version (must be "2.0")
- `id`: Unique identifier for correlating requests with responses (string or number)
- `method`: The operation to perform (string)

**Optional fields:**
- `params`: Parameters for the method (object or array)

**Example request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "get_weather",
    "arguments": {
      "location": "San Francisco, CA"
    }
  }
}
```

#### 2. Responses

Responses are sent in reply to requests and contain either a result or an error.

**Required fields:**
- `jsonrpc`: Protocol version (must be "2.0")
- `id`: Must match the request's id

**Success response includes:**
- `result`: The result of the operation (any valid JSON value)

**Error response includes:**
- `error`: Object containing:
  - `code`: Error code (integer)
  - `message`: Error description (string)
  - `data`: Additional error information (optional)

**Example success response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "temperature": 72,
    "conditions": "Sunny",
    "humidity": 45
  }
}
```

**Example error response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32602,
    "message": "Invalid location parameter",
    "data": {
      "parameter": "location",
      "reason": "Location not found"
    }
  }
}
```

#### 3. Notifications

Notifications are one-way messages that don't require or expect a response. They enable servers to proactively inform clients about state changes.

**Required fields:**
- `jsonrpc`: Protocol version (must be "2.0")
- `method`: The notification type (string)

**Optional fields:**
- `params`: Parameters for the notification (object or array)

**Example notification:**
```json
{
  "jsonrpc": "2.0",
  "method": "notifications/progress",
  "params": {
    "progressToken": "operation-123",
    "progress": 75,
    "total": 100
  }
}
```

### Connection Lifecycle

MCP defines a strict lifecycle for client-server connections, ensuring proper capability negotiation and state management. The initialization phase must be the first interaction between the client and server.

#### Phase 1: Initialization

**Step 1: Initialize Request**

The client initiates the connection by sending an `initialize` request:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2025-03-26",
    "capabilities": {
      "tools": {},
      "resources": {
        "subscribe": true
      }
    },
    "clientInfo": {
      "name": "ExampleClient",
      "version": "1.0.0"
    }
  }
}
```

**Step 2: Initialize Response**

The server responds with its capabilities:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "protocolVersion": "2025-03-26",
    "capabilities": {
      "tools": {},
      "resources": {
        "subscribe": true,
        "listChanged": true
      },
      "prompts": {
        "listChanged": true
      }
    },
    "serverInfo": {
      "name": "ExampleServer",
      "version": "1.0.0"
    },
    "instructions": "This server provides weather data and forecasts."
  }
}
```

**Step 3: Initialized Notification**

The client confirms readiness with an `initialized` notification:

```json
{
  "jsonrpc": "2.0",
  "method": "notifications/initialized"
}
```

**Important constraints:**
- Only `ping` requests and server logging notifications are permitted before initialization completes
- All other requests are FORBIDDEN before the `initialized` notification is sent
- Capability negotiation determines available features for the entire session

#### Phase 2: Operation

During the operation phase, the client and server exchange messages according to their negotiated capabilities. Common operations include:

**Discovering capabilities:**
- `resources/list`: List available resources
- `tools/list`: List available tools
- `prompts/list`: List available prompts

**Accessing resources:**
- `resources/read`: Read resource contents
- `resources/subscribe`: Subscribe to resource updates
- `resources/unsubscribe`: Unsubscribe from updates

**Calling tools:**
- `tools/call`: Execute a tool with specified arguments

**Using prompts:**
- `prompts/get`: Retrieve a prompt template

**Progress and logging:**
- `notifications/progress`: Report operation progress
- `notifications/message`: Send log messages

#### Phase 3: Shutdown

The connection is gracefully terminated through a shutdown sequence:

1. Client sends `shutdown` request
2. Server responds acknowledging the shutdown
3. Client may send `exit` notification
4. Connection is closed

### Error Codes

MCP inherits standard JSON-RPC error codes and adds application-level errors:

**Standard JSON-RPC errors:**
- `-32700`: Parse error (invalid JSON)
- `-32600`: Invalid request (malformed JSON-RPC)
- `-32601`: Method not found
- `-32602`: Invalid parameters
- `-32603`: Internal error

**MCP-specific errors:**
- Application-level errors returned within successful responses with an `isError` flag
- Tool execution errors indicating the tool ran but encountered an error
- Resource access errors for permission or availability issues

### Core Methods

MCP implementations must support specific JSON-RPC methods organized by capability:

#### Resource Methods
- `resources/list`: Enumerate available resources
- `resources/read`: Retrieve resource contents
- `resources/templates/list`: List resource templates
- `resources/subscribe`: Subscribe to resource changes
- `resources/unsubscribe`: Unsubscribe from resource changes

#### Tool Methods
- `tools/list`: Enumerate available tools
- `tools/call`: Execute a tool with arguments

#### Prompt Methods
- `prompts/list`: Enumerate available prompts
- `prompts/get`: Retrieve a prompt template

#### Sampling Methods (Advanced)
- `sampling/createMessage`: Request the client to generate a message using the LLM

#### Logging Methods
- `logging/setLevel`: Set the logging level

#### Completion Methods
- `completion/complete`: Request argument completion suggestions

---

## MCP Primitives

MCP servers expose three primary types of capabilities, called primitives, that enable AI models to access context and perform actions.

### 1. Resources

Resources allow servers to share data that provides context to language models. They are analogous to GET endpoints in a REST API—data sources that LLMs can access, providing information without performing significant computation or side effects.

#### What Are Resources?

Resources represent any data the server can provide:
- Files and documents
- Database records
- API responses
- Computed values
- Application-specific information
- Configuration data
- Real-time data streams

#### Resource Structure

A resource is identified by a URI and includes metadata:

```json
{
  "uri": "file:///project/README.md",
  "name": "Project README",
  "description": "Main project documentation",
  "mimeType": "text/markdown"
}
```

#### Implementing Resources

**Python Example using FastMCP:**

```python
from mcp.server.fastmcp import FastMCP
import json

mcp = FastMCP("Resource Server")

@mcp.resource("config://app/settings")
def get_app_settings() -> str:
    """Get application configuration settings"""
    settings = {
        "theme": "dark",
        "language": "en",
        "notifications": True
    }
    return json.dumps(settings, indent=2)

@mcp.resource("data://users/{user_id}")
def get_user_data(user_id: str) -> str:
    """Get user data by ID"""
    # Fetch from database
    user = fetch_user_from_db(user_id)
    return json.dumps(user)

if __name__ == "__main__":
    mcp.run(transport="stdio")
```

**TypeScript Example:**

```typescript
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";

const server = new Server({
  name: "ResourceServer",
  version: "1.0.0",
}, {
  capabilities: {
    resources: {}
  }
});

server.setRequestHandler("resources/list", async () => {
  return {
    resources: [
      {
        uri: "config://app/settings",
        name: "App Settings",
        description: "Application configuration",
        mimeType: "application/json"
      }
    ]
  };
});

server.setRequestHandler("resources/read", async (request) => {
  const uri = request.params.uri;

  if (uri === "config://app/settings") {
    return {
      contents: [
        {
          uri: uri,
          mimeType: "application/json",
          text: JSON.stringify({
            theme: "dark",
            language: "en"
          })
        }
      ]
    };
  }

  throw new Error("Resource not found");
});

const transport = new StdioServerTransport();
await server.connect(transport);
```

#### Resource Best Practices

1. **Use descriptive URIs**: Make resource identifiers self-documenting
2. **Include metadata**: Provide mime types and descriptions
3. **Handle errors gracefully**: Return meaningful error messages
4. **Support subscriptions**: Notify clients when resources change (if applicable)
5. **Efficient data loading**: Lazy-load large resources
6. **Validate access**: Implement appropriate access controls

### 2. Tools

Tools are functions that LLMs can call to perform specific actions. They enable AI models to interact with external systems, execute computations, and modify state.

#### What Are Tools?

Tools represent executable functions with well-defined inputs and outputs:
- API calls to external services
- Database operations
- File system operations
- Calculations and data processing
- System commands
- Integration with third-party services

#### Tool Structure

A tool definition includes:
- **Name**: Unique identifier for the tool
- **Description**: Clear explanation of what the tool does
- **Input Schema**: JSON Schema defining required and optional parameters
- **Output Format**: Structure of the returned data

#### Implementing Tools

**Python Example using FastMCP:**

```python
from mcp.server.fastmcp import FastMCP
import httpx

mcp = FastMCP("Weather Server")

@mcp.tool()
async def get_forecast(latitude: float, longitude: float) -> str:
    """Get weather forecast for a location

    Args:
        latitude: Latitude of the location
        longitude: Longitude of the location

    Returns:
        Weather forecast as a formatted string
    """
    async with httpx.AsyncClient() as client:
        # Get grid point
        response = await client.get(
            f"https://api.weather.gov/points/{latitude},{longitude}"
        )
        data = response.json()
        forecast_url = data["properties"]["forecast"]

        # Get forecast
        forecast_response = await client.get(forecast_url)
        forecast_data = forecast_response.json()

        periods = forecast_data["properties"]["periods"]
        result = []
        for period in periods[:3]:  # Next 3 periods
            result.append(
                f"{period['name']}: {period['temperature']}°{period['temperatureUnit']} "
                f"- {period['shortForecast']}"
            )

        return "\n".join(result)

@mcp.tool()
def calculate_sum(numbers: list[float]) -> float:
    """Calculate the sum of a list of numbers

    Args:
        numbers: List of numbers to sum

    Returns:
        The sum of all numbers
    """
    return sum(numbers)

if __name__ == "__main__":
    mcp.run(transport="stdio")
```

**TypeScript Example:**

```typescript
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { z } from "zod";

const server = new Server({
  name: "ToolServer",
  version: "1.0.0",
}, {
  capabilities: {
    tools: {}
  }
});

server.setRequestHandler("tools/list", async () => {
  return {
    tools: [
      {
        name: "calculate_sum",
        description: "Calculate the sum of a list of numbers",
        inputSchema: {
          type: "object",
          properties: {
            numbers: {
              type: "array",
              items: { type: "number" },
              description: "List of numbers to sum"
            }
          },
          required: ["numbers"]
        }
      }
    ]
  };
});

server.setRequestHandler("tools/call", async (request) => {
  const { name, arguments: args } = request.params;

  if (name === "calculate_sum") {
    const numbers = args.numbers as number[];
    const sum = numbers.reduce((a, b) => a + b, 0);

    return {
      content: [
        {
          type: "text",
          text: `The sum is: ${sum}`
        }
      ]
    };
  }

  throw new Error("Tool not found");
});
```

**Java Example using Spring:**

```java
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.ai.mcp.spring.McpTool;

@SpringBootApplication
public class WeatherServer {

    @McpTool(description = "Get weather forecast for coordinates")
    public String getWeather(
        @McpToolParameter(description = "Latitude") double latitude,
        @McpToolParameter(description = "Longitude") double longitude
    ) {
        // Implementation
        return "Weather data for " + latitude + ", " + longitude;
    }

    public static void main(String[] args) {
        SpringApplication.run(WeatherServer.class, args);
    }
}
```

#### Tool Best Practices

1. **Clear descriptions**: Explain what the tool does and when to use it
2. **Validate inputs**: Use JSON Schema to define and validate parameters
3. **Handle errors**: Return structured error information
4. **Async operations**: Use async/await for I/O operations
4. **Idempotency**: Make tools idempotent when possible
5. **User consent**: Remember that hosts must obtain user consent before invoking tools
6. **Timeout handling**: Implement timeouts for long-running operations
7. **Progress reporting**: Use progress notifications for lengthy operations

### 3. Prompts

Prompts are pre-written templates that guide the LLM to use tools or resources in optimal ways. They provide structured ways to interact with server capabilities.

#### What Are Prompts?

Prompts are reusable templates that:
- Guide AI models toward specific workflows
- Combine tools and resources effectively
- Provide context and instructions
- Include dynamic arguments for customization
- Standardize common interaction patterns

#### Prompt Structure

A prompt includes:
- **Name**: Unique identifier
- **Description**: Explanation of the prompt's purpose
- **Arguments**: Template variables that can be filled in
- **Messages**: The actual prompt content with placeholders

#### Implementing Prompts

**Python Example using FastMCP:**

```python
from mcp.server.fastmcp import FastMCP
from mcp.types import TextContent, ImageContent

mcp = FastMCP("Prompt Server")

@mcp.prompt()
def code_review_prompt(file_path: str, focus_areas: str = "general") -> str:
    """Generate a code review prompt for a specific file

    Args:
        file_path: Path to the file to review
        focus_areas: Specific areas to focus on (security, performance, style, general)

    Returns:
        Formatted code review prompt
    """
    return f"""Please review the code in {file_path} with a focus on {focus_areas}.

Consider the following aspects:
1. Code quality and readability
2. Potential bugs or edge cases
3. Performance implications
4. Security vulnerabilities
5. Best practices adherence

Provide specific suggestions for improvement with code examples where applicable."""

@mcp.prompt()
def debug_analysis_prompt(error_message: str, context: str = "") -> list:
    """Generate a debugging analysis prompt

    Args:
        error_message: The error message to analyze
        context: Additional context about when the error occurs

    Returns:
        Structured prompt messages
    """
    messages = [
        {
            "role": "user",
            "content": [
                TextContent(
                    type="text",
                    text=f"""I encountered the following error:

{error_message}

{f'Context: {context}' if context else ''}

Please help me:
1. Understand what caused this error
2. Identify the root cause
3. Suggest potential solutions
4. Recommend preventive measures for the future"""
                )
            ]
        }
    ]
    return messages

if __name__ == "__main__":
    mcp.run(transport="stdio")
```

**TypeScript Example:**

```typescript
server.setRequestHandler("prompts/list", async () => {
  return {
    prompts: [
      {
        name: "code_review",
        description: "Generate a code review prompt",
        arguments: [
          {
            name: "file_path",
            description: "Path to the file to review",
            required: true
          },
          {
            name: "focus_areas",
            description: "Areas to focus on",
            required: false
          }
        ]
      }
    ]
  };
});

server.setRequestHandler("prompts/get", async (request) => {
  const { name, arguments: args } = request.params;

  if (name === "code_review") {
    const filePath = args.file_path;
    const focusAreas = args.focus_areas || "general";

    return {
      messages: [
        {
          role: "user",
          content: {
            type: "text",
            text: `Please review the code in ${filePath} with a focus on ${focusAreas}...`
          }
        }
      ]
    };
  }

  throw new Error("Prompt not found");
});
```

#### Prompt Best Practices

1. **Descriptive arguments**: Clearly document what each argument does
2. **Default values**: Provide sensible defaults for optional arguments
3. **Context inclusion**: Include relevant context in the prompt
4. **Clear instructions**: Make expectations explicit
5. **Structured output**: Guide the model toward structured responses
6. **Reusability**: Design prompts to be reusable across scenarios

---

## Transport Mechanisms

MCP supports multiple transport mechanisms for client-server communication, each suited for different deployment scenarios.

### Transport Types Overview

MCP currently supports two standard transports with additional custom transport options:

1. **Stdio (Standard Input/Output)**: Local process communication
2. **SSE (Server-Sent Events)**: HTTP-based streaming
3. **Custom Transports**: WebSocket and other implementations

All transports use the JSON-RPC 2.0 wire protocol for message formatting.

### 1. Stdio Transport

#### Overview

The stdio transport uses standard input and output streams for communication. It's the recommended transport for local processes and CLI integrations.

#### Characteristics

- **Bidirectional**: Messages flow in both directions over stdin/stdout
- **Process-based**: Server runs as a separate process
- **Local only**: Limited to same-machine communication
- **Simple setup**: No network configuration required
- **Low latency**: Direct process communication

#### Implementation

**Python Example:**

```python
from mcp.server.fastmcp import FastMCP

mcp = FastMCP("StdioServer")

@mcp.tool()
def example_tool(param: str) -> str:
    """An example tool"""
    return f"Received: {param}"

if __name__ == "__main__":
    # Stdio is the default transport
    mcp.run(transport="stdio")
```

**TypeScript Example:**

```typescript
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";

const server = new Server({
  name: "StdioServer",
  version: "1.0.0",
}, {
  capabilities: {
    tools: {}
  }
});

// Set up handlers...

const transport = new StdioServerTransport();
await server.connect(transport);
```

#### Configuration

For Claude for Desktop, configure stdio servers in `claude_desktop_config.json`:

**macOS/Linux:**
```json
{
  "mcpServers": {
    "my-server": {
      "command": "python",
      "args": ["/absolute/path/to/server.py"]
    }
  }
}
```

**Windows:**
```json
{
  "mcpServers": {
    "my-server": {
      "command": "python.exe",
      "args": ["C:\\absolute\\path\\to\\server.py"]
    }
  }
}
```

#### Critical Stdio Rule

**NEVER write to stdout except for JSON-RPC messages.** This will corrupt the protocol communication.

**Safe logging approaches:**
- Python: Use `logging` module (writes to stderr)
- JavaScript: Use `console.error()` (writes to stderr)
- General: Write to stderr or log files

**Incorrect (breaks stdio):**
```python
print("Debug message")  # BAD - writes to stdout
```

**Correct:**
```python
import logging
logging.error("Debug message")  # GOOD - writes to stderr

import sys
print("Debug message", file=sys.stderr)  # GOOD - explicitly stderr
```

### 2. SSE (Server-Sent Events) Transport

#### Overview

Server-Sent Events (SSE) provide a persistent HTTP connection for server-to-client streaming, while client-to-server messages are sent using HTTP POST requests. This transport is useful for remote servers, web-based integrations, and environments where long-lived connections are preferred.

#### Characteristics

- **HTTP-based**: Works over standard HTTP(S)
- **Unidirectional streaming**: Server pushes events to client
- **HTTP POST for requests**: Client sends messages via POST
- **Remote capable**: Can work across networks
- **Firewall friendly**: Uses standard HTTP ports
- **Browser compatible**: Supported by web browsers

#### Architecture

SSE establishes a unidirectional communication channel from server to client over HTTP. Unlike WebSockets which offer bidirectional communication, SSE creates a persistent connection where servers push updates to clients while clients use separate HTTP requests to send messages back.

#### Implementation

**Python Example:**

```python
from mcp.server.fastmcp import FastMCP

mcp = FastMCP("SSEServer")

@mcp.tool()
def example_tool(param: str) -> str:
    """An example tool"""
    return f"Received: {param}"

if __name__ == "__main__":
    # Run with SSE transport
    mcp.run(
        transport="sse",
        host="127.0.0.1",  # Bind to localhost
        port=8000
    )
```

**TypeScript Example:**

```typescript
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { SSEServerTransport } from "@modelcontextprotocol/sdk/server/sse.js";
import express from "express";

const app = express();
const server = new Server({
  name: "SSEServer",
  version: "1.0.0",
}, {
  capabilities: {
    tools: {}
  }
});

// Set up handlers...

app.get("/sse", async (req, res) => {
  const transport = new SSEServerTransport("/messages", res);
  await server.connect(transport);
});

app.post("/messages", async (req, res) => {
  // Handle client messages
});

app.listen(8000);
```

#### Security Considerations for SSE

1. **Bind to localhost**: For local servers, bind only to 127.0.0.1
2. **Origin headers**: Check Origin headers for security
3. **Authentication**: Implement authentication for remote servers
4. **HTTPS**: Use TLS for remote connections
5. **Rate limiting**: Implement rate limits to prevent abuse

#### Configuration

For Claude for Desktop with SSE:

```json
{
  "mcpServers": {
    "my-sse-server": {
      "url": "http://localhost:8000/sse"
    }
  }
}
```

### 3. WebSocket Transport (Custom/Proposed)

#### Overview

While not yet an official MCP transport, WebSocket is being discussed in the community as a potential standard transport. WebSockets provide exactly the bidirectional message exchange that closely mimics the stdio transport behavior.

#### Characteristics

- **Bidirectional**: Full duplex communication
- **Persistent connection**: Long-lived connection
- **Low latency**: Efficient message passing
- **Standardized**: Widely supported protocol
- **Remote capable**: Works across networks

#### Community Discussion

The MCP community is actively discussing WebSocket support:
- Proposals for HTTP endpoint that upgrades to WebSocket
- Initial authorization via HTTP before WebSocket upgrade
- Would avoid leaving open-ended SSE sockets
- Better matches stdio communication patterns

#### Custom Implementation

You can implement custom WebSocket transports by conforming to the Transport interface:

```typescript
interface Transport {
  start(): Promise<void>;
  send(message: JSONRPCMessage): Promise<void>;
  close(): Promise<void>;
  onclose?: () => void;
  onerror?: (error: Error) => void;
  onmessage?: (message: JSONRPCMessage) => void;
}
```

### Transport Selection Guidelines

**Use Stdio when:**
- Building local development tools
- Integrating with CLI applications
- Server and client on same machine
- Simplicity is priority
- Low latency is critical

**Use SSE when:**
- Server needs to be remotely accessible
- Building web-based integrations
- Working with browser clients
- Need to traverse firewalls easily
- HTTP infrastructure exists

**Use WebSocket when:**
- Need bidirectional communication
- Building real-time applications
- Want stdio-like behavior over network
- Low latency is important
- Can implement custom transport

### Proxy Solutions

Tools like `mcp-proxy` can convert stdio-based servers to SSE or WebSocket services, exposing them as OpenAPI endpoints. This allows you to:
- Develop with stdio for simplicity
- Deploy with HTTP-based transports
- Expose servers as REST APIs
- Add authentication layers
- Implement load balancing

---

## Building MCP Servers

This section provides comprehensive guides for building MCP servers in multiple programming languages.

### Python Server Development

#### Setup and Prerequisites

**Requirements:**
- Python 3.10 or higher
- pip or uv package manager

**Initial Setup:**

```bash
# Using uv (recommended)
curl -LsSf https://astral.sh/uv/install.sh | sh
uv init my-mcp-server
cd my-mcp-server
uv venv && source .venv/bin/activate  # On Windows: .venv\Scripts\activate
uv add "mcp[cli]" httpx

# Using pip
python -m venv mcp-env
source mcp-env/bin/activate  # On Windows: mcp-env\Scripts\activate
pip install mcp "mcp[cli]" httpx
```

#### Basic Server Structure

```python
from mcp.server.fastmcp import FastMCP
import logging

# Configure logging (writes to stderr)
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Create server instance
mcp = FastMCP("MyServer")

# Server is now ready for tool/resource/prompt definitions

if __name__ == "__main__":
    mcp.run(transport="stdio")
```

#### Complete Example: Weather Server

```python
from mcp.server.fastmcp import FastMCP
from mcp.types import TextContent, Tool
import httpx
import logging
from typing import Optional

# Setup logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Create server
mcp = FastMCP("Weather Server")

async def get_weather_data(latitude: float, longitude: float) -> dict:
    """Helper function to fetch weather data from NWS API"""
    async with httpx.AsyncClient(timeout=30.0) as client:
        try:
            # Get grid point
            points_url = f"https://api.weather.gov/points/{latitude},{longitude}"
            response = await client.get(points_url)
            response.raise_for_status()
            points_data = response.json()

            return {
                "forecast_url": points_data["properties"]["forecast"],
                "forecast_hourly_url": points_data["properties"]["forecastHourly"],
                "station": points_data["properties"]["cwa"]
            }
        except Exception as e:
            logger.error(f"Error fetching weather data: {e}")
            raise

@mcp.tool()
async def get_forecast(latitude: float, longitude: float, periods: int = 3) -> str:
    """Get weather forecast for specified coordinates

    Args:
        latitude: Latitude (-90 to 90)
        longitude: Longitude (-180 to 180)
        periods: Number of forecast periods to return (default: 3)

    Returns:
        Formatted weather forecast string
    """
    try:
        weather_data = await get_weather_data(latitude, longitude)

        async with httpx.AsyncClient(timeout=30.0) as client:
            forecast_response = await client.get(weather_data["forecast_url"])
            forecast_response.raise_for_status()
            forecast_data = forecast_response.json()

            forecast_periods = forecast_data["properties"]["periods"][:periods]

            result_lines = []
            for period in forecast_periods:
                result_lines.append(
                    f"{period['name']}: "
                    f"{period['temperature']}°{period['temperatureUnit']} - "
                    f"{period['shortForecast']}\n"
                    f"{period['detailedForecast']}"
                )

            return "\n\n".join(result_lines)

    except Exception as e:
        return f"Error getting forecast: {str(e)}"

@mcp.tool()
async def get_alerts(state: str) -> str:
    """Get active weather alerts for a US state

    Args:
        state: Two-letter US state code (e.g., 'CA', 'TX')

    Returns:
        List of active weather alerts
    """
    try:
        async with httpx.AsyncClient(timeout=30.0) as client:
            alerts_url = f"https://api.weather.gov/alerts/active/area/{state}"
            response = await client.get(alerts_url)
            response.raise_for_status()
            alerts_data = response.json()

            features = alerts_data.get("features", [])

            if not features:
                return f"No active weather alerts for {state}"

            result_lines = []
            for alert in features:
                props = alert["properties"]
                result_lines.append(
                    f"Event: {props.get('event', 'Unknown')}\n"
                    f"Area: {props.get('areaDesc', 'Unknown')}\n"
                    f"Severity: {props.get('severity', 'Unknown')}\n"
                    f"Description: {props.get('description', 'No description')}\n"
                )

            return "\n---\n".join(result_lines)

    except Exception as e:
        return f"Error getting alerts: {str(e)}"

@mcp.resource("weather://config")
def get_config() -> str:
    """Get weather server configuration"""
    import json
    config = {
        "api": "National Weather Service",
        "coverage": "United States only",
        "version": "1.0.0"
    }
    return json.dumps(config, indent=2)

@mcp.prompt()
def weather_analysis_prompt(location: str) -> str:
    """Generate a prompt for detailed weather analysis

    Args:
        location: Location to analyze

    Returns:
        Weather analysis prompt
    """
    return f"""Please provide a detailed weather analysis for {location}.

Include:
1. Current conditions
2. Short-term forecast (next 24-48 hours)
3. Any weather alerts or warnings
4. Recommendations for outdoor activities
5. What to wear/bring

Use the available weather tools to get accurate, up-to-date information."""

if __name__ == "__main__":
    logger.info("Starting Weather MCP Server")
    mcp.run(transport="stdio")
```

#### Advanced Python Features

**Progress Reporting:**

```python
from mcp.types import ProgressNotification

@mcp.tool()
async def long_operation(items: list[str], ctx) -> str:
    """Process items with progress reporting"""
    total = len(items)

    for i, item in enumerate(items):
        # Report progress
        await ctx.report_progress(
            progress=i + 1,
            total=total,
            message=f"Processing {item}"
        )

        # Do work
        await process_item(item)

    return f"Processed {total} items"
```

**Dynamic Resources:**

```python
@mcp.resource("data://users/{user_id}")
async def get_user(user_id: str) -> str:
    """Get user data dynamically"""
    user = await fetch_user_from_database(user_id)
    return json.dumps(user)
```

**Error Handling:**

```python
from mcp.types import McpError

@mcp.tool()
async def risky_operation(param: str) -> str:
    """Operation that might fail"""
    try:
        result = await external_api_call(param)
        return result
    except ValueError as e:
        raise McpError(
            code=-32602,
            message="Invalid parameter",
            data={"parameter": param, "reason": str(e)}
        )
    except Exception as e:
        logger.error(f"Unexpected error: {e}")
        raise McpError(
            code=-32603,
            message="Internal server error",
            data={"error": str(e)}
        )
```

#### Testing Python Servers

```bash
# Run the development dashboard
mcp dev ./server.py

# Or test with MCP Inspector
npx @modelcontextprotocol/inspector python server.py
```

### TypeScript/Node.js Server Development

#### Setup and Prerequisites

**Requirements:**
- Node.js 16 or higher
- npm or yarn

**Initial Setup:**

```bash
mkdir my-mcp-server
cd my-mcp-server
npm init -y
npm install @modelcontextprotocol/sdk zod
npm install -D @types/node typescript

# Create source directory
mkdir src
touch src/index.ts

# Create tsconfig.json
cat > tsconfig.json << 'EOF'
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "Node16",
    "moduleResolution": "Node16",
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules"]
}
EOF
```

**Add to package.json:**

```json
{
  "type": "module",
  "scripts": {
    "build": "tsc",
    "start": "node dist/index.js"
  }
}
```

#### Basic Server Structure

```typescript
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";

const server = new Server(
  {
    name: "MyServer",
    version: "1.0.0",
  },
  {
    capabilities: {
      tools: {},
      resources: {},
      prompts: {},
    },
  }
);

// Set up request handlers here...

// Start server
const transport = new StdioServerTransport();
await server.connect(transport);
console.error("Server started");  // Log to stderr
```

#### Complete Example: GitHub Server

```typescript
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
  ListResourcesRequestSchema,
  ReadResourceRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { z } from "zod";

// GitHub API interface
interface GitHubRepo {
  name: string;
  full_name: string;
  description: string;
  html_url: string;
  stargazers_count: number;
}

const GITHUB_TOKEN = process.env.GITHUB_TOKEN;

async function fetchFromGitHub(endpoint: string): Promise<any> {
  const response = await fetch(`https://api.github.com${endpoint}`, {
    headers: {
      Authorization: `Bearer ${GITHUB_TOKEN}`,
      Accept: "application/vnd.github.v3+json",
    },
  });

  if (!response.ok) {
    throw new Error(`GitHub API error: ${response.statusText}`);
  }

  return response.json();
}

const server = new Server(
  {
    name: "GitHubServer",
    version: "1.0.0",
  },
  {
    capabilities: {
      tools: {},
      resources: {},
    },
  }
);

// List available tools
server.setRequestHandler(ListToolsRequestSchema, async () => {
  return {
    tools: [
      {
        name: "search_repositories",
        description: "Search for GitHub repositories",
        inputSchema: {
          type: "object",
          properties: {
            query: {
              type: "string",
              description: "Search query",
            },
            limit: {
              type: "number",
              description: "Maximum number of results",
              default: 5,
            },
          },
          required: ["query"],
        },
      },
      {
        name: "get_repository",
        description: "Get details about a specific repository",
        inputSchema: {
          type: "object",
          properties: {
            owner: {
              type: "string",
              description: "Repository owner",
            },
            repo: {
              type: "string",
              description: "Repository name",
            },
          },
          required: ["owner", "repo"],
        },
      },
    ],
  };
});

// Handle tool calls
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  try {
    if (name === "search_repositories") {
      const query = args.query as string;
      const limit = (args.limit as number) || 5;

      const data = await fetchFromGitHub(
        `/search/repositories?q=${encodeURIComponent(query)}&per_page=${limit}`
      );

      const repos = data.items.map((repo: GitHubRepo) => ({
        name: repo.full_name,
        description: repo.description,
        stars: repo.stargazers_count,
        url: repo.html_url,
      }));

      return {
        content: [
          {
            type: "text",
            text: JSON.stringify(repos, null, 2),
          },
        ],
      };
    }

    if (name === "get_repository") {
      const owner = args.owner as string;
      const repo = args.repo as string;

      const data = await fetchFromGitHub(`/repos/${owner}/${repo}`);

      return {
        content: [
          {
            type: "text",
            text: JSON.stringify(
              {
                name: data.full_name,
                description: data.description,
                stars: data.stargazers_count,
                forks: data.forks_count,
                language: data.language,
                url: data.html_url,
              },
              null,
              2
            ),
          },
        ],
      };
    }

    throw new Error(`Unknown tool: ${name}`);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    return {
      content: [
        {
          type: "text",
          text: `Error: ${errorMessage}`,
        },
      ],
      isError: true,
    };
  }
});

// List available resources
server.setRequestHandler(ListResourcesRequestSchema, async () => {
  return {
    resources: [
      {
        uri: "github://config",
        name: "GitHub Configuration",
        description: "Server configuration and status",
        mimeType: "application/json",
      },
    ],
  };
});

// Read resources
server.setRequestHandler(ReadResourceRequestSchema, async (request) => {
  const uri = request.params.uri;

  if (uri === "github://config") {
    return {
      contents: [
        {
          uri,
          mimeType: "application/json",
          text: JSON.stringify(
            {
              authenticated: !!GITHUB_TOKEN,
              api_url: "https://api.github.com",
              version: "1.0.0",
            },
            null,
            2
          ),
        },
      ],
    };
  }

  throw new Error(`Unknown resource: ${uri}`);
});

// Error handling
server.onerror = (error) => {
  console.error("[MCP Error]", error);
};

process.on("SIGINT", async () => {
  await server.close();
  process.exit(0);
});

// Start server
const transport = new StdioServerTransport();
await server.connect(transport);
console.error("GitHub MCP Server running on stdio");
```

#### TypeScript Best Practices

**Use Zod for validation:**

```typescript
import { z } from "zod";

const SearchSchema = z.object({
  query: z.string().min(1),
  limit: z.number().int().positive().max(100).default(10),
});

// In tool handler:
const validated = SearchSchema.parse(args);
```

**Type-safe error handling:**

```typescript
type ToolResult = {
  content: Array<{ type: "text"; text: string }>;
  isError?: boolean;
};

async function handleToolCall(name: string, args: unknown): Promise<ToolResult> {
  try {
    // Implementation
    return {
      content: [{ type: "text", text: "Success" }],
    };
  } catch (error) {
    return {
      content: [
        {
          type: "text",
          text: error instanceof Error ? error.message : "Unknown error",
        },
      ],
      isError: true,
    };
  }
}
```

### Java Server Development

#### Setup and Prerequisites

**Requirements:**
- Java 17 or higher
- Spring Boot 3.3 or higher
- Maven or Gradle

**Spring Boot Setup:**

```xml
<!-- pom.xml -->
<dependencies>
    <dependency>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter</artifactId>
    </dependency>
    <dependency>
        <groupId>org.springframework.ai</groupId>
        <artifactId>spring-ai-starter-mcp-server</artifactId>
    </dependency>
</dependencies>
```

#### Basic Server with Spring

```java
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.ai.mcp.spring.McpTool;
import org.springframework.ai.mcp.spring.McpToolParameter;

@SpringBootApplication
public class CalculatorServer {

    @McpTool(description = "Add two numbers together")
    public double add(
        @McpToolParameter(description = "First number") double a,
        @McpToolParameter(description = "Second number") double b
    ) {
        return a + b;
    }

    @McpTool(description = "Multiply two numbers")
    public double multiply(
        @McpToolParameter(description = "First number") double a,
        @McpToolParameter(description = "Second number") double b
    ) {
        return a * b;
    }

    public static void main(String[] args) {
        SpringApplication.run(CalculatorServer.class, args);
    }
}
```

### C# Server Development

#### Setup and Prerequisites

**Requirements:**
- .NET 8 SDK or higher

**Setup:**

```bash
dotnet new console -n MyMcpServer
cd MyMcpServer
dotnet add package ModelContextProtocol
dotnet add package Microsoft.Extensions.Hosting
```

#### Basic Server Structure

```csharp
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using ModelContextProtocol;
using ModelContextProtocol.Server;

var builder = Host.CreateEmptyApplicationBuilder(null);

builder.Services
    .AddMcpServer()
    .WithStdioTransport();

builder.Services.AddSingleton<CalculatorService>();

var app = builder.Build();
await app.RunAsync();

public class CalculatorService
{
    [McpServerTool(description: "Add two numbers")]
    public double Add(double a, double b)
    {
        return a + b;
    }

    [McpServerTool(description: "Multiply two numbers")]
    public double Multiply(double a, double b)
    {
        return a * b;
    }
}
```

---

## Best Practices

### Error Handling

#### Structured Error Responses

Always return structured error information:

```python
from mcp.types import McpError

@mcp.tool()
def validate_input(data: str) -> str:
    try:
        # Validation logic
        if not data:
            raise ValueError("Data cannot be empty")

        return process_data(data)

    except ValueError as e:
        raise McpError(
            code=-32602,
            message="Invalid input",
            data={
                "parameter": "data",
                "reason": str(e),
                "suggestion": "Provide non-empty data"
            }
        )
    except Exception as e:
        logging.error(f"Unexpected error: {e}", exc_info=True)
        raise McpError(
            code=-32603,
            message="Internal error",
            data={"type": type(e).__name__}
        )
```

#### Retry Logic

Implement retry logic for transient failures:

```python
import asyncio
from typing import TypeVar, Callable

T = TypeVar('T')

async def retry_with_backoff(
    func: Callable[[], T],
    max_retries: int = 3,
    initial_delay: float = 1.0,
    backoff_factor: float = 2.0
) -> T:
    """Retry function with exponential backoff"""
    delay = initial_delay

    for attempt in range(max_retries):
        try:
            return await func()
        except Exception as e:
            if attempt == max_retries - 1:
                raise

            logging.warning(
                f"Attempt {attempt + 1} failed: {e}. "
                f"Retrying in {delay}s..."
            )
            await asyncio.sleep(delay)
            delay *= backoff_factor
```

### Logging Best Practices

#### Always Log to Stderr

**Critical rule:** For stdio-based servers, all logs must go to stderr:

```python
import logging
import sys

# Configure logging to stderr
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    stream=sys.stderr  # Explicitly use stderr
)

logger = logging.getLogger(__name__)
```

#### Structured Logging

Use structured logging for better debugging:

```python
import json
import logging

def log_structured(level: str, message: str, **kwargs):
    """Log structured JSON to stderr"""
    log_entry = {
        "timestamp": datetime.utcnow().isoformat(),
        "level": level,
        "message": message,
        **kwargs
    }
    print(json.dumps(log_entry), file=sys.stderr)

# Usage
log_structured(
    "INFO",
    "Tool called",
    tool_name="get_weather",
    parameters={"lat": 37.7749, "lon": -122.4194}
)
```

#### Log Context

Include context in logs:

```python
@mcp.tool()
async def process_request(request_id: str, data: dict) -> str:
    logger.info(
        f"Processing request",
        extra={
            "request_id": request_id,
            "data_size": len(str(data))
        }
    )

    try:
        result = await process(data)
        logger.info(
            f"Request completed successfully",
            extra={"request_id": request_id}
        )
        return result
    except Exception as e:
        logger.error(
            f"Request failed",
            extra={
                "request_id": request_id,
                "error": str(e)
            },
            exc_info=True
        )
        raise
```

### Performance Optimization

#### Async Operations

Use async/await for I/O operations:

```python
import asyncio
import httpx

@mcp.tool()
async def fetch_multiple_sources(urls: list[str]) -> list[str]:
    """Fetch from multiple URLs concurrently"""
    async with httpx.AsyncClient() as client:
        tasks = [client.get(url) for url in urls]
        responses = await asyncio.gather(*tasks, return_exceptions=True)

        results = []
        for response in responses:
            if isinstance(response, Exception):
                results.append(f"Error: {response}")
            else:
                results.append(response.text)

        return results
```

#### Caching

Implement caching for expensive operations:

```python
from functools import lru_cache
import time

class CachedDataSource:
    def __init__(self, ttl_seconds: int = 300):
        self.ttl = ttl_seconds
        self.cache = {}

    def get(self, key: str):
        if key in self.cache:
            data, timestamp = self.cache[key]
            if time.time() - timestamp < self.ttl:
                return data

        # Fetch fresh data
        data = self.fetch_data(key)
        self.cache[key] = (data, time.time())
        return data
```

#### Resource Cleanup

Ensure proper cleanup:

```python
from contextlib import asynccontextmanager

class DatabaseServer:
    def __init__(self):
        self.db_connection = None

    async def start(self):
        self.db_connection = await connect_to_database()

    async def close(self):
        if self.db_connection:
            await self.db_connection.close()

    @asynccontextmanager
    async def transaction(self):
        async with self.db_connection.transaction():
            yield
```

### Input Validation

#### JSON Schema Validation

Use JSON Schema for tool inputs:

```python
@mcp.tool()
def create_user(
    username: str,  # Required
    email: str,     # Required
    age: int = 18,  # Optional with default
    roles: list[str] = None  # Optional
) -> str:
    """Create a new user

    Args:
        username: Unique username (3-20 characters)
        email: Valid email address
        age: User age (must be 18+)
        roles: List of user roles

    Returns:
        User ID
    """
    # Validation
    if not 3 <= len(username) <= 20:
        raise ValueError("Username must be 3-20 characters")

    if not is_valid_email(email):
        raise ValueError("Invalid email address")

    if age < 18:
        raise ValueError("User must be 18 or older")

    # Create user
    user_id = create_user_in_db(username, email, age, roles or [])
    return user_id
```

#### Type Hints

Use type hints for clarity and validation:

```typescript
import { z } from "zod";

const CreateUserSchema = z.object({
  username: z.string().min(3).max(20),
  email: z.string().email(),
  age: z.number().int().min(18).default(18),
  roles: z.array(z.string()).optional(),
});

type CreateUserInput = z.infer<typeof CreateUserSchema>;

async function createUser(input: unknown): Promise<string> {
  const validated = CreateUserSchema.parse(input);
  // Use validated data
  return userId;
}
```

### Documentation

#### Clear Tool Descriptions

Provide comprehensive descriptions:

```python
@mcp.tool()
async def search_documents(
    query: str,
    filters: dict[str, any] = None,
    limit: int = 10,
    offset: int = 0
) -> list[dict]:
    """Search for documents in the knowledge base

    This tool searches through all indexed documents and returns
    relevant results based on the query. Results are ranked by
    relevance score.

    Args:
        query: Search query string. Supports boolean operators
               (AND, OR, NOT) and phrase matching with quotes.
               Example: "machine learning" AND python

        filters: Optional filters to narrow results:
                 - category: str - Document category
                 - date_from: str - ISO date (YYYY-MM-DD)
                 - date_to: str - ISO date (YYYY-MM-DD)
                 - author: str - Document author

        limit: Maximum number of results to return (1-100, default: 10)

        offset: Number of results to skip for pagination (default: 0)

    Returns:
        List of matching documents, each containing:
        - id: Document ID
        - title: Document title
        - excerpt: Relevant excerpt with query terms highlighted
        - score: Relevance score (0-1)
        - url: Link to full document

    Examples:
        Basic search:
        search_documents("python programming")

        Filtered search:
        search_documents(
            "API design",
            filters={"category": "technical", "date_from": "2024-01-01"}
        )

        Paginated search:
        search_documents("machine learning", limit=20, offset=40)
    """
    # Implementation
    pass
```

#### Resource Documentation

Document resource URIs and formats:

```python
@mcp.resource("data://users/{user_id}")
async def get_user(user_id: str) -> str:
    """Get user profile data

    URI Format: data://users/{user_id}

    Parameters:
        user_id: Numeric user ID or special values:
                 - "me" for current user
                 - "all" for all users (requires admin role)

    Returns:
        JSON object containing:
        - id: User ID
        - username: Username
        - email: Email address
        - created_at: ISO timestamp
        - last_login: ISO timestamp
        - roles: Array of role names
        - profile: Nested profile object

    MIME Type: application/json

    Requires: read:users permission
    """
    # Implementation
    pass
```

---

## Testing and Debugging

### Testing Tools

#### MCP Inspector

The official MCP Inspector provides visual testing:

```bash
# Install
npm install -g @modelcontextprotocol/inspector

# Test Python server
npx @modelcontextprotocol/inspector python server.py

# Test Node server
npx @modelcontextprotocol/inspector node dist/server.js

# Test with arguments
npx @modelcontextprotocol/inspector python server.py --config config.json
```

The Inspector provides:
- Interactive tool testing
- Resource browsing
- Prompt testing
- Request/response inspection
- Real-time logging

#### Python Development Dashboard

For Python servers using FastMCP:

```bash
# Start development dashboard
mcp dev ./server.py

# Opens web interface for testing
```

### Unit Testing

#### Testing Python Tools

```python
import pytest
from server import mcp, calculate_sum, get_forecast

@pytest.mark.asyncio
async def test_calculate_sum():
    """Test sum calculation tool"""
    result = calculate_sum([1, 2, 3, 4, 5])
    assert result == 15

@pytest.mark.asyncio
async def test_calculate_sum_empty():
    """Test sum with empty list"""
    result = calculate_sum([])
    assert result == 0

@pytest.mark.asyncio
async def test_get_forecast(httpx_mock):
    """Test weather forecast tool"""
    # Mock API responses
    httpx_mock.add_response(
        url="https://api.weather.gov/points/37.7749,-122.4194",
        json={
            "properties": {
                "forecast": "https://api.weather.gov/forecast/1"
            }
        }
    )

    httpx_mock.add_response(
        url="https://api.weather.gov/forecast/1",
        json={
            "properties": {
                "periods": [
                    {
                        "name": "Today",
                        "temperature": 72,
                        "temperatureUnit": "F",
                        "shortForecast": "Sunny"
                    }
                ]
            }
        }
    )

    result = await get_forecast(37.7749, -122.4194, periods=1)
    assert "Today" in result
    assert "72°F" in result
```

#### Testing TypeScript Servers

```typescript
import { describe, it, expect, vi } from "vitest";
import { handleToolCall } from "./server";

describe("Tool Handlers", () => {
  it("should calculate sum correctly", async () => {
    const result = await handleToolCall("calculate_sum", {
      numbers: [1, 2, 3, 4, 5],
    });

    expect(result.content[0].text).toBe("15");
  });

  it("should handle errors gracefully", async () => {
    const result = await handleToolCall("divide", {
      a: 10,
      b: 0,
    });

    expect(result.isError).toBe(true);
    expect(result.content[0].text).toContain("division by zero");
  });
});
```

### Integration Testing

#### Testing with Mock Clients

```python
import asyncio
from mcp.client import ClientSession
from mcp.client.stdio import stdio_client

async def test_server_integration():
    """Integration test with mock client"""
    async with stdio_client(
        command="python",
        args=["server.py"]
    ) as (read, write):
        async with ClientSession(read, write) as session:
            # Initialize
            await session.initialize()

            # List tools
            tools = await session.list_tools()
            assert len(tools.tools) > 0

            # Call tool
            result = await session.call_tool(
                "calculate_sum",
                {"numbers": [1, 2, 3]}
            )
            assert result.content[0].text == "6"

# Run test
asyncio.run(test_server_integration())
```

### Debugging Techniques

#### Log Tailing

Monitor server and host logs simultaneously:

```bash
# macOS Claude for Desktop logs
tail -f ~/Library/Logs/Claude/mcp*.log

# Windows Claude for Desktop logs
Get-Content "$env:APPDATA\Claude\logs\mcp*.log" -Wait -Tail 50

# Your server logs (if writing to file)
tail -f server.log
```

#### Debug Output

Add comprehensive debug logging:

```python
import logging

logger = logging.getLogger(__name__)

@mcp.tool()
async def debug_tool(param: str) -> str:
    """Tool with debug logging"""
    logger.debug(f"Tool called with param: {param}")

    try:
        logger.debug("Starting processing")
        result = process(param)
        logger.debug(f"Processing complete: {result}")

        return result

    except Exception as e:
        logger.error(f"Error in debug_tool", exc_info=True)
        raise
```

#### Request/Response Logging

Log all JSON-RPC messages:

```python
import json
import sys

def log_message(direction: str, message: dict):
    """Log JSON-RPC messages"""
    log_entry = {
        "direction": direction,  # "incoming" or "outgoing"
        "message": message
    }
    print(json.dumps(log_entry), file=sys.stderr)

# In your server
original_send = transport.send

async def logged_send(message):
    log_message("outgoing", message)
    return await original_send(message)

transport.send = logged_send
```

### Common Issues and Solutions

#### Issue: Server Not Appearing in Claude

**Troubleshooting:**
1. Check `claude_desktop_config.json` syntax
2. Use absolute paths (not relative)
3. Fully restart Claude (Cmd+Q on macOS, exit system tray on Windows)
4. Check logs at `~/Library/Logs/Claude/mcp*.log`

**Example working config:**
```json
{
  "mcpServers": {
    "my-server": {
      "command": "/usr/local/bin/python",
      "args": ["/Users/username/servers/server.py"]
    }
  }
}
```

#### Issue: Tools Not Working

**Troubleshooting:**
1. Check tool appears in Claude's "Search and tools" slider
2. Verify tool input schema is valid JSON Schema
3. Check server logs for errors
4. Test tool with MCP Inspector
5. Ensure user consent is being granted

#### Issue: Logging Breaks Server

**Cause:** Writing to stdout in stdio transport

**Solution:**
```python
# BAD - writes to stdout
print("Debug message")

# GOOD - writes to stderr
import sys
print("Debug message", file=sys.stderr)

# BETTER - use logging
import logging
logging.basicConfig(stream=sys.stderr)
logger = logging.getLogger(__name__)
logger.info("Debug message")
```

#### Issue: Weather API Errors

**Cause:** National Weather Service API only supports US coordinates

**Solution:**
- Use coordinates within United States
- Add validation for coordinate ranges
- Provide clear error messages

```python
def validate_us_coordinates(lat: float, lon: float) -> bool:
    """Check if coordinates are roughly within US"""
    return (24.0 <= lat <= 50.0) and (-125.0 <= lon <= -66.0)
```

---

## Real-World Examples and Use Cases

### Official Production Servers

#### GitHub MCP Server

The official GitHub server enables AI to interact with GitHub repositories:

**Capabilities:**
- Repository management and file operations
- Issue and pull request management
- Code search and analysis
- Workflow run inspection
- Security insights (code scanning, Dependabot)

**Installation:**
```bash
npm install -g @github/mcp-server

# Configure in claude_desktop_config.json
{
  "mcpServers": {
    "github": {
      "command": "mcp-server-github",
      "env": {
        "GITHUB_TOKEN": "your_token_here"
      }
    }
  }
}
```

**Use cases:**
- Code reviews with AI assistance
- Automated issue triage
- PR summarization
- Repository analysis

#### Google Drive MCP Server

Access and search Google Drive files:

**Capabilities:**
- File search
- Document reading
- Folder navigation
- File metadata retrieval

#### Slack MCP Server

Integrate with Slack workspaces:

**Capabilities:**
- Channel management
- Message posting and reading
- User information
- Workspace search

### Community Examples

#### Browser Automation Server

Using Playwright for web automation:

**Implementation concept:**
```python
from mcp.server.fastmcp import FastMCP
from playwright.async_api import async_playwright

mcp = FastMCP("Browser Automation")

@mcp.tool()
async def navigate_to_page(url: str) -> str:
    """Navigate to a URL and return page title"""
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        page = await browser.new_page()
        await page.goto(url)
        title = await page.title()
        await browser.close()
        return f"Page title: {title}"

@mcp.tool()
async def extract_text(url: str, selector: str) -> str:
    """Extract text from element on page"""
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        page = await browser.new_page()
        await page.goto(url)
        text = await page.text_content(selector)
        await browser.close()
        return text or "No text found"
```

**Use cases:**
- Web scraping
- Automated testing
- Form filling
- Screenshot capture

#### Database Query Server

Provide natural language database access:

**Implementation concept:**
```python
from mcp.server.fastmcp import FastMCP
import psycopg2
import json

mcp = FastMCP("Database Server")

@mcp.tool()
async def query_database(sql: str) -> str:
    """Execute SQL query and return results

    IMPORTANT: Only SELECT queries are allowed
    """
    # Validate query
    if not sql.strip().upper().startswith("SELECT"):
        raise ValueError("Only SELECT queries allowed")

    conn = psycopg2.connect(DATABASE_URL)
    cur = conn.cursor()

    try:
        cur.execute(sql)
        rows = cur.fetchall()
        columns = [desc[0] for desc in cur.description]

        results = [dict(zip(columns, row)) for row in rows]
        return json.dumps(results, indent=2)

    finally:
        cur.close()
        conn.close()

@mcp.resource("schema://tables")
def get_schema() -> str:
    """Get database schema"""
    # Return schema information
    pass
```

**Use cases:**
- Natural language database queries
- Data analysis
- Report generation
- Schema exploration

#### Docker Management Server

Control Docker containers with natural language:

**Capabilities:**
- Container lifecycle management
- Image operations
- Volume management
- Network inspection

**Use cases:**
- DevOps automation
- Container debugging
- Resource monitoring
- Deployment orchestration

#### Music Production Server

Ableton Live integration:

**Capabilities:**
- Track creation
- Effect application
- Session manipulation
- Audio processing

**Use cases:**
- AI-assisted music production
- Automated mixing
- Sound design
- Composition assistance

### Development Tools Integration

#### Zed Editor

MCP support in Zed enhances code intelligence:
- Context-aware code completion
- Project-specific knowledge
- Tool integration

#### Replit

MCP enables Replit to:
- Access external APIs
- Integrate with services
- Provide enhanced context

#### Cursor IDE

Cursor uses MCP for:
- Enhanced code understanding
- External tool access
- Custom integrations

### Enterprise Use Cases

#### Customer Support Automation

**Scenario:** AI assistant with access to CRM, documentation, and ticketing systems

**Implementation:**
```python
@mcp.tool()
async def search_tickets(customer_id: str, status: str = "open") -> list[dict]:
    """Search support tickets for a customer"""
    tickets = await crm_api.get_tickets(customer_id, status)
    return tickets

@mcp.tool()
async def get_customer_history(customer_id: str) -> dict:
    """Get customer interaction history"""
    history = await crm_api.get_history(customer_id)
    return history

@mcp.resource("docs://knowledge-base/{article_id}")
async def get_kb_article(article_id: str) -> str:
    """Get knowledge base article"""
    article = await kb_api.get_article(article_id)
    return article.content
```

**Benefits:**
- Faster response times
- Consistent answers
- Context-aware support
- Reduced escalations

#### Internal Tool Integration

**Scenario:** Connect AI to internal APIs and databases

**Implementation:**
```python
@mcp.tool()
async def query_sales_data(
    start_date: str,
    end_date: str,
    region: str = "all"
) -> dict:
    """Query sales metrics"""
    data = await internal_api.get_sales(start_date, end_date, region)
    return {
        "revenue": data.total_revenue,
        "transactions": data.transaction_count,
        "average_order_value": data.aov,
        "top_products": data.top_products[:10]
    }

@mcp.tool()
async def generate_report(report_type: str, parameters: dict) -> str:
    """Generate business report"""
    report = await reporting_api.generate(report_type, parameters)
    return report.url
```

**Benefits:**
- Natural language data access
- Automated reporting
- Cross-system insights
- Reduced manual queries

#### Development Workflow Automation

**Scenario:** AI-assisted development workflows

**Capabilities:**
- Code analysis
- Test generation
- Deployment automation
- Documentation generation

**Example:**
```python
@mcp.tool()
async def run_tests(test_path: str = "tests/") -> dict:
    """Run test suite"""
    result = await subprocess_run(["pytest", test_path, "--json"])
    return parse_test_results(result)

@mcp.tool()
async def analyze_code_coverage(path: str) -> dict:
    """Analyze code coverage"""
    coverage = await coverage_tool.analyze(path)
    return {
        "overall_coverage": coverage.percentage,
        "uncovered_lines": coverage.missing_lines,
        "files_below_threshold": coverage.low_coverage_files
    }
```

---

## Advanced Topics

### Sampling

Sampling is a powerful MCP feature that allows servers to request LLM completions through the client. This enables sophisticated agentic behaviors while maintaining security and privacy.

#### What is Sampling?

Sampling allows an MCP server to ask the client to generate text using the LLM. The server never directly accesses the model—instead, it requests that the client perform the generation.

**Key benefits:**
- Servers don't need their own API keys
- Context isolation maintained
- User retains control
- Client enforces policies

#### Use Cases for Sampling

1. **Agentic Workflows**: Server needs AI-generated content to complete a task
2. **Content Generation**: Tool that processes and enhances user input
3. **Decision Making**: AI helps decide which actions to take
4. **Data Transformation**: Converting data formats with AI assistance

#### Implementing Sampling

**Python Example:**

```python
from mcp.server.fastmcp import FastMCP
from mcp.types import SamplingMessage, TextContent

mcp = FastMCP("Sampling Server")

@mcp.tool()
async def enhance_documentation(code: str, ctx) -> str:
    """Generate enhanced documentation for code"""

    # Request sampling from client
    messages = [
        SamplingMessage(
            role="user",
            content=TextContent(
                type="text",
                text=f"""Please analyze this code and generate comprehensive documentation:

{code}

Include:
1. Overview of functionality
2. Parameter descriptions
3. Return value explanation
4. Usage examples
5. Edge cases and limitations
"""
            )
        )
    ]

    result = await ctx.sample(
        messages=messages,
        max_tokens=1000,
        temperature=0.7
    )

    return result.content
```

#### Security Considerations

- Client must approve sampling requests
- Rate limits enforced by client
- Context isolated from main conversation
- User can review generated content

### Roots

Roots define filesystem boundaries for server operations, allowing clients to specify which directories servers should focus on.

#### What are Roots?

Roots represent:
- Directories servers can access
- Project boundaries
- Security boundaries
- Context scope

#### Implementing Roots Support

**Python Example:**

```python
@mcp.tool()
async def search_project_files(pattern: str, ctx) -> list[str]:
    """Search for files within project roots"""

    # Get roots from client
    roots = await ctx.get_roots()

    results = []
    for root in roots:
        root_path = root.uri.replace("file://", "")
        matches = glob.glob(
            f"{root_path}/**/{pattern}",
            recursive=True
        )
        results.extend(matches)

    return results
```

#### Root Notifications

Servers can subscribe to root changes:

```python
async def handle_roots_changed(notification):
    """Handle roots list changes"""
    new_roots = notification.params.roots
    logger.info(f"Roots updated: {new_roots}")
    # Re-index or update caches
    await reindex_files(new_roots)
```

### Progress Notifications

For long-running operations, report progress to improve UX:

```python
@mcp.tool()
async def process_large_dataset(file_path: str, ctx) -> str:
    """Process large dataset with progress reporting"""

    data = load_data(file_path)
    total_items = len(data)

    results = []
    for i, item in enumerate(data):
        # Report progress
        await ctx.report_progress(
            progress=i + 1,
            total=total_items,
            message=f"Processing item {i + 1} of {total_items}"
        )

        # Process item
        result = await process_item(item)
        results.append(result)

    return f"Processed {total_items} items"
```

### Resource Subscriptions

Allow clients to subscribe to resource changes:

```python
from mcp.types import ResourceUpdate

# Track subscribers
resource_subscribers = {}

@mcp.resource("data://live-feed")
async def get_live_feed() -> str:
    """Get current live feed data"""
    return json.dumps(current_feed_data)

async def notify_resource_updated(uri: str):
    """Notify subscribers of resource update"""
    if uri in resource_subscribers:
        for client in resource_subscribers[uri]:
            await client.send_notification(
                "notifications/resources/updated",
                {"uri": uri}
            )

# In your data update logic
async def update_feed_data(new_data):
    global current_feed_data
    current_feed_data = new_data
    await notify_resource_updated("data://live-feed")
```

### Custom Transports

Implement custom transports for specialized needs:

```python
from mcp.types import Transport, JSONRPCMessage

class CustomTransport(Transport):
    """Custom transport implementation"""

    def __init__(self, connection):
        self.connection = connection
        self.onmessage = None
        self.onerror = None
        self.onclose = None

    async def start(self):
        """Start the transport"""
        await self.connection.connect()
        asyncio.create_task(self._receive_loop())

    async def send(self, message: JSONRPCMessage):
        """Send a message"""
        await self.connection.write(json.dumps(message))

    async def close(self):
        """Close the transport"""
        await self.connection.close()

    async def _receive_loop(self):
        """Receive messages"""
        try:
            while True:
                data = await self.connection.read()
                message = json.loads(data)
                if self.onmessage:
                    self.onmessage(message)
        except Exception as e:
            if self.onerror:
                self.onerror(e)
        finally:
            if self.onclose:
                self.onclose()
```

---

## Security Considerations

### Authentication and Authorization

MCP currently lacks a standardized authentication mechanism. Implementers must create their own solutions:

#### API Key Authentication

```python
import os
from mcp.types import McpError

API_KEY = os.environ.get("MCP_API_KEY")

def verify_api_key(request):
    """Verify API key in request"""
    provided_key = request.headers.get("X-API-Key")

    if not provided_key or provided_key != API_KEY:
        raise McpError(
            code=-32000,
            message="Unauthorized",
            data={"reason": "Invalid or missing API key"}
        )

@mcp.tool()
async def protected_operation(param: str, ctx) -> str:
    """Operation requiring authentication"""
    verify_api_key(ctx.request)
    # Proceed with operation
    pass
```

#### Token-Based Authentication

```python
import jwt
from datetime import datetime, timedelta

SECRET_KEY = os.environ.get("JWT_SECRET")

def generate_token(user_id: str) -> str:
    """Generate JWT token"""
    payload = {
        "user_id": user_id,
        "exp": datetime.utcnow() + timedelta(hours=24)
    }
    return jwt.encode(payload, SECRET_KEY, algorithm="HS256")

def verify_token(token: str) -> dict:
    """Verify and decode JWT token"""
    try:
        return jwt.decode(token, SECRET_KEY, algorithms=["HS256"])
    except jwt.ExpiredSignatureError:
        raise McpError(code=-32000, message="Token expired")
    except jwt.InvalidTokenError:
        raise McpError(code=-32000, message="Invalid token")
```

### Input Validation

Always validate and sanitize inputs:

```python
import re
from pathlib import Path

def validate_file_path(path: str, allowed_dir: str) -> Path:
    """Validate file path to prevent directory traversal"""

    # Resolve to absolute path
    full_path = Path(allowed_dir) / path
    resolved = full_path.resolve()

    # Ensure it's within allowed directory
    if not str(resolved).startswith(str(Path(allowed_dir).resolve())):
        raise ValueError("Path outside allowed directory")

    return resolved

def sanitize_sql_identifier(identifier: str) -> str:
    """Sanitize SQL identifier"""

    # Only allow alphanumeric and underscore
    if not re.match(r'^[a-zA-Z_][a-zA-Z0-9_]*$', identifier):
        raise ValueError("Invalid SQL identifier")

    return identifier
```

### Preventing Command Injection

Never pass unsanitized input to shell commands:

```python
import shlex
import subprocess

@mcp.tool()
async def run_safe_command(file_path: str) -> str:
    """Run command safely"""

    # Validate input
    validated_path = validate_file_path(file_path, "/allowed/dir")

    # Use list form (not shell=True)
    result = subprocess.run(
        ["cat", str(validated_path)],
        capture_output=True,
        text=True,
        timeout=10
    )

    return result.stdout
```

### Rate Limiting

Implement rate limiting to prevent abuse:

```python
from collections import defaultdict
import time

class RateLimiter:
    def __init__(self, requests_per_minute: int):
        self.limit = requests_per_minute
        self.requests = defaultdict(list)

    def check_limit(self, client_id: str) -> bool:
        """Check if client has exceeded rate limit"""
        now = time.time()
        minute_ago = now - 60

        # Remove old requests
        self.requests[client_id] = [
            req_time for req_time in self.requests[client_id]
            if req_time > minute_ago
        ]

        # Check limit
        if len(self.requests[client_id]) >= self.limit:
            return False

        self.requests[client_id].append(now)
        return True

rate_limiter = RateLimiter(requests_per_minute=60)

@mcp.tool()
async def rate_limited_tool(param: str, ctx) -> str:
    """Tool with rate limiting"""
    client_id = ctx.client_id

    if not rate_limiter.check_limit(client_id):
        raise McpError(
            code=-32000,
            message="Rate limit exceeded",
            data={"retry_after": 60}
        )

    # Proceed with operation
    pass
```

### Data Privacy

Protect sensitive data:

```python
import hashlib
import json

def anonymize_data(data: dict) -> dict:
    """Anonymize sensitive fields"""
    sensitive_fields = ["email", "phone", "ssn", "credit_card"]

    anonymized = data.copy()
    for field in sensitive_fields:
        if field in anonymized:
            # Hash sensitive data
            value = str(anonymized[field])
            hashed = hashlib.sha256(value.encode()).hexdigest()[:16]
            anonymized[field] = f"***{hashed}"

    return anonymized

@mcp.tool()
async def get_user_analytics(user_id: str) -> str:
    """Get user analytics (anonymized)"""
    data = await fetch_user_data(user_id)
    anonymized = anonymize_data(data)
    return json.dumps(anonymized)
```

### Secure Communication

For remote servers, use TLS:

```python
import ssl

# SSE with TLS
ssl_context = ssl.create_default_context(ssl.Purpose.CLIENT_AUTH)
ssl_context.load_cert_chain("cert.pem", "key.pem")

mcp.run(
    transport="sse",
    host="0.0.0.0",
    port=443,
    ssl_context=ssl_context
)
```

### Security Checklist

- [ ] Implement authentication for remote servers
- [ ] Validate and sanitize all inputs
- [ ] Use parameterized queries for databases
- [ ] Prevent command injection
- [ ] Implement rate limiting
- [ ] Anonymize sensitive data
- [ ] Use TLS for remote connections
- [ ] Implement proper error handling (don't leak info)
- [ ] Log security events
- [ ] Regular security audits
- [ ] Keep dependencies updated
- [ ] Follow principle of least privilege
- [ ] Implement timeout handling
- [ ] Validate file paths
- [ ] Use secure random number generation

---

## Resources and References

### Official Documentation

- **MCP Documentation**: https://modelcontextprotocol.io/
- **Claude MCP Guide**: https://docs.claude.com/en/docs/agents-and-tools/mcp
- **MCP Specification**: https://modelcontextprotocol.io/specification
- **Anthropic Blog**: https://www.anthropic.com/news/model-context-protocol

### GitHub Repositories

- **MCP Organization**: https://github.com/modelcontextprotocol
- **Python SDK**: https://github.com/modelcontextprotocol/python-sdk
- **TypeScript SDK**: https://github.com/modelcontextprotocol/typescript-sdk
- **Official Servers**: https://github.com/modelcontextprotocol/servers
- **MCP Inspector**: https://github.com/modelcontextprotocol/inspector
- **Server Registry**: https://github.com/modelcontextprotocol/registry

### Learning Resources

- **Anthropic Course**: https://anthropic.skilljar.com/introduction-to-model-context-protocol
- **Microsoft MCP for Beginners**: https://github.com/microsoft/mcp-for-beginners
- **Awesome MCP Servers**: https://github.com/punkpeye/awesome-mcp-servers

### Community Resources

- **MCP Registry**: Community registry of MCP servers
- **GitMCP**: https://gitmcp.io/ - Generate MCP servers for GitHub projects
- **MCPcat**: https://mcpcat.io/ - MCP guides and resources

### SDK Documentation

**Python:**
- Package: `mcp`
- Install: `pip install mcp "mcp[cli]"`
- Framework: FastMCP for rapid development

**TypeScript:**
- Package: `@modelcontextprotocol/sdk`
- Install: `npm install @modelcontextprotocol/sdk`

**Java:**
- Package: `spring-ai-starter-mcp-server`
- Requires: Spring Boot 3.3+

**C#:**
- Package: `ModelContextProtocol`
- Requires: .NET 8+

**Other Languages:**
- Go: https://github.com/modelcontextprotocol/go-sdk
- Kotlin: https://github.com/modelcontextprotocol/kotlin-sdk
- Rust: https://github.com/modelcontextprotocol/rust-sdk
- Swift: https://github.com/modelcontextprotocol/swift-sdk
- PHP: https://github.com/modelcontextprotocol/php-sdk
- Ruby: https://github.com/modelcontextprotocol/ruby-sdk

### Example Servers

**Official Examples:**
- Filesystem server
- GitHub integration
- Google Drive
- Slack
- PostgreSQL
- Puppeteer

**Community Examples:**
- Weather APIs
- Calculator tools
- Documentation search
- Database queries
- Docker management
- Browser automation

### Tools and Utilities

- **MCP Inspector**: Interactive testing tool
- **MCP Proxy**: Convert stdio to SSE/WebSocket
- **FastMCP**: Python rapid development framework

### Support and Community

- **GitHub Discussions**: Ask questions and share ideas
- **Issue Trackers**: Report bugs and request features
- **Community Registry**: Discover and share servers

---

## Conclusion

The Model Context Protocol represents a significant advancement in how AI applications connect to data and tools. By providing a standardized, open protocol, MCP eliminates the need for custom integrations and enables developers to build once and deploy everywhere.

### Key Takeaways

1. **Universal Connectivity**: MCP is like USB-C for AI—one protocol, many applications
2. **Three Primitives**: Tools, Resources, and Prompts cover most use cases
3. **Multiple Languages**: SDKs available for Python, TypeScript, Java, C#, and more
4. **Open Standard**: Community-driven with active development
5. **Security First**: Capability-based with user consent requirements
6. **Production Ready**: Used by major companies and platforms

### Getting Started

1. Choose your language (Python recommended for beginners)
2. Install the SDK
3. Build a simple server with one tool
4. Test with MCP Inspector
5. Integrate with Claude Desktop
6. Iterate and expand

### Next Steps

- Explore the official examples
- Join the community discussions
- Build your own MCP server
- Contribute to open-source servers
- Share your creations with the community

The MCP ecosystem is growing rapidly, and developers worldwide are building innovative integrations. Whether you're connecting to databases, APIs, file systems, or external tools, MCP provides the foundation for seamless AI integration.

Start building today and be part of the future of AI application development!

---

**Document Version**: 1.0
**Last Updated**: November 2025
**Character Count**: ~51,000+

---
