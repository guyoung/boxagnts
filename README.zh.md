<div align="center">

<img src="boxagnts-dashboard-web/assets/boxagnts.svg" alt="Boxagnts" width="120" />

<h1>Boxagnts</h1>
<h3><em>AI 驱动的编程助手，配备 Web 管理面板</em></h3>

</div>

---

Boxagnts 是一个专为安全高效的软件工程设计的 **AI 编程助手**。它提供了 **Web 管理面板**用于多会话聊天管理，**WASM 沙箱**用于安全的工具执行，**MCP 协议**集成用于连接外部工具，**多提供商 LLM 支持**（30+ 提供商），以及 **Cron 定时任务**。

后端采用 **Rust** 构建，保证性能与安全；前端管理面板基于 **Vue 3**。

---

## 系统架构

```
┌─────────────────────────────────────────────┐
│          Vue 3 管理面板 (Web UI)              │
│   聊天 | 文件 | 工具 | MCP | 定时 | 站点      │
└──────────────────┬──────────────────────────┘
                   │ HTTP / WebSocket
┌──────────────────▼──────────────────────────┐
│           boxagnts-server (Axum)            │
│         API 路由 + 管理面板静态资源            │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│           boxagnts-gateway                  │
│   聊天 | 工具 | MCP | 技能 | 定时 | 站点      │
└──────────────────┬──────────────────────────┘
                   │
     ┌─────────────┼─────────────┐
     ▼             ▼             ▼
┌─────────┐ ┌──────────┐ ┌──────────────┐
│ LLM API │ │  工具    │ │   工作空间    │
│  抽象层  │ │  管理器  │ │  (SQLite)    │
│ 30+     │ │ WASM +  │ │ 配置/历史    │
│ 提供商   │ │ 内置工具 │ │ 认证/权限    │
└─────────┘ └──────────┘ └──────────────┘
```

---

## 功能特性

### 核心能力
- **多会话聊天** — 通过 Web 管理面板同时管理多个 AI 编程对话
- **自主代理循环** — 完整的工具使用循环：查询 → 响应 → 工具调度 → 结果反馈 → 继续
- **自动压缩** — 当上下文窗口即将填满时，自动压缩对话历史
- **预算控制** — 基于美元的成本追踪，可配置消费上限
- **计划模式** — 先进入规划模式进行结构化推理，再执行代码修改

### LLM 提供商（30+）
Anthropic、OpenAI、Google Gemini、Azure OpenAI、AWS Bedrock、GitHub Copilot、Cohere、MiniMax、Ollama、DeepSeek、Groq、Mistral 等 —— 全部通过统一的抽象层接入。

### 工具
| 工具 | 描述 |
|------|------|
| **读取文件** | 读取文件并显示行号，支持图片（PNG/JPG）和 PDF |
| **写入文件** | 创建或覆写文件 |
| **编辑文件** | 精确字符串替换，带唯一性校验 |
| **文件搜索** | 快速文件模式匹配（Glob） |
| **命令执行** | 执行 Shell 命令 |
| **网页抓取** | 获取网页内容并转换为 Markdown |
| **JS 沙箱执行** | 在沙箱环境中执行 JavaScript |
| **询问用户** | 代理执行过程中与用户交互确认 |
| **技能** | 加载并应用技能定义以执行专业任务 |

所有文件操作和执行类工具均运行在 **基于 Wasmtime 的沙箱**中，并支持网络访问控制。

### MCP 集成
完整的 **模型上下文协议**（Model Context Protocol）客户端实现：
- JSON-RPC 2.0 传输，支持 stdio 和 HTTP/SSE
- 工具发现（`tools/list`）与执行（`tools/call`）
- 资源管理（`resources/list`、`resources/read`）
- 提示词模板（`prompts/list`、`prompts/get`）
- 连接管理器，支持指数退避重连
- OAuth 支持，可连接需要认证的 MCP 服务器

### 技能系统
通过 Markdown Frontmatter 定义可复用的技能配置：

```yaml
---
name: code-review
description: 对变更文件进行深入代码审查
tools: read, bash, glob, grep
args:
  - name: target
    description: 待审查的文件或目录，留空则审查 git 暂存变更
    required: false
---
```

内置技能：**代码审查**、**CSS 重构顾问**、**前端组件生成器**、**天气预报**、**当前天气**。

### 定时任务
使用 Cron 表达式调度周期性代理任务 —— 让 AI 工作流按计划自动运行。

### 静态站点
直接从工作空间部署并托管静态网站。

### Web 管理面板
功能完备的 Vue 3 单页应用，采用 Vuetify Material Design 组件库：
- 通过 WebSocket 实时流式传输聊天响应
- 文件浏览器，内置代码编辑器（CodeMirror）和图片预览
- 工具和技能的开启/关闭配置管理
- MCP 服务器连接管理
- 使用 Chart.js 可视化呈现用量统计
- 定时任务调度界面

---

## 项目结构

