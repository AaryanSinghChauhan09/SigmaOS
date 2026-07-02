# sigma-agent — AI CLI Agent for SigmaOS

> Every GUI action, accessible from the terminal via natural language.
> Sovereign. Local. Learns from you. No cloud required.

---

## Overview

`sigma-agent` is SigmaOS's native AI CLI agent — a sovereign system assistant that maps natural language to every OS operation available in the Zenith Desktop GUI. It runs entirely on-device, learns from your feedback, specialises via sub-agents, audits security, and speaks to you through voice or terminal.

```
σ ~/code › security scan

Σ sigma-agent security scan
  Running log anomaly scan...    3 findings
  Running open port audit...     1 finding
  Running SUID binaries...       0 findings

🟡 WARN (3)
  ⚠ [auth]  3x 'authentication failure' in recent logs
     → Consider enabling fail2ban or rate-limiting SSH.
  ⚠ [ports] Port 6379 is listening (Redis)
     → Add requirepass in Redis config.

Security score: 82/100  ⚠ Some issues need attention
```

---

## Architecture

```
User Input (NL / voice / script / pipe / HTTP API)
       │
       ▼
 sigma-agent (Nim CLI — 28 modules)
  ├── IntentParser       → keyword → tool + args
  ├── ReAct Planner      → Thought→Action→Observation loop
  ├── Context Engine     → live CPU/mem/disk/git/logs injected into prompts
  ├── LLM Backend        → sigma-ai → Ollama → llama.cpp → offline fallback
  ├── Tool Executor      → 21 built-in tools (Rust + Nim)
  ├── Multi-Agent        → routes to specialist sub-agents
  │    ├── sigma-security  (audit, policy, threat detection)
  │    ├── sigma-sysadmin  (services, resources, maintenance)
  │    ├── sigma-developer (code editing, git, build)
  │    ├── sigma-teacher   (OS concepts, interactive learning)
  │    ├── sigma-netops    (interfaces, VPN, DNS, firewall)
  │    └── sigma-pkgops    (packages, updates, registry)
  ├── Learning Engine    → record → rate → DPO pairs → LoRA fine-tune
  ├── Voice Pipeline     → mic → Whisper STT → NL command
  ├── Plugin System      → community skills as .sigplugin packages
  ├── TUI Components     → dashboard, fuzzy picker, interactive diff
  ├── Benchmark Suite    → 40 golden tests, regression detection
  ├── Notification Layer → desktop toasts + event subscriptions
  ├── Doctor             → full environment self-diagnosis
  └── Updater            → GitHub releases check + atomic binary swap

Background Daemon (localhost:11430 + /run/sigma/agent.sock):
  ├── HTTP REST API     → /v1/chat  /v1/execute  /v1/status  /v1/context
  ├── /v1/complete      → LLM-powered tab completions
  ├── /v1/feedback      → RLHF data collection
  └── Knowledge sync   → GitHub wiki pulled hourly into LLM context
```

---

## Complete GUI → CLI Mapping

> Run `sigma-agent mirror list` to browse all 60+ mappings interactively.

### Desktop & Window Manager
| GUI | CLI |
|---|---|
| Open Terminal | `sigma-agent "open app sigma-terminal"` |
| Open Files | `sigma-agent "open app sigma-files"` |
| Switch Workspace 2 | `sigma-agent "workspace 2"` |
| Tile windows | `sigma-agent "tile"` |
| Fullscreen | `sigma-agent "fullscreen"` |
| Close window | `sigma-agent "close window"` |
| Lock screen | `sigma-lock` |
| Screenshot | `sigma-agent "run sigma-screenshot"` |

### Settings → Appearance / Accessibility
| GUI | CLI |
|---|---|
| Dark Mode | `sigma-agent "set dark mode"` |
| Light Mode | `sigma-agent "set light mode"` |
| High Contrast | `sigma-agent "accessibility high-contrast on"` |
| Screen Reader | `sigma-agent "accessibility screen-reader on"` |
| Large Text | `sigma-agent "accessibility large-text on"` |
| Reduce Motion | `sigma-agent "accessibility reduce-motion on"` |
| Colour-blind mode | `sigma-agent "accessibility colour-blind on"` |
| Disable animations | `sigma-agent "settings set appearance animations false"` |

