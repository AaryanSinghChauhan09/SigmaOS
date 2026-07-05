# sigma-agent — AI CLI Agent for SigmaOS

> Every GUI action, accessible from the terminal via natural language.
> Sovereign. Local. Learns from you. No cloud required.

---

## Overview

`sigma-agent` is SigmaOS's native AI CLI agent — a sovereign, full-spectrum system assistant that maps natural language to every OS operation, learns from your corrections, diagnoses its own health, manages persistent memory, generates automation scripts, and explains everything from commands to kernel internals.

```
σ ~/code › multi "why is CPU at 100%"

[sigma-sysadmin] routing...

High CPU diagnosis:
  PID   CPU%  NAME
  2341  85.3  sigma-build  ← build job consuming 85%

Options:
  - Wait for build to complete (normal during compilation)
  - Reduce parallelism: CARGO_JOBS=4 cargo build
  - Kill if stuck: sigma-agent "kill process 2341"
```

---

## Architecture

```
User Input (NL / voice / script / pipe / HTTP / memory-injected)
       │
       ▼
sigma-agent (Nim CLI — 35 modules)
  ├── IntentParser        → keyword → tool + args
  ├── ReAct Planner       → Thought → Action → Observation loop
  ├── Memory Engine       → persistent facts/prefs/patterns injected into prompts
  ├── Context Engine      → live OS state (CPU/mem/disk/git/logs) in prompts
  ├── LLM Backend         → sigma-ai → Ollama → llama.cpp → offline
  ├── Tool Executor       → 21 built-in tools (Rust + Nim)
  ├── Workflow Engine     → n8n-style YAML pipelines + event triggers ← NEW
  │    ├── 8 built-in templates (backup, update, cpu-alert, security...)
  │    ├── NL → workflow generator
  │    ├── Background scheduler (60s tick)
  │    └── Audit trail + run history
  ├── Multi-Agent         → 6 specialist sub-agents
  │    ├── sigma-security  (audit, policy, threat detection)
  │    ├── sigma-sysadmin  (services, resources, maintenance)
  │    ├── sigma-developer (code editing, git, build)
  │    ├── sigma-teacher   (OS concepts, interactive learning)
  │    ├── sigma-netops    (interfaces, VPN, DNS, firewall)
  │    └── sigma-pkgops    (packages, updates, registry)
  ├── Learning Engine     → record → rate → DPO pairs → LoRA fine-tune
  ├── Voice Pipeline      → mic → Whisper STT → NL command
  ├── Plugin System       → community .sigplugin skill packages
  ├── Script Generator    → NL goal → runnable .sa script
  ├── Explain Engine      → copilot-cli style ??, 45 built-in topics
  ├── TUI Components      → dashboard, fuzzy picker, interactive diff
  ├── Benchmark Suite     → 40 golden tests, regression detection
  ├── Notification Layer  → desktop toasts + event subscriptions
  ├── Doctor              → full environment self-diagnosis
  └── Updater             → GitHub releases check + atomic binary swap

Background Daemon (localhost:11430 + /run/sigma/agent.sock):
  ├── /v1/chat      → inference with context + memory injection
  ├── /v1/execute   → tool execution
  ├── /v1/complete  → LLM tab completions
  ├── /v1/status    → daemon health + backend info
  ├── /v1/context   → live OS snapshot
  ├── /v1/feedback  → RLHF data collection
  └── /v1/sync      → GitHub wiki knowledge sync (hourly)
```
  ├── Tool Executor       → 21 built-in tools (Rust + Nim)
  ├── Multi-Agent         → 6 specialist sub-agents
  │    ├── sigma-security  (audit, policy, threat detection)
  │    ├── sigma-sysadmin  (services, resources, maintenance)
  │    ├── sigma-developer (code editing, git, build)
  │    ├── sigma-teacher   (OS concepts, interactive learning)
  │    ├── sigma-netops    (interfaces, VPN, DNS, firewall)
  │    └── sigma-pkgops    (packages, updates, registry)
  ├── Learning Engine     → record → rate → DPO pairs → LoRA fine-tune
  ├── Voice Pipeline      → mic → Whisper STT → NL command
  ├── Plugin System       → community .sigplugin skill packages
  ├── Script Generator    → NL goal → runnable .sa script
  ├── Explain Engine      → copilot-cli style ??, 45 built-in topics
  ├── TUI Components      → dashboard, fuzzy picker, interactive diff
  ├── Benchmark Suite     → 40 golden tests, regression detection
  ├── Notification Layer  → desktop toasts + event subscriptions
  ├── Doctor              → full environment self-diagnosis
  ├── Updater             → GitHub releases check + atomic binary swap
  └── Autocomplete        → LLM-powered tab completion (56-entry static + dynamic)