```
boxagnts-pub/
├── Cargo.toml                      # Rust 工作空间根配置
│
├── boxagnts/                       # Rust 后端 crates
│   ├── api/                        # LLM API 抽象层（多提供商）
│   ├── core/                       # 核心类型、错误、常量
│   ├── gateway/                    # API 网关、聊天编排、定时任务、站点
│   ├── mcp/                        # MCP 客户端（JSON-RPC、SSE、stdio）
│   ├── query/                      # 核心代理查询循环
│   ├── server/                     # Axum Web 服务器 + 管理面板 API
│   ├── tools/                      # 内置工具实现
│   ├── tools-manager/              # 工具注册与调度
│   ├── wasm-sandbox/               # 基于 Wasmtime 的沙箱运行时
│   ├── wasm-tools/                 # WASM 工具抽象层
│   └── workspace/                  # 配置、历史、认证、权限
│
├── boxagnts-dashboard-web/         # Vue 3 前端单页应用
│   └── src/
│       ├── api/                    # HTTP/WebSocket API 通信层
│       ├── components/             # 侧边栏、代码编辑器、文件树、图片预览
│       ├── router/                 # Vue Router（10 条路由）
│       ├── stores/                 # Pinia 状态管理（10 个领域 Store）
│       ├── types/                  # TypeScript 类型定义
│       └── views/                  # 页面组件
│
├── app/
│   ├── dashboard-web/              # 前端构建产物（静态部署）
│   └── extensions/                 # 运行时扩展
│       ├── services/               # WASM 服务组件
│       ├── skills/                 # 技能定义（Markdown 格式）
│       └── tools/                  # WASM 工具组件（7 个工具）
│
└── src/lib.rs                      # 根 crate
```

---

## 快速开始

### 环境要求
- **Rust** 1.82+（edition 2024）
- **Node.js** 18+（用于构建前端管理面板）
- 任一受支持 LLM 提供商的 API 密钥

### 快速启动

```bash
# 克隆仓库
git clone <仓库地址>
cd boxagnts-pub

# 构建前端管理面板
cd boxagnts-dashboard-web
npm install
npm run build
cd ..

# 编译并启动服务端
cargo build --release --package boxagnts-server

# 设置 API 密钥
set ANTHROPIC_API_KEY=sk-ant-...    # Windows
# export ANTHROPIC_API_KEY=sk-ant-...   # Linux/macOS

# 启动服务
./target/release/Boxagnts --port 30001

# 打开管理面板
# http://127.0.0.1:30001/dashboard
```

### 命令行参数

```
Boxagnts — AI 编程助手 Web 服务器

参数：
  --port PORT          监听的端口号（默认：30001）
  --host HOST          绑定的主机地址（默认：127.0.0.1）
  --workspace-dir DIR  设置工作空间目录（默认：当前目录）
  --app-dir DIR        设置应用目录（默认：可执行文件所在目录）
  --admin-user USER    设置管理员用户名
  --admin-pass PASS    设置管理员密码
```

### 开发模式构建前端

```bash
cd boxagnts-dashboard-web
npm install
npm run dev          # 启动开发服务器（带热更新）
npm run build        # 生产构建 → app/dashboard-web/
```

---

## 管理面板页面

| 页面 | 路由 | 功能描述 |
|------|------|----------|
| **聊天** | `/dashboard/#/` | AI 对话，支持流式响应与多会话管理 |
| **用量** | `/dashboard/#/usage` | Token 使用量与费用统计 |
| **MCP** | `/dashboard/#/mcp` | MCP 服务器连接管理 |
| **文件** | `/dashboard/#/files` | 工作空间文件浏览器与编辑器 |
| **站点** | `/dashboard/#/sites` | 静态站点部署管理 |
| **定时任务** | `/dashboard/#/crons` | 定时任务配置与管理 |
| **代理** | `/dashboard/#/agents` | 代理与模型配置 |
| **技能** | `/dashboard/#/skills` | 技能定义管理 |
| **工具** | `/dashboard/#/tools` | 工具启用/禁用与配置 |
| **设置** | `/dashboard/#/settings` | 系统配置与偏好设置 |

---

## 核心设计决策

### 洁净室架构
LLM API 层采用分阶段抽象设计（阶段 1A–6），逐步构建提供商无关的接口，确保不耦合任何单一提供商的实现细节。

### WASM 沙箱化
所有文件操作和命令执行均通过 Wasmtime 运行，提供：
- 内存隔离和基于能力的安全模型
- 网络访问白名单/黑名单控制
- 语言无关的工具开发（任何能编译为 WASM 的语言均可）

### 自主代理循环
查询循环实现了完整的自主代理工作流：
1. 发送消息 → LLM
2. 处理流式 SSE 响应
3. 检测 `tool_use` 块 → 调度工具执行
4. 将工具结果反馈 → 循环直到 `end_turn` 或达到上限

支持**托管编排器**模式（实验性），可实现 Manager-Executor 多代理协作模式。

---

## 技术栈

| 层级 | 技术 |
|------|------|
| **开发语言** | Rust（edition 2024） |
| **Web 框架** | Axum 0.8 + Tokio |
| **数据库** | SQLite（rusqlite，bundled） |
| **WASM 运行时** | Wasmtime |
| **前端框架** | Vue 3 + TypeScript |
| **UI 组件库** | Vuetify 3（Material Design） |
| **状态管理** | Pinia |
| **构建工具** | Vite 6 |
| **代码编辑器** | CodeMirror 6 |
| **图表绘制** | Chart.js + vue-chartjs |

---

## 许可证

本项目是基于 CLAURST 开源项目的衍生作品。详见 [LICENSE.md](claurst-LICENSE.md)。

---

## 致谢

- **CLAURST** — Boxagnts 所基于的开源 Claude Code Rust 重实现
- **Wasmtime** — Bytecode Alliance 的 WebAssembly 运行时
- **MCP** — Anthropic 的模型上下文协议
