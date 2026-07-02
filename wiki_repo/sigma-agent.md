# sigma-agent — AI CLI Agent for SigmaOS

> Every GUI action, accessible from the terminal via natural language.
> Sovereign. Local. Learns from you. No cloud required.

---

## Overview

`sigma-agent` is SigmaOS's native AI CLI agent — a full system assistant that combines natural language command execution, multi-agent specialisation, live OS context awareness, security auditing, reinforcement learning, voice input, and a background knowledge-sync daemon.

```
σ ~/code › security scan

Σ sigma-agent security scan

  Running log anomaly scan...    3 findings
  Running open port audit...     1 finding
  Running file permissions...    0 findings
  Running SUID binaries...       0 findings
  Running telemetry audit...     1 finding

🟡 WARN (4)
  ⚠ [auth]    3x 'authentication failure' in recent logs
     → Consider enabling fail2ban or rate-limiting SSH.
  ⚠ [ports]   Port 6379 is listening (Redis)
     → Add requirepass in Redis config.

Security score: 78/100
  ⚠ Some issues need attention
```

---

## Architecture

```
User Input (NL / voice / script / pipe / HTTP)
       │
       ▼
 sigma-agent (Nim CLI master)
  ├── IntentParser     → keyword → tool + args
  ├── ReAct Planner    → multi-step Thought→Action→Observation loop
  ├── Context Engine   → live CPU/mem/disk/git state injected into prompts
  ├── LLM Backend      → sigma-ai → Ollama → llama.cpp → offline fallback
  ├── Tool Executor    → 21 built-in tools (Rust + Nim)
  ├── Multi-Agent      → routes to specialist sub-agents
  │    ├── sigma-security  (audit, policy, threat detection)
  │    ├── sigma-sysadmin  (services, resources, maintenance)
  │    ├── sigma-developer (code editing, git, build)
  │    ├── sigma-teacher   (OS concepts, interactive learning)
  │    ├── sigma-netops    (interfaces, VPN, DNS, firewall)
  │    └── sigma-pkgops    (packages, updates, registry)
  ├── Learning Engine  → record → rate → DPO pairs → LoRA fine-tune
  └── Voice Pipeline   → mic → Whisper STT → NL command

Background Daemon (port 11430 + /run/sigma/agent.sock):
  ├── HTTP REST API     → /v1/chat  /v1/execute  /v1/status  /v1/context
  ├── Knowledge sync   → GitHub wiki pages pulled hourly
  ├── Feedback API     → /v1/feedback (RLHF data collection)
  └── Context API      → /v1/context (live OS snapshot for IDE plugins)
```

---

## Complete GUI → CLI Mapping

> `sigma-agent mirror list` shows all 60+ mappings. Key examples:

### Desktop / Window Manager

| GUI | CLI |
|---|---|
| Open Terminal | `sigma-agent "open app sigma-terminal"` |
| Switch Workspace 2 | `sigma-agent "workspace 2"` |
| Tile Windows | `sigma-agent "tile"` |
| Fullscreen | `sigma-agent "fullscreen"` |
| Close Window | `sigma-agent "close window"` |

### Settings → Appearance / Accessibility

| GUI | CLI |
|---|---|
| Dark Mode | `sigma-agent "set dark mode"` |
| Light Mode | `sigma-agent "set light mode"` |
| High Contrast | `sigma-agent "accessibility high-contrast on"` |
| Screen Reader | `sigma-agent "accessibility screen-reader on"` |
| Large Text | `sigma-agent "accessibility large-text on"` |
| Reduce Motion | `sigma-agent "accessibility reduce-motion on"` |

### Network / VPN / Security

| GUI | CLI |
|---|---|
| Connect Wi-Fi | `sigma-netctl wifi <iface> <ssid> <pass>` |
| Change DNS | `sigma-netctl dns <server>` |
| Connect VPN | `sigma-vpn connect <profile>` |
| Enable Firewall | `sigma-agent "settings set network firewall true"` |

### App Store / System

| GUI | CLI |
|---|---|
| Install App | `sigma-pkg install <name>` |
| Update All | `sigma-pkg update` |
| System Overview | `sigma-agent "system info"` |
| Kill Process | `sigma-agent "kill process <pid>"` |
| Disk Usage | `sigma-disks list` |

---

## Usage

