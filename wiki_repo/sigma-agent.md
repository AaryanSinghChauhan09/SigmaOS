# sigma-agent — AI CLI Agent for SigmaOS

> Every GUI action, accessible from the terminal via natural language.
> Inspired by Claude Code · Aider · llama.cpp · ai-shell · copilot-cli · azure-cli · Hermes IDE.

---

## What it does

`sigma-agent` is SigmaOS's sovereign AI CLI agent. It maps natural language to OS operations — every setting, app, file, and system control you can click in the Zenith Desktop GUI can be done with a sentence.

```
σ ~/code › install sigma-edit and open it

  Planning multi-step task...

  Step 1: install sigma-edit
  ✓ Installed sigma-edit 1.2.0

  Step 2: open it
  ✓ Launched sigma-edit
```

---

## Architecture

```
User Input → Intent Parser → ReAct Planner → LLM Backend → Tool Executor → Output
               (Nim)            (Rust)         (auto-select)  (Rust/Nim)    (ANSI)

LLM Backends (priority order):
  1. sigma-ai daemon  → /run/sigma/ai.sock   (sovereign, always-on)
  2. Ollama HTTP API  → localhost:11434       (easy setup)
  3. llama.cpp CLI    → GGUF model file       (any model)
  4. Offline fallback → built-in responses   (no LLM needed)
```

**ReAct Loop** (Claude Code / Aider pattern):
```
Thought: [LLM reasons about what to do]
Action: tool_name
Args: key=value
Observation: [tool result]
... repeat until Final Answer
```

---

## Complete GUI → CLI Mapping

### Desktop / Window Manager
| GUI | CLI |
|---|---|
| Open Terminal | `sigma-agent "open app sigma-terminal"` |
| Open Files | `sigma-agent "open app sigma-files"` |
| Switch Workspace 2 | `sigma-agent "workspace 2"` |
| Tile Windows | `sigma-agent "tile"` |
| Fullscreen | `sigma-agent "fullscreen"` |
| Close Window | `sigma-agent "close window"` |
| Cycle Layout | `sigma-agent "cycle layout"` |

### Settings → Appearance
| GUI | CLI |
|---|---|
| Dark Mode | `sigma-agent "set dark mode"` |
| Light Mode | `sigma-agent "set light mode"` |
| High Contrast | `sigma-agent "set high contrast"` |
| Corner Radius | `sigma-agent "settings set appearance corner_radius 8"` |
| Disable Animations | `sigma-agent "settings set appearance animations false"` |

### Settings → Network
| GUI | CLI |
|---|---|
| View Interfaces | `sigma-netctl list` |
| Connect Wi-Fi | `sigma-netctl wifi <iface> <ssid> <pass>` |
| Set Static IP | `sigma-netctl static <iface> <ip> <mask> <gw>` |
| Change DNS | `sigma-netctl dns <server>` |
| Connect VPN | `sigma-vpn connect <profile>` |

### Settings → Accessibility
| GUI | CLI |
|---|---|
| High Contrast | `sigma-agent "accessibility high-contrast on"` |
| Screen Reader | `sigma-agent "accessibility screen-reader on"` |
| Reduce Motion | `sigma-agent "accessibility reduce-motion on"` |
| Large Text | `sigma-agent "accessibility large-text on"` |
| Colour Blind | `sigma-agent "accessibility colour-blind on"` |
| Sticky Keys | `sigma-agent "accessibility sticky-keys on"` |

### App Store
| GUI | CLI |
|---|---|
| Search | `sigma-pkg search <query>` |
| Install | `sigma-pkg install <name>` |
| Remove | `sigma-pkg remove <name>` |
| Update All | `sigma-pkg update` |
| List Installed | `sigma-pkg list` |

### System Monitor
| GUI | CLI |
|---|---|
| Overview | `sigma-agent "system info"` |
| Processes | `sigma-top -1` |
| Kill Process | `sigma-agent "kill process <pid>"` |
| Disk Usage | `sigma-disks list` |

---

## Usage

```bash
# Interactive REPL (like Claude Code)
sigma-agent

# Single command
sigma-agent "install sigma-edit"
sigma-agent "set dark mode"
sigma-agent "system info"

# Script file (.sa)
sigma-agent --script ~/setup.sa

# Pipe mode
echo "system info" | sigma-agent --pipe

# Preview only (dry-run)
sigma-agent --dry-run "install sigma-edit and configure it"

# Trust levels
sigma-agent --trust safe      # read-only
sigma-agent --trust standard  # default
sigma-agent --trust full      # all operations

# Watch files + AI suggestions (Aider-style)
sigma-agent watch . --suggest
sigma-agent watch /home/user/code --ext .rs,.nim

# Shell integration (like copilot-cli)
sigma-agent install --shell-integration
# Then: ai "your request", aifix file.rs, ai-dark, ai-sysinfo
```