Background Daemon (localhost:11430 + /run/sigma/agent.sock):
  ├── /v1/chat      → inference with context + memory injection
  ├── /v1/execute   → tool execution
  ├── /v1/complete  → LLM tab completions
  ├── /v1/status    → daemon health + backend info
  ├── /v1/context   → live OS snapshot
  ├── /v1/feedback  → RLHF data collection
  └── /v1/sync      → GitHub wiki knowledge sync (hourly)
```

---

## Complete GUI → CLI Mapping

`sigma-agent mirror list` — all 60+ mappings. Key examples:

| GUI | CLI |
|---|---|
| Open Terminal | `sigma-agent "open app sigma-terminal"` |
| Dark Mode | `sigma-agent "set dark mode"` |
| High Contrast | `sigma-agent "accessibility high-contrast on"` |
| Screen Reader | `sigma-agent "accessibility screen-reader on"` |
| Connect Wi-Fi | `sigma-netctl wifi <iface> <ssid> <pass>` |
| Install App | `sigma-pkg install <name>` |
| System Overview | `sigma-agent "system info"` |
| Kill Process | `sigma-agent "kill process <pid>"` |
| Switch Workspace | `sigma-agent "workspace 2"` |
| Screenshot | `sigma-agent "run sigma-screenshot"` |
| Lock Screen | `sigma-lock` |
| Send Notification | `sigma-agent notify "Title" "Body"` |

---

## Quick Start

```bash
sigma-agent                          # interactive REPL

sigma-agent "install sigma-edit"     # one-shot

sigma-agent doctor                   # health check

sigma-agent daemon start             # start background service

sigma-agent install --shell-integration  # set up shell hooks

```

---

## All Subcommands

### `doctor` — Self-diagnosis

```bash
sigma-agent doctor           # check all components + backends

sigma-agent doctor --verbose # show fix commands for every failure

```

Checks: binaries, LLM backends, shell integration, daemon, training data, plugins, voice backend, configuration, SigmaOS tools. Returns exit code 0/1/2.

### `update` — Self-update

```bash
sigma-agent update           # check + install from GitHub releases

sigma-agent update --check   # check only

sigma-agent update --dry-run # preview

sigma-agent update rollback  # revert to previous version

```

### `daemon` — Background AI service

```bash
sigma-agent daemon start|stop|status|sync|logs
```

HTTP API:
```bash
curl -X POST http://localhost:11430/v1/chat    -d '{"message":"system info"}'
curl http://localhost:11430/v1/status
curl http://localhost:11430/v1/context
curl -X POST http://localhost:11430/v1/complete -d '{"partial":"install sig"}'
curl -X POST http://localhost:11430/v1/feedback -d '{"quality":"excellent"}'
curl -X POST http://localhost:11430/v1/sync
```

### `memory` — Persistent long-term memory

Inspired by Claude Code's CLAUDE.md and Aider's `/remember`:

```bash
sigma-agent memory add "my code is in ~/code/myapp"
sigma-agent memory add "I prefer dark mode" --pref
sigma-agent memory add "always run sigma-agent doctor after install" --pattern
sigma-agent memory list                     # see everything the agent knows

sigma-agent memory list "dark mode"         # search