```bash
# Interactive REPL
sigma-agent

# One-shot
sigma-agent "install sigma-edit and open it"
sigma-agent "set dark mode"
sigma-agent "system info"

# Script (.sa file)
sigma-agent --script ~/setup.sa

# Pipe
echo "system info" | sigma-agent --pipe
cat commands.txt | sigma-agent --pipe

# Trust levels
sigma-agent --trust safe      # read-only
sigma-agent --trust standard  # default
sigma-agent --trust full      # all operations

# Dry-run
sigma-agent --dry-run "install sigma-edit"

# Verbose ReAct reasoning
sigma-agent --verbose "diagnose why system is slow"
```

---

## Subcommands

### `daemon` — Background AI service

The daemon runs in the background, provides a REST API, and syncs knowledge from the GitHub wiki hourly so `sigma-agent` always has up-to-date documentation.

```bash
sigma-agent daemon start     # start daemon (HTTP :11430 + Unix socket)
sigma-agent daemon stop      # stop daemon
sigma-agent daemon status    # show stats: backend, requests, knowledge pages
sigma-agent daemon sync      # force GitHub wiki knowledge sync
sigma-agent daemon logs      # show daemon log
```

HTTP API (when daemon is running):
```bash
curl http://localhost:11430/v1/status
curl http://localhost:11430/v1/context
curl -X POST http://localhost:11430/v1/chat \
     -d '{"message":"system info","include_context":true}'
curl -X POST http://localhost:11430/v1/execute \
     -d '{"command":"list /home/user"}'
curl -X POST http://localhost:11430/v1/feedback \
     -d '{"quality":"excellent"}'
curl -X POST http://localhost:11430/v1/sync
```

### `context` — Live system context

Collects and displays live OS state. When the daemon is running, this context is automatically injected into every LLM prompt, giving the agent accurate, tailored answers.

```bash
sigma-agent context          # pretty-print system snapshot
sigma-agent context --json   # JSON output for scripting
```

Context sources: CPU load, memory usage, disk, network interfaces, top processes, sigma daemons, installed packages, pending updates, git branch/dirty files, recent error logs, security posture, listening ports, loaded drivers.

### `security` — Security advisor sub-agent

```bash
sigma-agent security scan           # full audit + score (0–100)
sigma-agent security logs           # scan logs for anomalies
sigma-agent security ports          # suspicious open ports
sigma-agent security permissions    # file permissions + SUID audit
sigma-agent security policies       # AI policy recommendations
sigma-agent security telemetry      # privacy / telemetry audit
```

Score breakdown: 100 − (critical findings × 20) − (warnings × 5) − (info × 1).

What it checks:
- Log patterns: auth failures, OOM kills, segfaults, kernel BUGs, service failures
- Open ports: FTP (21), Telnet (23), Redis (6379), MongoDB (27017), etc.
- File permissions: /etc/shadow, /etc/sudoers, /etc/ssh/sshd_config
- Unexpected SUID binaries
- Telemetry settings, analytics processes

### `learn` — Reinforcement learning from feedback

Every sigma-agent interaction is recorded automatically. Rate responses to improve accuracy:

```bash
sigma-agent learn rate good           # thumbs up
sigma-agent learn rate excellent      # prioritised in training
sigma-agent learn rate bad            # excluded from training

# If the agent gave the wrong answer, provide the right one:
sigma-agent learn correct "sigma-netctl wifi wlan0 MyNetwork secret123"

# Build fine-tuning datasets
sigma-agent learn build sigma-v1      # ChatML + Alpaca + DPO JSONL

# Fine-tune with llama.cpp LoRA
sigma-agent learn finetune tinyllama-1.1b sigma-agent-v1

# Use your custom model
sigma-agent config set model sigma-agent-v1

sigma-agent learn stats               # learning statistics
sigma-agent learn export              # export datasets
```

Dataset formats produced:
- `*_chatml.jsonl` — OpenAI/llama.cpp ChatML format
- `*_alpaca.jsonl` — Alpaca instruction-input-output format
- `*_dpo.jsonl` — Direct Preference Optimisation pairs (chosen/rejected)

### `multi` — Multi-agent orchestration

Automatically routes your request to the best specialist sub-agent:

```bash
# Auto-route (sigma-agent classifies by keywords)
sigma-agent multi "why is my CPU spiking?"         # → sigma-sysadmin
sigma-agent multi "scan logs for intrusions"        # → sigma-security
sigma-agent multi "fix the segfault in main.rs"     # → sigma-developer
sigma-agent multi "explain how paging works"        # → sigma-teacher
sigma-agent multi "my VPN keeps dropping"           # → sigma-netops

# Force a specific agent
sigma-agent multi --agent security "review firewall rules"
sigma-agent multi --agent developer "refactor sigma_agent.rs"
sigma-agent multi --agent teacher "explain sigma_pledge syscall"

# Collaborative multi-agent diagnosis
sigma-agent multi diagnose "system is slow and network drops"
# → sysadmin gathers system state
# → security checks for anomalies
# → combined report

sigma-agent multi --list              # show all sub-agents
```

