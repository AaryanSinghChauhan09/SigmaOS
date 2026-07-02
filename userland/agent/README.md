# sigma-agent — AI CLI Agent for SigmaOS

> Do everything the Zenith Desktop GUI can do, from the terminal.
> Inspired by Claude Code, Aider, Hermes IDE.

---

## What it does

`sigma-agent` is an AI-powered CLI agent that maps natural language commands to
SigmaOS operations. Every GUI capability has a CLI equivalent:

| GUI Action | sigma-agent command |
|---|---|
| Open Settings → Appearance → Dark Mode | `sigma-agent "set dark mode"` |
| Open Files app → browse /home | `sigma-agent "list /home/user"` |
| Click Install in App Store | `sigma-agent "install sigma-edit"` |
| Launch sigma-terminal | `sigma-agent "open app sigma-terminal"` |
| Network Settings → Wi-Fi | `sigma-agent "connect wifi MyNetwork secret"` |
| System Monitor | `sigma-agent "system info"` |
| Accessibility → High Contrast | `sigma-agent "high contrast on"` |
| VPN → Connect | `sigma-agent "vpn connect work-vpn"` |
| Desktop → Switch Workspace 2 | `sigma-agent "switch workspace 2"` |
| Notifications | `sigma-agent "notify 'Build done' --body 'Rust compiled'"` |
| Clipboard | `sigma-agent "copy Hello world"` |
| Find Files | `sigma-agent "find sigma_net in /home/user/code"` |
| View file | `sigma-agent "read README.md"` |
| Edit file with AI | `sigma-agent "fix src/main.rs add error handling"` |

---

## Architecture

```
User Input (natural language)
       │
       ▼
  IntentParser (Nim)
  - Pattern matching on keywords
  - Extracts tool name + args
       │
       ▼
  Agent Core (Rust)
  - Tool registry (20 built-in tools)
  - Agentic loop (multi-step tasks)
  - Conversation history (50 turns)
       │
       ▼
  Tool Executor
  - Each tool = struct implementing Tool trait
  - Returns ToolResult { success, output, next }
       │
       ▼
  Output Formatter (Nim)
  - ANSI colours
  - Success/error styling
  - Multi-step progress
       │
       ▼
  sigma-ai (optional)
  - For explain/summarise/code-edit
  - Falls back gracefully when offline
```

---

## Available Tools (20)

| Tool | Aliases | What it does |
|---|---|---|
| `read_file` | cat, show, read | Read file content |
| `write_file` | write, save, create_file | Write/append to file |
| `list_dir` | ls, dir, list | List directory |
| `shell` | run, exec, bash | Execute shell command |
| `install_package` | install, add | Install via sigma-pkg |
| `open_app` | open, launch, start | Launch application |
| `settings` | config, set, get | Get/set settings |
| `system_info` | sysinfo, neofetch | System overview |
| `network` | net, wifi, netctl | Network management |
| `process` | ps, kill, top | Process management |
| `explain` | what, how, why | Ask sigma-ai to explain |
| `code_edit` | edit, fix, refactor | AI-assisted code editing |
| `summarise` | summary, tldr | Summarise with AI |
| `wm_control` | window, tile, workspace | Window manager control |
| `notify` | notification, toast | Send desktop notification |
| `clipboard` | copy, paste | Clipboard read/write |
| `find_files` | find, search, grep | Search files by name/content |
| `accessibility` | a11y, access | Toggle accessibility features |
| `vpn` | wireguard | VPN connect/disconnect |
| `disk` | df, du, storage | Disk usage and management |

---

## Usage

```bash
# Interactive REPL
sigma-agent

# Single command
sigma-agent "install sigma-edit"
sigma-agent "set dark mode"
sigma-agent "system info"

# Run a script
sigma-agent --script ~/setup.sa

# With flags
sigma-agent --verbose "find sigma_net.rs"
sigma-agent --no-color "list /usr/bin"
```

---

## Script Format (.sa files)

```bash
# sigma-agent script: dev-setup.sa
# Lines starting with # are comments

install sigma-edit
install sigma-terminal
set dark mode
notify "Setup complete" --body "Your dev environment is ready"
```

---

## Agentic Loop

For complex multi-step goals, sigma-agent breaks them down automatically:

```
σ> install sigma-edit and open it

  Planning multi-step task...

  Step 1: install sigma-edit
  ✓ Installed sigma-edit

  Step 2: open it
  ✓ Launched sigma-edit
```

---

## AI Integration

When `sigma-ai` is installed, these commands use the local LLM:

```
σ> explain what sigma_pledge does
σ> summarise /home/user/project/README.md
σ> fix src/kernel.rs add proper error handling
σ> what does sigma-pkg install do
```

Without sigma-ai, built-in explanations cover common SigmaOS topics.

---

## Files

```
userland/agent/
├── sigma_agent.rs            # 10 core tools (Rust)
├── sigma_agent_core.rs       # Agent loop, intent parser, REPL core
├── sigma_agent_tools_ext.rs  # 10 extended tools (AI, code, GUI mirror)
├── sigma_agent_main.nim      # CLI entry point, REPL, output formatting
└── README.md                 # This file
```

---

## Build

```bash
# Build Nim CLI
nim c -d:release -o:sigma-agent userland/agent/sigma_agent_main.nim

# Build Rust core (optional, improves accuracy)
cargo build --release -p sigma-agent-core

# Install
cp sigma-agent /usr/bin/
sigma-pkg install sigma-agent  # when published
```

---

*Inspired by Claude Code · Aider · Hermes IDE — built sovereign, no cloud required.*