sigma-agent memory forget "dark mode"       # remove

sigma-agent memory pin "my code is in"      # protect from forget

sigma-agent memory project init             # create .sigma_memory file

sigma-agent memory project show             # display project memory

sigma-agent memory context                  # preview what's injected into prompts

```

Project memory (`.sigma_memory` in any directory) works like CLAUDE.md — the agent reads it automatically for project context. Edit it freely.

### `script-gen` — NL → .sa script generator

Inspired by ai-shell `??!` and Claude Code `/generate`:

```bash
sigma-agent script-gen "set up my development environment"
sigma-agent script-gen "backup home directory" -o backup.sa
sigma-agent script-gen "harden system security" --dry-run
sigma-agent script-gen --template dev-setup -o ~/setup.sa
sigma-agent script-gen --run "install sigma-edit and configure dark mode"
sigma-agent script-gen --list          # 12 built-in templates

```

Built-in templates: `dev-setup`, `security-harden`, `backup`, `update-system`, `network-setup`, `workspace-init`, `install-tools`, `accessibility-setup`, `dark-mode`, `privacy-mode`, `kiosk-mode`, `ai-setup`.

### `explain` — Explain everything

Inspired by copilot-cli `??`, Claude Code explain, Aider `/ask`:

```bash
sigma-agent explain                             # interactive explain mode

sigma-agent explain "sigma-pkg install <name>"  # explain a command

sigma-agent explain --concept "how paging works"
sigma-agent explain --concept "sigma_pledge"
sigma-agent explain --error "cargo build" "permission denied"
sigma-agent explain --code src/main.rs
sigma-agent explain --list                      # 45 built-in topics

sigma-agent "??" "what does sigma_pledge do"    # shorthand

```

45 built-in topics (no LLM needed): sigma_pledge, sigma_unveil, sigma-pkg, sigma-sh, paging, shard, buddy allocator, sigma-bus, MLFQ, sigma-agent, post-quantum cryptography, and more.

### `workflow` — n8n-style automation engine

Inspired by n8n, Claude Code multi-step, azure-cli automation runbooks.

```bash

# Install all 8 built-in templates

sigma-agent workflow install --all

# Run a workflow

sigma-agent workflow run weekly-backup
sigma-agent workflow run weekly-backup --dry-run
sigma-agent workflow run dev-workflow --verbose

# Generate from natural language

sigma-agent workflow create "backup home folder every Friday"
sigma-agent workflow create "run security audit nightly" -o nightly.yaml

# Manage

sigma-agent workflow list
sigma-agent workflow enable weekly-backup
sigma-agent workflow disable cpu-alert
sigma-agent workflow history
sigma-agent workflow audit

# Background scheduler (checks triggers every 60s)

sigma-agent workflow scheduler
```

Built-in templates: `weekly-backup`, `daily-update`, `cpu-alert`, `low-disk-alert`, `dev-workflow`, `security-hardening`, `on-boot-setup`, `pkg-update-notify`.

YAML format:
```yaml
name: my-workflow
trigger: schedule=daily 06:00   # or: manual, cpu>90, disk<10, pkg_update, boot

steps:
  - name: step-one
    action: "sigma-agent natural language command"
    on_fail: stop|continue|notify
    condition: "exit_code_of(prev-step) == 0"
    timeout: 60
    retries: 1
```

Full documentation: [sigma-agent-workflow](sigma-agent-workflow)

### `context` — Live system context

```bash
sigma-agent context          # pretty-print

sigma-agent context --json   # JSON for scripting

```

### `security` — Security advisor

```bash
sigma-agent security scan|logs|ports|permissions|policies|telemetry
```

### `learn` — RLHF feedback

```bash
sigma-agent learn rate good|bad|excellent
sigma-agent learn correct "sigma-pkg install sigma-edit"
sigma-agent learn build sigma-v1
sigma-agent learn finetune tinyllama-1.1b sigma-agent-v1
sigma-agent learn stats
```

### `multi` — Multi-agent orchestration

```bash
sigma-agent multi "why is CPU high"              # → sigma-sysadmin