### Settings → Network / VPN
| GUI | CLI |
|---|---|
| View interfaces | `sigma-netctl list` |
| Connect Wi-Fi | `sigma-netctl wifi <iface> <ssid> <pass>` |
| Change DNS | `sigma-netctl dns 1.1.1.1` |
| Connect VPN | `sigma-vpn connect <profile>` |
| Enable Firewall | `sigma-agent "settings set network firewall true"` |

### App Store / System Monitor
| GUI | CLI |
|---|---|
| Install app | `sigma-pkg install <name>` |
| Update all | `sigma-pkg update` |
| List installed | `sigma-pkg list` |
| System overview | `sigma-agent "system info"` |
| Kill process | `sigma-agent "kill process <pid>"` |
| Disk usage | `sigma-disks list` |
| Send notification | `sigma-agent notify "Title" "Body"` |

---

## Usage

```bash
# Interactive REPL (like Claude Code)
sigma-agent

# One-shot commands
sigma-agent "install sigma-edit and open it"
sigma-agent "set dark mode"
sigma-agent "system info"

# Script (.sa file)
sigma-agent --script ~/setup.sa

# Pipe
echo "system info" | sigma-agent --pipe
cat commands.txt | sigma-agent --pipe

# Flags
sigma-agent --trust safe       # read-only
sigma-agent --trust full       # all operations
sigma-agent --dry-run "rm -rf /tmp/old"
sigma-agent --verbose "diagnose why system is slow"
sigma-agent --no-color "list /usr/bin"
```

---

## All Subcommands

### `doctor` — Environment self-diagnosis

```bash
sigma-agent doctor           # check all components
sigma-agent doctor --verbose # show fix commands for each failure
```

Checks: sigma-agent binary, sigma-agent-core Rust engine, LLM backends (sigma-ai/Ollama/llama.cpp/model files), shell integration, daemon status, training data, plugins, voice backend, SigmaOS tool availability.

### `update` — Self-update

```bash
sigma-agent update           # check + install latest from GitHub
sigma-agent update --check   # check only, don't install
sigma-agent update --dry-run # preview only
sigma-agent update rollback  # revert to previous version
```

### `daemon` — Background AI service

```bash
sigma-agent daemon start     # start (HTTP :11430 + /run/sigma/agent.sock)
sigma-agent daemon stop      # stop
sigma-agent daemon status    # backend, requests, knowledge pages
sigma-agent daemon sync      # force GitHub wiki knowledge sync
sigma-agent daemon logs      # daemon log
```

HTTP API (when daemon running):
```bash
curl -X POST http://localhost:11430/v1/chat \
     -d '{"message":"system info","include_context":true}'
curl http://localhost:11430/v1/status
curl http://localhost:11430/v1/context
curl -X POST http://localhost:11430/v1/complete \
     -d '{"partial":"install sig"}'
curl -X POST http://localhost:11430/v1/feedback \
     -d '{"quality":"excellent"}'
curl -X POST http://localhost:11430/v1/sync
```

### `context` — Live system context

```bash
sigma-agent context          # pretty-print snapshot
sigma-agent context --json   # JSON output
```

Sources: CPU load, memory, disk, network, top processes, sigma daemons, packages, pending updates, git state (branch/dirty files/last commit), recent error logs, security posture, listening ports, loaded drivers.

### `security` — Security advisor

```bash
sigma-agent security scan         # full audit + 0-100 score
sigma-agent security logs         # log anomaly detection (12 patterns)
sigma-agent security ports        # suspicious open ports
sigma-agent security permissions  # file permissions + SUID audit
sigma-agent security policies     # AI policy recommendations
sigma-agent security telemetry    # privacy / telemetry audit
```

### `learn` — Reinforcement learning

```bash
sigma-agent learn rate good           # thumbs up last response
sigma-agent learn rate bad            # excluded from training
sigma-agent learn rate excellent      # highest training priority
sigma-agent learn correct "sigma-pkg install sigma-edit"  # DPO pair
sigma-agent learn build sigma-v1      # ChatML + Alpaca + DPO JSONL
sigma-agent learn finetune tinyllama-1.1b sigma-agent-v1
sigma-agent learn stats
sigma-agent learn export
```

### `multi` — Multi-agent orchestration

