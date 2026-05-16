<div align="center">

<img src="boxagnts-dashboard-web/assets/boxagnts.svg" alt="Boxagnts" width="120" />

<h1>Boxagnts</h1>
<h3><em>AI-Powered Coding Assistant with Web Dashboard</em></h3>

</div>

---

Boxagnts is an **AI coding assistant** designed for safe and effective software engineering. It features a **Web Dashboard** for multi-session chat management, a **WASM sandbox** for secure tool execution, **MCP protocol** integration for external tools, **multi-provider LLM support** (30+ providers), and **cron-style scheduled tasks**.

Built in **Rust** for performance and safety, with a **Vue 3** frontend dashboard.

---

## Architecture

```
┌─────────────────────────────────────────────┐
│          Vue 3 Dashboard (Web UI)            │
│   Chat | Files | Tools | MCP | Cron | Sites  │
└──────────────────┬──────────────────────────┘
                   │ HTTP / WebSocket
┌──────────────────▼──────────────────────────┐
│           boxagnts-server (Axum)            │
│        API Routes + Dashboard Static        │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│           boxagnts-gateway                  │
│   Chat | Tools | MCP | Skills | Cron | Site │
└──────────────────┬──────────────────────────┘
                   │
     ┌─────────────┼─────────────┐
     ▼             ▼             ▼
┌─────────┐ ┌──────────┐ ┌──────────────┐
│ LLM API │ │  Tools   │ │   Workspace  │
│ Layer   │ │ Manager  │ │  (SQLite)    │
│ 30+     │ │ WASM +   │ │ Config/Hist  │
│ Provider│ │ Built-in │ │ Auth/Perms   │
└─────────┘ └──────────┘ └──────────────┘
```

---

## Features

### Core Capabilities
- **Multi-Session Chat** — Manage multiple AI coding conversations simultaneously with the Web Dashboard
- **Agentic Loop** — Full tool-use cycle: query → response → tool dispatch → result feedback → continue
- **Auto Compact** — Automatic context window compression when limits are reached
- **Budget Control** — USD-based cost tracking with configurable spending limits
- **Plan Mode** — Enter plan mode for structured reasoning before making changes

### LLM Providers (30+)
Anthropic, OpenAI, Google Gemini, Azure OpenAI, AWS Bedrock, GitHub Copilot, Cohere, MiniMax, Ollama, DeepSeek, Groq, Mistral, and many more — all through a unified abstraction layer.

### Tools
| Tool | Description |
|------|-------------|
| **Read** | Read files with line numbers, supports images (PNG/JPG) and PDF |
| **Write** | Create or overwrite files |
| **Edit** | Exact string replacement with uniqueness validation |
| **Glob** | Fast file pattern matching |
| **Bash** | Execute shell commands |
| **Web Fetch** | Retrieve and convert web content |
| **BoxedJS Execute** | Execute JavaScript in a sandboxed environment |
| **Ask User** | Interactive user prompts during agent execution |
| **Skill** | Load and apply skill definitions for specialized tasks |

All file and execution tools run inside a **Wasmtime-based sandbox** with network access control.

### MCP Integration
Full **Model Context Protocol** (MCP) client implementation:
- JSON-RPC 2.0 transport with stdio and HTTP/SSE
- Tool discovery (`tools/list`) and execution (`tools/call`)
- Resource management (`resources/list`, `resources/read`)
- Prompt templates (`prompts/list`, `prompts/get`)
- Connection manager with exponential-backoff reconnection
- OAuth support for authenticated MCP servers

### Skills System
Define reusable skill configurations via Markdown frontmatter:

```yaml
---
name: code-review
description: Perform an in-depth code review of changed files
tools: read, bash, glob, grep
args:
  - name: target
    description: File or directory to review
    required: false
---
```

Built-in skills: **Code Review**, **CSS Refactor Advisor**, **Frontend Component Generator**, **Weather Forecast**, **Current Weather**.

### Cron Jobs
Schedule recurring agent tasks with cron expressions — runs AI-powered workflows on a timer.

### Static Sites
Deploy and serve static sites directly from your workspace.

### Web Dashboard
Full-featured Vue 3 SPA with Vuetify Material Design:
- Real-time chat with streaming responses via WebSocket
- File browser with code editor (CodeMirror) and image preview
- Tool and skill configuration management
- MCP server connection management
- Usage statistics with Chart.js visualizations
- Cron job scheduling interface

---

## Project Structure