sigma-agent multi --agent security "audit logs"  # force agent

sigma-agent multi diagnose "slow + network drops"
sigma-agent multi --list
```

### `voice` — Voice input

```bash
sigma-agent voice [--secs N] [--session] [--dry-run] [--status]
```

### `notify` — Notifications + event watcher

```bash
sigma-agent notify "Title" "Body" [--critical|--low]
sigma-agent notify history|watch|clear
```

### `plugin` — Skill extension system

```bash
sigma-agent plugin list|install|create|example|training|remove
```

### `complete` — Smart tab completion

```bash
sigma-agent complete "install sig"         # instant static

sigma-agent complete --dynamic "why is my" # LLM-powered

sigma-agent complete --shell > /etc/bash_completion.d/sigma-agent
sigma-agent complete --top
```

### `tui` — Terminal UI

```bash
sigma-agent tui dashboard [--refresh N]
sigma-agent tui pick
sigma-agent tui diff <file>
```

### `benchmark` — Quality benchmarking

```bash
sigma-agent benchmark [quick] [--cat <category>] [--save]
sigma-agent benchmark compare a.json b.json
```

### `train` — Training pipeline

```bash
sigma-agent train seed|build|stats|sync|compare
```

### `watch` — File watcher

```bash
sigma-agent watch [dir] [--ext .rs,.nim] [--suggest]
```

### `mirror` — GUI→CLI explorer

```bash
sigma-agent mirror list [filter]|run|count|search
```

### `config` — Profile system

```bash
sigma-agent config [set|profile|profiles|alias|models|reset]
```

### `install` — Shell integration

```bash
sigma-agent install --shell-integration [--shell fish|zsh|bash]
```

---

## Tools (21 built-in)

| # | Tool | Description |

|---|---|---|
| 1 | `read_file` | Read file content |
| 2 | `write_file` | Write/append to file |
| 3 | `list_dir` | List directory |
| 4 | `shell` | Execute shell command |
| 5 | `install_package` | Install via sigma-pkg |
| 6 | `open_app` | Launch application |
| 7 | `settings` | Get/set OS settings |
| 8 | `system_info` | System overview |
| 9 | `network` | Network management |
| 10 | `process` | Process management |
| 11 | `explain` | AI explanation |
| 12 | `code_edit` | AI code editing (Aider-style) |
| 13 | `summarise` | AI file summary |
| 14 | `wm_control` | Window manager |
| 15 | `notify` | Desktop notification |
| 16 | `clipboard` | Clipboard operations |
| 17 | `find_files` | Search files |
| 18 | `accessibility` | Accessibility toggles |
| 19 | `vpn` | VPN management |
| 20 | `disk` | Disk management |
| 21 | `context_query` | Live system snapshot |

---

## LLM Setup

```bash
sigma-pkg install sigma-ai          # sovereign daemon (recommended)

ollama pull tinyllama               # via Ollama

sigma-pkg install sigma-model-tinyllama  # GGUF (~700MB)

```

---

## Training & Fine-tuning

```bash
sigma-agent train seed              # 10 + 55 built-in samples (v1 + v2)

sigma-agent train sync              # pull GitHub wiki → training samples

sigma-agent learn rate good         # rate interactions as they happen

sigma-agent learn correct "..."     # provide corrections → DPO pairs

sigma-agent learn build sigma-v1    # ChatML + Alpaca + DPO JSONL

