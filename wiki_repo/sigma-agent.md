# sigma-agent — AI CLI Agent for SigmaOS

> Do everything the Zenith Desktop GUI can do, from the terminal.
> Inspired by Claude Code, Aider, llama.cpp, ai-shell, copilot-cli.

---

## Overview

`sigma-agent` is an AI-powered CLI agent that maps natural language to SigmaOS operations. Every GUI action has a CLI equivalent.

```
σ ~/code › install sigma-edit
  ⚡ install_package(name=sigma-edit)
  📋 Downloading sigma-edit 1.2.0...
  ✓ Installed sigma-edit

σ ~/code › set dark mode
  ✓ appearance.theme = zenith-dark

σ ~/code › system info
Σ SigmaOS v15.0 Zenith
CPU: SovereignCPU x86_64  Memory: 128MB / 512MB
```

---

## Architecture

```
User Input (natural language)
       │
       ▼
  Intent Parser (Nim)          ← keyword + pattern matching
       │
       ▼
  ReAct Planner (Rust)         ← Reason + Act loop (Claude Code pattern)
  │   ├── Thought              ← LLM reasoning step
  │   ├── Action               ← tool call
  │   └── Observation          ← tool result
  │         └── repeat until Final Answer
       │
       ▼
  LLM Backend (auto-select)    ← inspired by llama.cpp
  ├── sigma-ai daemon (IPC)    ← fastest, always-on
  ├── Ollama HTTP API          ← local, easy setup
  ├── llama.cpp CLI            ← any GGUF model
  └── Null (offline fallback)  ← built-in responses
       │
       ▼
  Tool Registry (20 tools)     ← Rust OOP (Tool trait)
       │
       ▼
  Output Renderer (Nim)        ← ANSI colours, streaming
```

---

## GUI → CLI Mapping

| GUI Action | sigma-agent command |
|---|---|
| Settings → Appearance → Dark Mode | `set dark mode` |
| Files → Browse /home | `list /home/user` |
| App Store → Install | `install sigma-edit` |
| Launch Terminal | `open app sigma-terminal` |
| Network → Wi-Fi Connect | `connect wifi MyNetwork password` |
| System Monitor | `system info` |
| Accessibility → High Contrast | `accessibility high-contrast on` |
| VPN → Connect | `vpn connect work-profile` |
| Desktop → Workspace 2 | `workspace 2` |
| Send Notification | `notify "Done" "Build complete"` |
| Window → Fullscreen | `fullscreen` |
| Find File | `find sigma_net.rs in /home/user` |
| View File | `read README.md` |
| Edit File (AI-assisted) | `fix src/main.rs add error handling` |
| Clipboard Copy | `copy Hello world` |
| Disk Usage | `disk usage` |

---

## Usage

```bash
# Interactive REPL (default)
sigma-agent

# Single command
sigma-agent "install sigma-edit"

# Script file (.sa extension)
sigma-agent --script ~/setup.sa

# Pipe mode (stdin)
echo "system info" | sigma-agent --pipe

# Preview only (no writes)
sigma-agent --dry-run "install sigma-edit and open it"

# Use specific model
sigma-agent --model tinyllama "explain sigma_pledge"

# Trust levels
sigma-agent --trust safe      # read-only
sigma-agent --trust standard  # default
sigma-agent --trust full      # all operations
```

---

## LLM Backend Auto-Selection

```
1. sigma-ai daemon   → /run/sigma/ai.sock  (fastest, sovereign)
2. Ollama HTTP API   → localhost:11434      (easy setup)
3. llama.cpp CLI     → auto-detect binary   (any GGUF model)
4. Offline fallback  → built-in responses   (no LLM needed)
```

Install TinyLlama for full AI capability:
```bash
sigma-pkg install sigma-ai       # installs TinyLlama-1.1B + daemon
sigma-pkg install sigma-ai-code  # adds code-optimised variant
```

---

## Script Format

```bash
# setup.sa — sigma-agent script
# Lines starting with # are comments
install sigma-edit
install sigma-terminal
set dark mode
accessibility large-text on
notify "Setup complete" "Your dev environment is ready"
```

---

## Tools (20 built-in)

| Tool | What it does |
|---|---|
| `read_file` | Read file content |
| `write_file` | Write/append to file |
| `list_dir` | List directory |
| `shell` | Execute shell command |
| `install_package` | Install via sigma-pkg |
| `open_app` | Launch application |
| `settings` | Get/set OS settings |
| `system_info` | System overview |
| `network` | Interface management |
| `process` | Process list/kill |
| `explain` | Ask AI to explain |
| `code_edit` | AI code editing (Aider-style) |
| `summarise` | Summarise file with AI |
| `wm_control` | Window manager control |
| `notify` | Desktop notification |
| `clipboard` | Clipboard read/write |
| `find_files` | Search files |
| `accessibility` | A11y feature toggles |
| `vpn` | WireGuard VPN |
| `disk` | Disk usage/management |

---

## Files

```
userland/agent/
├── sigma_agent.rs              # 10 core tools (Rust)
├── sigma_agent_core.rs         # Agent + intent parser + REPL core
├── sigma_agent_tools_ext.rs    # 10 extended tools
├── sigma_llm.rs                # LLM backends (llama.cpp/Ollama/sigma-ai)
├── sigma_agent_planner.rs      # ReAct planner + command suggestor
├── sigma_agent_session.nim     # Session manager + streaming output
├── sigma_agent_main.nim        # CLI entry point + REPL UI
└── README.md                   # Full documentation
```

---

*Inspired by: [Claude Code](https://github.com/anthropics/claude-code) · [Aider](https://github.com/Aider-AI/aider) · [llama.cpp](https://github.com/ggml-org/llama.cpp) · [ai-shell](https://github.com/BuilderIO/ai-shell) · [copilot-cli](https://github.com/github/copilot-cli) · [Hermes IDE](https://github.com/hermes-hq/hermes-ide)*
