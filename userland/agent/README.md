# sigma-agent — AI CLI Agent for SigmaOS

> Every GUI action, accessible from the terminal via natural language.
> Sovereign. Local. No cloud required.

---

## What it does

`sigma-agent` is SigmaOS's native AI CLI agent. It maps natural language to OS operations — every setting, app, file, and system control you can click in the Zenith Desktop GUI has a CLI equivalent. It runs locally, learns from your feedback, scans for security issues, and speaks to specialised sub-agents for complex tasks.

```
σ ~/code › install sigma-edit and open it

  Planning multi-step task...

  Step 1: install sigma-edit      ✓ Installed sigma-edit 1.2.0
  Step 2: open sigma-edit         ✓ Launched sigma-edit
```

---

## Architecture

```
User Input (natural language / voice)
       │
       ▼
 sigma-agent (Nim CLI master entry)
  ├── IntentParser    — keyword → tool + args
  ├── ReAct Planner   — multi-step reasoning (Thought→Action→Observation)
  ├── LLM Backend     — auto-select: sigma-ai → Ollama → llama.cpp → offline
  ├── Tool Executor   — 21 built-in tools (Rust + Nim)
  ├── Context Engine  — live OS state injected into LLM prompts
  └── Multi-agent     — routes to specialist sub-agents
         ├── sigma-security   (audit, policies, threat detection)
         ├── sigma-sysadmin   (services, resources, maintenance)
         ├── sigma-developer  (code editing, git, build tools)
         ├── sigma-teacher    (OS concepts, interactive learning)
         ├── sigma-netops     (interfaces, VPN, DNS, firewall)
         └── sigma-pkgops     (packages, updates, registry)

Background:
  sigma-agent daemon  — Unix socket + HTTP API (port 11430)
                        GitHub wiki knowledge sync (hourly)
                        Live system context for all requests

Learning:
  sigma-agent learn   — records interactions → DPO pairs → LoRA fine-tune
```

---

## Quick Start

```bash
# Interactive REPL
sigma-agent

# One-shot command
sigma-agent "install sigma-edit"
sigma-agent "set dark mode"
sigma-agent "system info"

# Start background daemon (enables context + knowledge sync)
sigma-agent daemon start

# Security audit
sigma-agent security scan

# Voice command
sigma-agent voice

# Route to specialist
sigma-agent multi "explain how sigma_pledge works"
sigma-agent multi --agent developer "fix src/main.rs add error handling"
```

---

## All Subcommands

| Subcommand | What it does |
|---|---|
| *(no args)* | Interactive REPL |
| `"<command>"` | One-shot natural language command |
| `--script <file>` | Run a `.sa` script file |
| `--pipe` | Read commands from stdin |
| `mirror list` | Show all 60+ GUI→CLI mappings |
| `mirror run <action>` | Execute a GUI action via CLI |
| `watch [dir]` | File watcher with AI suggestions (Aider-style) |
| `train seed` | Write seed training dataset |
| `train build` | Build ChatML + Alpaca + DPO JSONL datasets |
| `config` | Show/edit configuration and profiles |
| **`daemon start`** | **Start background daemon (HTTP + Unix socket)** |
| **`daemon sync`** | **Force GitHub wiki knowledge sync** |
| **`context`** | **Snapshot live system context** |
| **`security scan`** | **Full security audit + score** |
| **`security logs`** | **Scan logs for anomalies** |
| **`security policies`** | **AI policy recommendations** |
| **`learn rate good`** | **Rate last interaction (RLHF feedback)** |
| **`learn correct "<answer>"`** | **Record the right answer (DPO pair)** |
| **`learn finetune`** | **LoRA fine-tune via llama.cpp** |
| **`multi <input>`** | **Auto-route to specialist sub-agent** |
| **`multi --agent developer`** | **Force a specific sub-agent** |
| **`multi diagnose <problem>`** | **Multi-agent collaborative diagnosis** |
| **`voice`** | **Voice-to-command (Whisper STT)** |
| **`voice --session`** | **Continuous hands-free voice session** |
| `install --shell-integration` | Install bash/zsh/fish/sigma-sh hooks |

---

## Files