sigma-agent learn finetune tinyllama-1.1b sigma-agent-v1
sigma-agent config set model sigma-agent-v1
```

The seed dataset v2 (`sigma_agent_seed_v2.jsonl`) contains 55 high-quality examples covering all 21 tools, GUI parity, multi-agent routing, memory, script generation, explain mode, security scan, doctor, and more.

---

## Module File Structure (35 modules)

```
userland/agent/
├── main.rs                           Rust binary (sigma-agent-core)
├── Cargo.toml
├── sigma_agent.rs / core.rs / tools_ext.rs / llm.rs / planner.rs / code.rs
├── sigma_agent_main.nim              CLI master entry (35 modules imported)
├── sigma_agent_session.nim           Session manager
├── sigma_agent_config.nim            Profile system
├── sigma_agent_training.nim          Training + sync + A/B test
├── sigma_agent_gui_mirror.nim        60+ GUI→CLI mappings
├── sigma_agent_watch.nim             File watcher
├── sigma_agent_shell_integration.nim Shell hooks
├── sigma_agent_daemon.nim            HTTP daemon + /v1/complete
├── sigma_agent_context.nim           OS context engine
├── sigma_agent_security.nim          Security advisor
├── sigma_agent_learn.nim             RLHF + DPO fine-tune
├── sigma_agent_multi.nim             Multi-agent orchestration
├── sigma_agent_voice.nim             Voice input (Whisper STT)
├── sigma_agent_plugin.nim            Plugin / skill system
├── sigma_agent_autocomplete.nim      LLM tab completion
├── sigma_agent_tui.nim               TUI dashboard + picker + diff
├── sigma_agent_benchmark.nim         40-test benchmark suite
├── sigma_agent_notify.nim            Notifications + event watcher
├── sigma_agent_doctor.nim            Self-diagnosis
├── sigma_agent_update.nim            Self-update from GitHub releases
├── sigma_agent_memory.nim            Persistent memory (CLAUDE.md style)
├── sigma_agent_script_gen.nim        NL → .sa script generator
├── sigma_agent_explain.nim           Explain mode (copilot-cli ??)
├── sigma_agent_workflow.nim          n8n-style workflow engine  ← NEW
├── sigma_agent_seed_v2.jsonl         55 v2 training samples
├── sigma_agent.nimble                Package v15.1.0
├── sigma_agent_ci.yml                12-job CI pipeline
└── README.md
```

---

## Build

```bash
nim c -d:release --opt:speed -o:sigma-agent userland/agent/sigma_agent_main.nim
cargo build --release -p sigma-agent-core
cp sigma-agent /usr/bin/ && cp target/release/sigma-agent-core /usr/bin/
```

---

## Inspiration

| Project | What we took |
|---|---|
| [Claude Code](https://github.com/anthropics/claude-code) | ReAct, streaming, tool calling, REPL, daemon, CLAUDE.md memory, doctor |
| [Aider](https://github.com/Aider-AI/aider) | File watch, diff viewer, DPO training, /remember, /architect |
| [llama.cpp](https://github.com/ggml-org/llama.cpp) | Local LLM, GGUF, ChatML, LoRA fine-tune, Whisper STT |
| [ai-shell](https://github.com/BuilderIO/ai-shell) | NL→shell, error auto-fix, self-update, ??! script generation |
| [copilot-cli](https://github.com/github/copilot-cli) | ?? explain, shell integration, suggest/execute |
| [azure-cli](https://github.com/Azure/azure-cli) | Subcommand namespacing, extension system, upgrade, automation runbooks |
| [openclaw](https://github.com/openclaw/openclaw) | GUI parity, feedback loop, event-driven agent actions |
| [Hermes IDE](https://github.com/hermes-hq/hermes-ide) | Context injection, IDE plugin API, notifications |
| [openai-cli](https://github.com/openai/openai-cli) | Streaming, conversation history |
| [chatgpt-cli](https://github.com/j178/chatgpt) | Session persistence, multi-turn context |
| [claw-code](https://github.com/ultraworkers/claw-code) | Agent routing, multi-provider LLM |
| [ClaudeCode-Leak](https://github.com/0PeterAdel/ClaudeCode-Leak) | Tool schema, system prompt patterns |
| n8n | YAML workflow pipelines, event triggers, step conditions, scheduler |

---

### Sovereign AI — local inference, no telemetry, privacy-first.

*See also: [Architecture Overview](Architecture-Overview) · [Zenith Desktop](Zenith-Desktop) · [Security Model](Security-Model)*
```