---

## Tools (20 built-in)

| Tool | Aliases | Description |
|---|---|---|
| `read_file` | cat, show, read | Read file content |
| `write_file` | write, save | Write/append to file |
| `list_dir` | ls, dir, list | List directory |
| `shell` | run, exec, bash | Execute shell command |
| `install_package` | install, add | Install via sigma-pkg |
| `open_app` | open, launch | Launch application |
| `settings` | config, set, get | Get/set OS settings |
| `system_info` | sysinfo, neofetch | System overview |
| `network` | net, wifi, netctl | Network management |
| `process` | ps, kill, top | Process management |
| `explain` | what, how, why | AI explanation |
| `code_edit` | edit, fix, refactor | AI code editing (Aider-style) |
| `summarise` | summary, tldr | AI file summary |
| `wm_control` | window, tile, workspace | Window manager |
| `notify` | notification, toast | Desktop notification |
| `clipboard` | copy, paste | Clipboard operations |
| `find_files` | find, search, grep | Search files |
| `accessibility` | a11y, access | Accessibility toggles |
| `vpn` | wireguard | VPN management |
| `disk` | df, du, storage | Disk management |

---

## LLM Model Setup

```bash
# Install TinyLlama-1.1B (recommended, ~700MB)
sigma-pkg install sigma-ai

# Or use Ollama (easy setup)
curl -fsSL https://ollama.ai/install.sh | sh
ollama pull tinyllama

# Or use any GGUF model with llama.cpp
sigma-pkg install llama-cpp
# Place model at: ~/.cache/sigma/models/tinyllama.gguf
```

---

## Shell Integration

```bash
# Install (auto-detects sigma-sh/bash/zsh/fish)
sigma-agent install --shell-integration

# New shortcuts after integration:
ai "your request"    # natural language OS commands
ai-dark              # dark mode
ai-sysinfo           # system info
ai-procs             # processes
explain <command>    # AI explains
aifix <file>         # AI fixes errors
```

---

## Training & Fine-tuning

```bash
# Seed training dataset
sigma-agent train seed

# Rate interactions (improves future suggestions)
sigma-agent train rate good
sigma-agent train rate excellent

# Build fine-tuning dataset (ChatML + Alpaca JSONL)
sigma-agent train build sigma-agent-v1

# Use with llama.cpp fine-tuning
llama-finetune --model base.gguf --train datasets/sigma-agent-v1_chatml.jsonl
```

---

## Files

```
userland/agent/
├── sigma_agent.rs                  # 10 core tools (Rust)
├── sigma_agent_core.rs             # Intent parser + Agent + REPL
├── sigma_agent_tools_ext.rs        # 10 extended tools
├── sigma_llm.rs                    # LLM backends (llama.cpp/Ollama/sigma-ai)
├── sigma_agent_planner.rs          # ReAct planner + command suggestor
├── sigma_agent_code.rs             # Code editing + diff + git (Aider-style)
├── sigma_agent_session.nim         # Session manager + memory + streaming
├── sigma_agent_main.nim            # CLI entry point + REPL UI
├── sigma_agent_config.nim          # Profile system + model management
├── sigma_agent_training.nim        # Training data + fine-tuning pipeline
├── sigma_agent_gui_mirror.nim      # 60+ GUI→CLI complete mapping
├── sigma_agent_watch.nim           # File watcher + proactive AI suggestions
├── sigma_agent_shell_integration.nim # Shell hooks + keybindings + aliases
├── sigma_agent_ci.yml              # CI: build/lint/test/seed-dataset
├── sigma_agent.nimble              # Nim package definition
└── README.md                       # Full documentation
```

---

*Inspired by: [Claude Code](https://github.com/anthropics/claude-code) · [Aider](https://github.com/Aider-AI/aider) · [llama.cpp](https://github.com/ggml-org/llama.cpp) · [ai-shell](https://github.com/BuilderIO/ai-shell) · [copilot-cli](https://github.com/github/copilot-cli) · [azure-cli](https://github.com/Azure/azure-cli) · [Hermes IDE](https://github.com/hermes-hq/hermes-ide) · [openclaw](https://github.com/openclaw/openclaw)*