Sub-agents:

| Agent | Trust | Specialisation |
|---|---|---|
| `sigma-security` | standard | Log scan, policy advisor, threat detection, port audit |
| `sigma-sysadmin` | standard | Processes, services, disks, cgroups, kernel params |
| `sigma-developer` | full | Rust/Nim/Zig code edit, git, debug, refactor, test gen |
| `sigma-teacher` | safe | OS internals, kernel concepts, interactive learning |
| `sigma-netops` | standard | Interfaces, VPN, DNS, firewall, connectivity |
| `sigma-pkgops` | standard | sigma-pkg, flatpak, AppImage, package recipes |

### `voice` — Voice input (Whisper STT)

```bash
sigma-agent voice                    # record 5s → transcribe → execute
sigma-agent voice --secs 10          # longer recording window
sigma-agent voice --dry-run          # transcribe only, don't execute
sigma-agent voice --session          # continuous hands-free mode
sigma-agent voice --transcribe audio.wav  # transcribe existing WAV
sigma-agent voice --status           # show voice backend status
```

Backends (priority order):
1. `sigma-voice` daemon — `sigma-pkg install sigma-voice` (sovereign, offline)
2. `whisper.cpp` — `sigma-pkg install whisper-cpp` (local GGML model)
3. Python SpeechRecognition — fallback (requires network for Google API)

Model setup:
```bash
sigma-pkg install whisper-model-base-en  # ~150MB
# Places model at: ~/.cache/sigma/models/whisper-base.en.bin
```

### `watch` — File watcher with AI suggestions

```bash
sigma-agent watch .                      # watch current directory
sigma-agent watch /home/user/code        # watch specific directory
sigma-agent watch . --ext .rs,.nim       # filter by extension
sigma-agent watch . --suggest            # auto-suggest on changes
sigma-agent watch . --interval 500       # 500ms poll interval
```

### `mirror` — GUI→CLI mapping explorer

```bash
sigma-agent mirror list              # all 60+ mappings
sigma-agent mirror list network      # filter by keyword
sigma-agent mirror run "dark mode"   # execute a GUI action
sigma-agent mirror count             # total mapped actions
sigma-agent mirror search vpn        # search mappings
```

### `config` — Profile system

```bash
sigma-agent config                   # show active config
sigma-agent config set model auto    # set LLM model
sigma-agent config set trust full    # set trust level
sigma-agent config profile code      # switch to code profile (temp=0.1)
sigma-agent config profiles          # list all profiles
sigma-agent config alias k "kill process"  # add shortcut
sigma-agent config models            # list downloaded GGUF models
```

### `train` — Low-level training pipeline

```bash
sigma-agent train seed               # write seed dataset
sigma-agent train build sigma-v1     # build fine-tuning dataset
sigma-agent train stats              # dataset statistics
sigma-agent train rate excellent     # rate last interaction
```

### `install` — Shell integration

```bash
sigma-agent install --shell-integration         # auto-detect shell
sigma-agent install --shell-integration --shell fish
sigma-agent uninstall shell-integration
```

After sourcing `~/.sigma_agent_rc`:
```bash
ai "your request"        # any natural language command
ai-dark                  # dark mode
ai-sysinfo               # system info
ai-procs                 # processes
ai-net                   # network status
ai-disk                  # disk usage
explain <command>        # AI explains
aifix <file>             # AI fixes errors
```

---

## Tools (21 built-in)

| # | Tool | Aliases | Description |
|---|---|---|---|
| 1 | `read_file` | cat, show, read | Read file content |
| 2 | `write_file` | write, save | Write/append to file |
| 3 | `list_dir` | ls, dir, list | List directory |
| 4 | `shell` | run, exec, bash | Execute shell command |
| 5 | `install_package` | install, add | Install via sigma-pkg |
| 6 | `open_app` | open, launch | Launch application |
| 7 | `settings` | config, set, get | Get/set OS settings |
| 8 | `system_info` | sysinfo, neofetch | System overview |
| 9 | `network` | net, wifi, netctl | Network management |
| 10 | `process` | ps, kill, top | Process management |
| 11 | `explain` | what, how, why | AI explanation |
| 12 | `code_edit` | edit, fix, refactor | AI code editing (Aider-style) |
| 13 | `summarise` | summary, tldr | AI file summary |
| 14 | `wm_control` | window, tile, workspace | Window manager |
| 15 | `notify` | notification, toast | Desktop notification |
| 16 | `clipboard` | copy, paste | Clipboard operations |
| 17 | `find_files` | find, search, grep | Search files |
| 18 | `accessibility` | a11y | Accessibility toggles |
| 19 | `vpn` | wireguard | VPN management |
| 20 | `disk` | df, du, storage | Disk management |
| 21 | `context_query` | ctx, context | Live system context snapshot |

