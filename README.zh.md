# BoxAgnts

[English](README.md) | [中文](README.zh.md)

BoxAgnts 是一个基于 Rust 构建的开源 AI Agent ToolBox，专注于开箱即用（Out of the Box）的极致体验。它利用 WebAssembly 沙箱（Sandbox）提供兼顾安全与灵活的运行环境，帮助用户轻松处理各种复杂任务，进而成为高效、可信赖的个人智能助理。

## 核心架构

### 🎯 AI Agent Tool**Box**

BoxAgnts 是一个功能完备的 AI Agent 工具箱，提供：

- **多模型支持**：兼容 OpenAI、Anthropic、CodeX、Google、Deepseek、MiniMax、OpenCode 等主流 AI 模型提供商
- **工具系统**：内置文件操作、Web 访问、代码执行等多种工具
- **技能系统**：通过简单的配置即可创建专用 AI 技能

### 🛡️ WebAssembly Sand**Box**

利用 WebAssembly 技术构建安全的运行环境：

- **隔离执行**：所有自定义工具和技能在 WASM 沙箱中运行
- **安全控制**：精细的权限管理和网络访问控制
- **跨平台**：一次编译，多平台运行
- **高性能**：基于 Wasmtime 运行时，接近原生性能

### ✨ Out of the **Box**

开箱即用的极致体验：

- **零配置启动**：下载即可运行，无需复杂配置
- **Web 界面**：内置美观的 Dashboard，可视化管理所有功能
- **内置扩展**：预置常用工具和技能，直接可用
- **快速上手**：简洁的 API 和直观的操作流程

## 主要功能

### 🤖 AI 对话与代理
- 与多种 AI 模型对话
- 创建和管理自定义 Agents
- 保存和管理对话历史
- 支持流式响应

### 🔧 工具执行
- 文件读写与编辑
- Shell 命令执行
- Web 内容抓取
- 代码审查与分析

### 📦 技能系统
- 快速创建专用技能
- 技能组合与复用
- 内置代码审查、天气查询、前端组件生成等技能

### ⏰ 自动任务 Cron
- 创建和管理定时任务
- 支持标准 Cron 表达式
- 任务执行日志与状态追踪
- 灵活的任务配置与触发方式

### 🌐 Web 服务
- 自定义网站部署
- 静态文件服务
- API 端点管理

## 快速上手

### 下载可执行文件

从 [Releases](https://github.com/guyoung/boxagnts/releases) 页面下载最新版本的压缩包文件，解压后即可运行。

### 启动服务

```bash
# 启动服务
boxagnts

# 指定工作空间目录
boxagnts --workspace-dir /path/to/workspace

# 指定端口
boxagnts --workspace-dir /path/to/workspace --port 30002
```

> 建议：BoxAgnts 支持多工作空间，每个工作空间都有自己的配置文件和数据目录，建议不要在默认目录下运行，而是指定一个工作空间目录，或者指定workspace-dir。

命令行参数：

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

### 访问 Dashboard

打开浏览器访问 `http://127.0.0.1:30001`

### 配置模型

在设置页面添加 AI 模型以及 API Key

## 项目结构与源代码编译

本项目是基于 [claurst](https://github.com/Kuberwastaken/claurst) 项目代码开发

### 目录结构

```
boxagnts/
├── boxagnts/                 # Rust 后端核心代码
│   ├── api/                 # AI 模型 API（多提供商支持）
│   ├── core/                # 核心类型、常量与基础功能
│   ├── gateway/             # API 网关（包含 Cron 任务调度）
│   ├── mcp/                 # MCP 协议实现（可选）
│   ├── server/              # Web 服务器与 Dashboard 接口
│   ├── tools/               # 工具系统与内置工具
│   ├── tools-manager/       # 工具管理器
│   ├── query/               # 查询编排
│   ├── wasm-sandbox/        # WebAssembly 沙箱运行时
│   ├── wasm-tools/          # WASM 工具封装
│   └── workspace/           # 工作空间与配置管理
├── boxagnts-dashboard-web/  # Vue 3 前端源码
│   ├── src/
│   │   ├── api/            # API 接口封装
│   │   ├── components/     # Vue 组件
│   │   ├── composables/    # 组合式函数
│   │   ├── stores/         # Pinia 状态管理
│   │   ├── views/          # 页面组件
│   │   └── router/         # 路由配置
│   └── package.json        # 前端依赖
├── app/                     # 应用资源
│   ├── dashboard-web/      # 编译后的 Web 界面静态资源
│   └── extensions/         # 扩展（工具/技能）
└── Cargo.toml              # Rust 工作区配置
```

### 后端代码分析

后端采用 Rust 语言开发，使用 Tokio 异步运行时，主要模块如下：

- **api/**：封装了 OpenAI、Anthropic、Google、Azure、Bedrock 等多个 AI 提供商的 API，提供统一的接口调用和消息格式转换
- **core/**：定义核心数据类型、常量、错误处理和系统提示词
- **gateway/**：API 网关层，处理 HTTP 请求，包含 Cron 任务调度系统（cron/ 子目录），支持定时任务的创建、管理和执行
- **server/**：Web 服务器，提供 Dashboard 的 REST API 和 WebSocket 支持
- **tools/**：工具系统，实现了内置工具和技能的执行框架
- **wasm-sandbox/**：基于 Wasmtime 的 WebAssembly 沙箱，实现安全的代码执行环境
- **workspace/**：工作空间管理，处理配置、认证和历史记录存储

### 前端代码分析

前端采用 Vue 3 + TypeScript + Vuetify 技术栈：

- 使用 **Pinia** 进行状态管理（stores/ 目录）
- 使用 **Vue Router** 进行路由管理（router/ 目录）
- 主要页面：聊天、Agents、Cron 任务、文件、技能、工具、网站、设置等
- 支持 Markdown 渲染、代码编辑器（CodeMirror）、图表（Chart.js）等功能
- 与后端通过 REST API 和 WebSocket 进行通信

### 源代码编译方法

#### 环境要求

- Rust 1.75+（安装：https://www.rust-lang.org/tools/install）
- Node.js 18+（安装：https://nodejs.org/）
- npm 或 pnpm

#### 编译后端

```bash
# 进入项目根目录
cd boxagnts-pub

# 编译 Debug 版本
cargo build

# 编译 Release 版本（优化体积和性能）
cargo build --release

# 编译后的可执行文件位于 target/release/boxagnts
```

#### 编译前端

```bash
# 进入前端目录
cd boxagnts-dashboard-web

# 安装依赖
npm install

# 开发模式启动（热重载）
npm run dev

# 编译生产版本
npm run build

# 编译后的静态文件会输出到 app/dashboard-web/
```

#### 完整构建流程

```bash
# 1. 编译前端
cd boxagnts-dashboard-web
npm install
npm run build

# 2. 编译后端
cd ..
cargo build --release

# 3. 运行
./target/release/boxagnts
```

## 许可证

[MIT](LICENSE)


---

**Repository**: [https://github.com/guyoung/boxagnts](https://github.com/guyoung/boxagnts)