```
userland/agent/
├── main.rs                         Rust binary entry (sigma-agent-core)
├── Cargo.toml                      Rust crate manifest
├── sigma_agent.rs                  10 core tools (Rust)
├── sigma_agent_core.rs             Intent parser + Agent + REPL
├── sigma_agent_tools_ext.rs        10 extended tools
├── sigma_llm.rs                    LLM backends (llama.cpp/Ollama/sigma-ai/null)
├── sigma_agent_planner.rs          ReAct planner + command suggestor
├── sigma_agent_code.rs             Code editing + diff + git (Aider-style)
├── sigma_agent_main.nim            CLI master entry + subcommand router
├── sigma_agent_session.nim         Session manager + memory + streaming
├── sigma_agent_config.nim          Profile system + model management
├── sigma_agent_training.nim        Seed + fine-tuning pipeline
├── sigma_agent_gui_mirror.nim      60+ GUI→CLI complete mapping
├── sigma_agent_watch.nim           File watcher + proactive AI suggestions
├── sigma_agent_shell_integration.nim  Shell hooks + keybindings + aliases
├── sigma_agent_daemon.nim          Background daemon (HTTP + socket + knowledge sync)
├── sigma_agent_context.nim         Live OS state context collection
├── sigma_agent_security.nim        Security advisor + anomaly scanner
├── sigma_agent_learn.nim           RLHF feedback + DPO fine-tuning pipeline
├── sigma_agent_multi.nim           Multi-agent orchestration + specialist routing
├── sigma_agent_voice.nim           Voice input (Whisper STT pipeline)
├── sigma_agent.nimble              Nim package definition
├── sigma_agent_ci.yml              CI pipeline (8 jobs)
└── README.md                       This file
```

---

## Build

```bash
# Build the Nim CLI
cd userland/agent
nim c -d:release --opt:speed -o:sigma-agent sigma_agent_main.nim

# Build the Rust engine (improves accuracy)
cargo build --release -p sigma-agent-core

# Install both
cp sigma-agent /usr/bin/
cp ../../target/release/sigma-agent-core /usr/bin/

# Or via sigma-pkg
sigma-pkg install sigma-agent
```

---

## LLM Setup

```bash
# Option 1 — sigma-ai daemon (recommended, fully sovereign)
sigma-pkg install sigma-ai

# Option 2 — Ollama (easy)
curl -fsSL https://ollama.ai/install.sh | sh && ollama pull tinyllama

# Option 3 — llama.cpp + GGUF model
sigma-pkg install llama-cpp
sigma-pkg install sigma-model-tinyllama   # ~700MB, places in ~/.cache/sigma/models/
```

---

## Training & Fine-tuning

```bash
# Every interaction is automatically recorded
sigma-agent "install sigma-edit"       # runs and records

# Rate responses to improve future accuracy
sigma-agent learn rate good            # thumbs up
sigma-agent learn rate bad             # thumbs down
sigma-agent learn correct "sigma-pkg install sigma-edit"  # correct answer

# Build fine-tuning dataset
sigma-agent learn build sigma-v1

# Fine-tune with llama.cpp LoRA
sigma-agent learn finetune tinyllama-1.1b sigma-agent-v1

# Use your custom model
sigma-agent config set model sigma-agent-v1
```

---

## Security

```bash
sigma-agent security scan         # full audit + score (0–100)
sigma-agent security logs         # log anomaly detection
sigma-agent security ports        # suspicious open ports
sigma-agent security permissions  # file permission + SUID audit
sigma-agent security policies     # AI policy recommendations
sigma-agent security telemetry    # privacy / telemetry audit
```

---

## Inspiration

| Project | What we took |
|---|---|
| [Claude Code](https://github.com/anthropics/claude-code) | ReAct loop, streaming, tool calling, REPL |
| [Aider](https://github.com/Aider-AI/aider) | Git-aware code editing, file watching, diff |
| [llama.cpp](https://github.com/ggml-org/llama.cpp) | Local LLM, GGUF models, ChatML, LoRA fine-tune |
| [ai-shell](https://github.com/BuilderIO/ai-shell) | NL → shell, error fixing |
| [copilot-cli](https://github.com/github/copilot-cli) | Shell integration, explain, suggest |
| [azure-cli](https://github.com/Azure/azure-cli) | Comprehensive subcommand surface |
| [openclaw](https://github.com/openclaw/openclaw) | GUI parity: every click = CLI command |
| [Hermes IDE](https://github.com/hermes-hq/hermes-ide) | Context-aware agent, IDE integration |
| [openai-cli](https://github.com/openai/openai-cli) | Streaming output, conversation history |
| [chatgpt-cli](https://github.com/j178/chatgpt) | Session management, conversation context |

---

*Sovereign AI — local inference, no telemetry, privacy-first.*
*Docs: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/sigma-agent*