---

## LLM Backend Setup

```bash
# Recommended: sigma-ai (sovereign, always-on daemon)
sigma-pkg install sigma-ai

# Easy: Ollama
curl -fsSL https://ollama.ai/install.sh | sh
ollama pull tinyllama

# Flexible: any GGUF model with llama.cpp
sigma-pkg install llama-cpp
sigma-pkg install sigma-model-tinyllama   # ~700MB
```

---

## File Structure

```
userland/agent/
├── main.rs                           Rust binary entry (sigma-agent-core)
├── Cargo.toml                        Rust crate manifest
├── sigma_agent.rs                    10 core tools
├── sigma_agent_core.rs               Intent parser + Agent + REPL
├── sigma_agent_tools_ext.rs          10 extended tools
├── sigma_llm.rs                      LLM backends
├── sigma_agent_planner.rs            ReAct planner
├── sigma_agent_code.rs               Code edit + diff + git
├── sigma_agent_main.nim              CLI master entry + subcommand router  ← NEW
├── sigma_agent_session.nim           Session manager
├── sigma_agent_config.nim            Profile system
├── sigma_agent_training.nim          Training pipeline
├── sigma_agent_gui_mirror.nim        60+ GUI→CLI mappings
├── sigma_agent_watch.nim             File watcher
├── sigma_agent_shell_integration.nim Shell hooks
├── sigma_agent_daemon.nim            Background daemon + HTTP API  ← NEW
├── sigma_agent_context.nim           Live OS context engine        ← NEW
├── sigma_agent_security.nim          Security advisor sub-agent    ← NEW
├── sigma_agent_learn.nim             RLHF + DPO fine-tune          ← NEW
├── sigma_agent_multi.nim             Multi-agent orchestration     ← NEW
├── sigma_agent_voice.nim             Voice input (Whisper STT)     ← NEW
├── sigma_agent.nimble                Nim package definition
├── sigma_agent_ci.yml                CI pipeline (8 jobs)
└── README.md                         Developer documentation
```

---

## Build

```bash
# Nim CLI
nim c -d:release --opt:speed -o:sigma-agent userland/agent/sigma_agent_main.nim
cp sigma-agent /usr/bin/

# Rust engine
cargo build --release -p sigma-agent-core
cp target/release/sigma-agent-core /usr/bin/

# Or: sigma-pkg install sigma-agent
```

---

## Inspiration

| Project | What we took |
|---|---|
| [Claude Code](https://github.com/anthropics/claude-code) | ReAct loop, streaming, tool calling, REPL, daemon mode |
| [Aider](https://github.com/Aider-AI/aider) | File watch, git-aware edits, diff display, DPO training |
| [llama.cpp](https://github.com/ggml-org/llama.cpp) | Local LLM, GGUF, ChatML, LoRA fine-tune, Whisper |
| [ai-shell](https://github.com/BuilderIO/ai-shell) | NL→shell, error auto-fix |
| [copilot-cli](https://github.com/github/copilot-cli) | Shell integration, explain/suggest/execute pattern |
| [azure-cli](https://github.com/Azure/azure-cli) | Comprehensive subcommand surface, namespaced agents |
| [openclaw](https://github.com/openclaw/openclaw) | GUI parity principle, feedback loop |
| [Hermes IDE](https://github.com/hermes-hq/hermes-ide) | Context-aware agent, IDE plugin API |
| [openai-cli](https://github.com/openai/openai-cli) | Streaming, conversation history, pipe mode |
| [chatgpt-cli](https://github.com/j178/chatgpt) | Session management, multi-turn context |

---

*Sovereign AI — local inference, no telemetry, no external API dependencies.*

*See also: [Architecture Overview](Architecture-Overview) · [Zenith Desktop](Zenith-Desktop) · [Security Model](Security-Model) · [sigma-cli Reference](sigma-cli-man-page)*
