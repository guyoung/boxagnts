# BoxAgnts

[English](README.md) | [中文](README.zh.md)

BoxAgnts is an open-source AI Agent ToolBox built with Rust, dedicated to delivering an ultimate out-of-the-box experience. Leveraging WebAssembly sandbox, it provides a runtime environment that balances security and flexibility, helping users effortlessly tackle a wide range of complex tasks and thus becoming an efficient and trustworthy personal AI assistant.

## Core Architecture

### 🎯 AI Agent Tool**Box**

BoxAgnts is a fully-featured AI Agent toolkit providing:

- **Multi-model support**: Compatible with major AI model providers including OpenAI, Anthropic, CodeX, Google, Deepseek, MiniMax, OpenCode
- **Tool system**: Built-in file operations, web access, code execution, and many other tools
- **Skill system**: Create specialized AI skills through simple configuration

### 🛡️ WebAssembly Sand**Box**

Build a secure runtime environment using WebAssembly technology:

- **Isolated execution**: All custom tools and skills run in a WASM sandbox
- **Security control**: Fine-grained permission management and network access control
- **Cross-platform**: Compile once, run everywhere
- **High performance**: Based on Wasmtime runtime, near-native performance

### ✨ Out of the **Box**

Out-of-the-box experience:

- **Zero-configuration startup**: Download and run, no complex configuration
- **Web interface**: Built-in beautiful Dashboard for visual management of all features
- **Built-in extensions**: Pre-configured with commonly used tools and skills, ready to use
- **Quick start**: Simple API and intuitive workflow

## Key Features

### 🤖 AI Chat and Agents
- Chat with multiple AI models
- Create and manage custom Agents
- Save and manage chat history
- Support for streaming responses

### 🔧 Tool Execution
- File read/write and editing
- Shell command execution
- Web content scraping
- Code review and analysis

### 📦 Skill System
- Quickly create specialized skills
- Skill combination and reuse
- Built-in skills including code review, weather query, front-end component generation, etc.

### ⏰ Automatic Tasks Cron
- Create and manage scheduled tasks
- Support for standard Cron expressions
- Task execution logs and status tracking
- Flexible task configuration and triggering methods

### 🌐 Web Service
- Custom website deployment
- Static file serving
- API endpoint management

## Quick Start

### Download Executable

Download the latest compressed package from the [Releases](https://github.com/guyoung/boxagnts/releases) page, extract and run.

### Start Service

```bash
# Start service
boxagnts

# Specify workspace directory
boxagnts --workspace-dir /path/to/workspace

# Specify port
boxagnts --workspace-dir /path/to/workspace --port 30002
```

> Suggestion: BoxAgnts supports multiple workspaces, each with its own configuration file and data directory. It is recommended not to run in the default directory, but to specify a workspace directory or workspace-dir.

Command line arguments:

```bash
BoxAgnts is an open-source AI Agent ToolBox built with Rust.

Usage: boxagnts [OPTIONS]

Options:
      --port <PORT>          Port to run the web server on [default: 30001]
      --host <HOST>          Host to bind to (0.0.0.0 for all interfaces) [default: 127.0.0.1]
      --workspace-dir <DIR>  Set workspace dir, default current dir
      --app-dir <DIR>        Set app dir, default Boxagnts executable file dir
      --admin-user <USERNAME>  Set admin username
      --admin-pass <PASSWORD>  Set admin password
  -h, --help                 Print help
  -V, --version              Print version
```

### Access Dashboard

Open your browser and visit `http://127.0.0.1:30001`

### Configure Model

Add AI models and API Keys in the settings page

## Project Structure and Source Code Compilation

This project is developed based on [claurst](https://github.com/Kuberwastaken/claurst) project code

### Directory Structure

```
boxagnts/
├── boxagnts/                 # Rust backend core code
│   ├── api/                 # AI model API (multi-provider support)
│   ├── core/                # Core types, constants, and basic functions
│   ├── gateway/             # API gateway (includes Cron task scheduling)
│   ├── mcp/                 # MCP protocol implementation (optional)
│   ├── server/              # Web server and Dashboard interface
│   ├── tools/               # Tool system and built-in tools
│   ├── tools-manager/       # Tool manager
│   ├── query/               # Query orchestration
│   ├── wasm-sandbox/        # WebAssembly sandbox runtime
│   ├── wasm-tools/          # WASM tool wrappers
│   └── workspace/           # Workspace and configuration management
├── boxagnts-dashboard-web/  # Vue 3 frontend source code
│   ├── src/
│   │   ├── api/            # API interface wrappers
│   │   ├── components/     # Vue components
│   │   ├── composables/    # Composables
│   │   ├── stores/         # Pinia state management
│   │   ├── views/          # Page components
│   │   └── router/         # Router configuration
│   └── package.json        # Frontend dependencies
├── app/                     # Application resources
│   ├── dashboard-web/      # Compiled web interface static assets
│   └── extensions/         # Extensions (tools/skills)
└── Cargo.toml              # Rust workspace configuration
```

### Backend Code Analysis

The backend is developed in Rust using Tokio async runtime. The main modules are:

- **api/**: Wraps APIs from multiple AI providers including OpenAI, Anthropic, Google, Azure, Bedrock, providing unified interface calling and message format conversion
- **core/**: Defines core data types, constants, error handling, and system prompts
- **gateway/**: API gateway layer, handles HTTP requests, includes Cron task scheduling system (cron/ subdirectory), supporting scheduled task creation, management, and execution
- **server/**: Web server, providing Dashboard REST API and WebSocket support
- **tools/**: Tool system, implements execution framework for built-in tools and skills
- **wasm-sandbox/**: WebAssembly sandbox based on Wasmtime, implementing secure code execution environment
- **workspace/**: Workspace management, handles configuration, authentication, and history storage

### Frontend Code Analysis

The frontend uses Vue 3 + TypeScript + Vuetify technology stack:

- Uses **Pinia** for state management (stores/ directory)
- Uses **Vue Router** for routing management (router/ directory)
- Main pages: Chat, Agents, Cron tasks, Files, Skills, Tools, Sites, Settings, etc.
- Supports Markdown rendering, code editor (CodeMirror), charts (Chart.js), etc.
- Communicates with backend via REST API and WebSocket

### Source Code Compilation Method

#### Environment Requirements

- Rust 1.75+ (Install: https://www.rust-lang.org/tools/install)
- Node.js 18+ (Install: https://nodejs.org/)
- npm or pnpm

#### Compile Backend

```bash
# Enter project root directory
cd boxagnts-pub

# Compile Debug version
cargo build

# Compile Release version (optimize for size and performance)
cargo build --release

# Compiled executable is located at target/release/boxagnts
```

#### Compile Frontend

```bash
# Enter frontend directory
cd boxagnts-dashboard-web

# Install dependencies
npm install

# Start development mode (hot reload)
npm run dev

# Compile production version
npm run build

# Compiled static files will be output to app/dashboard-web/
```

#### Complete Build Process

```bash
# 1. Compile frontend
cd boxagnts-dashboard-web
npm install
npm run build

# 2. Compile backend
cd ..
cargo build --release

# 3. Run
./target/release/boxagnts
```

## License

[MIT](LICENSE)


---

**Repository**: [https://github.com/guyoung/boxagnts](https://github.com/guyoung/boxagnts)