```bash
sigma-agent multi "why is CPU spiking"          # auto-route → sysadmin
sigma-agent multi "scan logs for intrusions"    # auto-route → security
sigma-agent multi "fix the segfault in main.rs" # auto-route → developer
sigma-agent multi "explain how paging works"    # auto-route → teacher
sigma-agent multi --agent security "review firewall"
sigma-agent multi --agent developer "refactor sigma_agent.rs"
sigma-agent multi diagnose "system is slow and network drops"
sigma-agent multi --list
```

### `voice` — Voice input (Whisper STT)

```bash
sigma-agent voice                 # record 5s → execute
sigma-agent voice --secs 10       # longer window
sigma-agent voice --dry-run       # transcribe only
sigma-agent voice --session       # continuous hands-free mode
sigma-agent voice --status        # show STT backend status
sigma-agent voice --transcribe audio.wav
```

Backends: sigma-voice daemon → whisper.cpp → Python SpeechRecognition.

### `notify` — Notifications + event subscriptions

```bash
sigma-agent notify "Build done" "cargo compiled successfully"
sigma-agent notify "Alert" "Auth failure" --critical
sigma-agent notify history         # notification log
sigma-agent notify watch           # start event watcher
sigma-agent notify watch --duration 60
sigma-agent notify clear
```

Event watcher monitors: build completion, high CPU load, low disk space, pending updates, daemon status.

### `plugin` — Skill extension system

```bash
sigma-agent plugin list
sigma-agent plugin install sigma-devtools
sigma-agent plugin create my-plugin      # scaffold plugin
sigma-agent plugin example               # install built-in example
sigma-agent plugin training              # export plugin training samples
sigma-agent plugin remove <name>
```

Plugin format: `~/.config/sigma/agent/plugins/<name>/plugin.toml` with `[[commands]]` trigger/shell_cmd entries. Plugins can include `training.jsonl` seed data merged into fine-tuning automatically.

### `complete` — LLM-powered tab completion

```bash
sigma-agent complete "install sig"         # instant static completions
sigma-agent complete --dynamic "why is my" # LLM-powered (needs daemon)
sigma-agent complete --ghost "set dark"    # single ghost-text token
sigma-agent complete --shell > /etc/bash_completion.d/sigma-agent
sigma-agent complete --shell fish          # fish completion script
sigma-agent complete --top                 # your most-used commands
```

### `tui` — Terminal UI

```bash
sigma-agent tui dashboard                  # live system metrics panel
sigma-agent tui dashboard --refresh 1     # 1s refresh rate
sigma-agent tui pick                       # fuzzy command picker (fzf-style)
sigma-agent tui diff <file>               # interactive hunk-level diff
```

### `benchmark` — Quality benchmarking

```bash
sigma-agent benchmark              # 40 golden tests
sigma-agent benchmark quick        # skip slow tests
sigma-agent benchmark --cat gui    # single category
sigma-agent benchmark --save       # save JSON report
sigma-agent benchmark compare a.json b.json  # regression detection
```

### `watch` — File watcher

```bash
sigma-agent watch .
sigma-agent watch . --ext .rs,.nim
sigma-agent watch . --suggest       # auto-AI on changes
```

### `mirror` — GUI→CLI explorer

```bash
sigma-agent mirror list
sigma-agent mirror list network
sigma-agent mirror run "dark mode"
sigma-agent mirror count
sigma-agent mirror search vpn
```

### `train` — Training pipeline

```bash
sigma-agent train seed               # write seed dataset
sigma-agent train build sigma-v1     # ChatML + Alpaca + DPO JSONL
sigma-agent train stats
sigma-agent train sync               # pull GitHub wiki as training samples
sigma-agent train compare tinyllama sigma-agent-v1  # A/B test models
```

### `config` — Profile system

```bash
sigma-agent config
sigma-agent config set model auto
sigma-agent config set trust full
sigma-agent config profile code      # temp=0.1, trust=full
sigma-agent config profiles
sigma-agent config alias k "kill process"
sigma-agent config models
sigma-agent config reset
```

### `install` — Shell integration

```bash
sigma-agent install --shell-integration         # auto-detect shell
sigma-agent install --shell-integration --shell fish
sigma-agent uninstall shell-integration
```