```
boxagnts-pub/
├── Cargo.toml                      # Rust workspace root
│
├── boxagnts/                       # Rust backend crates
│   ├── api/                        # LLM API abstraction (multi-provider)
│   ├── core/                       # Core types, errors, constants
│   ├── gateway/                    # API gateway, chat orchestrator, cron, sites
│   ├── mcp/                        # MCP client (JSON-RPC, SSE, stdio)
│   ├── query/                      # Agentic query loop core
│   ├── server/                     # Axum web server + Dashboard APIs
│   ├── tools/                      # Built-in tool implementations
│   ├── tools-manager/              # Tool registry and dispatch
│   ├── wasm-sandbox/               # Wasmtime-based sandbox runtime
│   ├── wasm-tools/                 # WASM tool abstraction layer
│   └── workspace/                  # Config, history, auth, permissions
│
├── boxagnts-dashboard-web/         # Vue 3 Frontend SPA
│   └── src/
│       ├── api/                    # HTTP/WebSocket API layer
│       ├── components/             # Sidebar, CodeEditor, FileTree, ImagePreview
│       ├── router/                 # Vue Router (10 routes)
│       ├── stores/                 # Pinia state (10 domain stores)
│       ├── types/                  # TypeScript type definitions
│       └── views/                  # Page components
│
├── app/
│   ├── dashboard-web/              # Built frontend (static deployment)
│   └── extensions/                 # Runtime extensions
│       ├── services/               # WASM service components
│       ├── skills/                 # Skill definitions (Markdown)
│       └── tools/                  # WASM tool components (7 tools)
│
└── src/lib.rs                      # Root crate
```

---

## Getting Started

### Prerequisites
- **Rust** 1.82+ (edition 2024)
- **Node.js** 18+ (for building the dashboard frontend)
- An API key from a supported LLM provider

### Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd boxagnts-pub

# Build the frontend dashboard
cd boxagnts-dashboard-web
npm install
npm run build
cd ..

# Build and run the server
cargo build --release --package boxagnts-server

# Set your API key
export ANTHROPIC_API_KEY=sk-ant-...

# Start the server
./target/release/Boxagnts --port 30001

# Open the dashboard
# http://127.0.0.1:30001/dashboard
```

### CLI Options

```
Boxagnts -- AI coding assistant web server

Options:
  --port PORT          Port to run the web server on (default: 30001)
  --host HOST          Host to bind to (default: 127.0.0.1)
  --workspace-dir DIR  Set workspace directory (default: current dir)
  --app-dir DIR        Set app directory (default: executable dir)
  --admin-user USER    Set admin username
  --admin-pass PASS    Set admin password
```

### Build the Dashboard (Development)

```bash
cd boxagnts-dashboard-web
npm install
npm run dev          # Development server with HMR
npm run build        # Production build → app/dashboard-web/
```

---

## Dashboard Pages

| Page | Route | Description |
|------|-------|-------------|
| **Chat** | `/dashboard/#/` | AI conversation with streaming, session management |
| **Usage** | `/dashboard/#/usage` | Token usage and cost statistics |
| **MCP** | `/dashboard/#/mcp` | MCP server connection management |
| **Files** | `/dashboard/#/files` | Workspace file browser with editor |
| **Sites** | `/dashboard/#/sites` | Static site deployment management |
| **Crons** | `/dashboard/#/crons` | Scheduled task configuration |
| **Agents** | `/dashboard/#/agents` | Agent and model configuration |
| **Skills** | `/dashboard/#/skills` | Skill definition management |
| **Tools** | `/dashboard/#/tools` | Tool enable/disable and configuration |
| **Settings** | `/dashboard/#/settings` | System configuration and preferences |

---

## Key Design Decisions

### Clean-Room Architecture
The LLM API layer uses a phased abstraction design (Phases 1A–6) that progressively builds provider-agnostic interfaces, ensuring no coupling to any single provider's implementation.

### WASM Sandboxing
All file operations and command execution run through Wasmtime, providing:
- Memory isolation and capability-based security
- Network access whitelist/blacklist control
- Language-agnostic tool development (any language compiling to WASM)

### Agentic Loop
The query loop implements a full autonomous agent cycle:
1. Send messages → LLM
2. Process streaming SSE response
3. Detect `tool_use` blocks → dispatch tools
4. Feed tool results back → continue until `end_turn` or limit

Supports **Managed Orchestrator** mode (experimental) for Manager-Executor multi-agent patterns.

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| **Language** | Rust (edition 2024) |
| **Web Framework** | Axum 0.8 + Tokio |
| **Database** | SQLite (rusqlite, bundled) |
| **WASM Runtime** | Wasmtime |
| **Frontend Framework** | Vue 3 + TypeScript |
| **UI Library** | Vuetify 3 (Material Design) |
| **State Management** | Pinia |
| **Build Tool** | Vite 6 |
| **Code Editor** | CodeMirror 6 |
| **Charts** | Chart.js + vue-chartjs |

---

## License

This project is a derivative work based on the CLAURST open-source project. See [LICENSE.md](claurst-LICENSE.md) for details.

---

## Acknowledgments

- **CLAURST** — The open-source Claude Code reimplementation that Boxagnts is built upon
- **Wasmtime** — Bytecode Alliance's WebAssembly runtime
- **MCP** — Anthropic's Model Context Protocol