After sourcing `~/.sigma_agent_rc`:
```bash
ai "your request"  ai-dark  ai-sysinfo  ai-procs  ai-net  ai-disk
explain <cmd>      aifix <file>         ai_run <cmd>
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
| 12 | `code_edit` | edit, fix, refactor | AI code editing |
| 13 | `summarise` | summary, tldr | AI file summary |
| 14 | `wm_control` | window, tile, workspace | Window manager |
| 15 | `notify` | notification, toast | Desktop notification |
| 16 | `clipboard` | copy, paste | Clipboard |
| 17 | `find_files` | find, search, grep | Search files |
| 18 | `accessibility` | a11y | Accessibility toggles |
| 19 | `vpn` | wireguard | VPN management |
| 20 | `disk` | df, du, storage | Disk management |
| 21 | `context_query` | ctx, context | Live system snapshot |

---

## LLM Setup

```bash
# Sovereign (recommended)
sigma-pkg install sigma-ai

# Easy
curl -fsSL https://ollama.ai/install.sh | sh && ollama pull tinyllama

# Any GGUF model
sigma-pkg install llama-cpp && sigma-pkg install sigma-model-tinyllama
```

---

## File Structure (28 modules)

```
userland/agent/
├── main.rs                           Rust binary (sigma-agent-core)
├── Cargo.toml
├── sigma_agent.rs                    10 core tools
├── sigma_agent_core.rs               Intent parser + Agent + REPL
├── sigma_agent_tools_ext.rs          10 extended tools
├── sigma_llm.rs                      LLM backends
├── sigma_agent_planner.rs            ReAct planner
├── sigma_agent_code.rs               Code edit + diff + git
├── sigma_agent_main.nim              CLI master entry (28 modules)
├── sigma_agent_session.nim           Session manager
├── sigma_agent_config.nim            Profile system
├── sigma_agent_training.nim          Training pipeline + sync + A/B test
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
├── sigma_agent_notify.nim            Notifications + event watcher  ← NEW
├── sigma_agent_doctor.nim            Self-diagnosis (like claude doctor)  ← NEW
├── sigma_agent_update.nim            Self-update from GitHub releases  ← NEW
├── sigma_agent.nimble
├── sigma_agent_ci.yml                11-job CI pipeline
└── README.md
```

---

## Build

```bash
nim c -d:release --opt:speed -o:sigma-agent userland/agent/sigma_agent_main.nim
cargo build --release -p sigma-agent-core
cp sigma-agent /usr/bin/ && cp target/release/sigma-agent-core /usr/bin/

# Or: sigma-pkg install sigma-agent
```

---

## Inspiration

| Project | What we took |
|---|---|
| [Claude Code](https://github.com/anthropics/claude-code) | ReAct loop, streaming, tool calling, REPL, daemon mode, `claude doctor` |
| [Aider](https://github.com/Aider-AI/aider) | File watch, git-aware edits, diff display, DPO training, `--dry-run` |
| [llama.cpp](https://github.com/ggml-org/llama.cpp) | Local LLM, GGUF, ChatML, LoRA fine-tune, Whisper STT |
| [ai-shell](https://github.com/BuilderIO/ai-shell) | NL→shell, error auto-fix, self-update pattern |
| [copilot-cli](https://github.com/github/copilot-cli) | Shell integration, `??` explain, suggest, completion |
| [azure-cli](https://github.com/Azure/azure-cli) | Subcommand namespacing, `az upgrade`, extension system |
| [openclaw](https://github.com/openclaw/openclaw) | GUI parity, feedback loop, community plugin system |
| [Hermes IDE](https://github.com/hermes-hq/hermes-ide) | Context injection, IDE plugin HTTP API |
| [openai-cli](https://github.com/openai/openai-cli) | Streaming output, conversation history |
| [chatgpt-cli](https://github.com/j178/chatgpt) | Session management, multi-turn context |
| [claw-code](https://github.com/ultraworkers/claw-code) | Agent routing, multi-provider LLM |
| [ClaudeCode-Leak](https://github.com/0PeterAdel/ClaudeCode-Leak) | Tool schema design, system prompt patterns |

---

*Sovereign AI — local inference, no telemetry, privacy-first.*

*See also: [Architecture Overview](Architecture-Overview) · [Zenith Desktop](Zenith-Desktop) · [Security Model](Security-Model) · [sigma-cli Reference](sigma-cli-man-page)*
