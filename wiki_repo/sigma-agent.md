# sigma-agent — AI CLI Agent for SigmaOS

> Every GUI action, accessible from the terminal via natural language.
> No cloud required. Sovereign by design.

---

## Overview

`sigma-agent` is SigmaOS's built-in AI CLI agent. It maps natural language to OS operations — every setting, app, file, and system control you can click in the Zenith Desktop GUI can be done with a sentence.

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
User Input (natural language)
       │
       ▼
sigma-agent (Nim CLI)
  ├── IntentParser: keyword → tool + args
  ├── ReAct Planner: multi-step reasoning
  ├── LLM Backend (auto-select):
  │     1. sigma-ai daemon  → /run/sigma/ai.sock  (sovereign, always-on)
  │     2. Ollama HTTP API  → localhost:11434      (easy setup)
  │     3. llama.cpp CLI    → GGUF model file      (any model)
  │     4. NullBackend      → built-in responses  (offline fallback)
  └── Tool Executor (20 built-in tools)
       │
       ▼
  ANSI output (colour-coded, streaming)
```

**ReAct Loop** (Claude Code / Aider pattern):
```
Thought: [LLM reasons about the task]
Action:  tool_name
Args:    key=value
Observation: [tool result]
→ repeat until Final Answer
```

---

## Complete GUI → CLI Mapping

### Desktop / Window Manager

| GUI Action | CLI Command |
|---|---|
| Open Terminal | `sigma-agent "open app sigma-terminal"` |
| Open Files | `sigma-agent "open app sigma-files"` |
| Open Editor | `sigma-agent "open app sigma-edit"` |
| Open App Store | `sigma-agent "open app sigma-appstore"` |
| Switch to Workspace 2 | `sigma-agent "workspace 2"` |
| Tile windows | `sigma-agent "tile"` |
| Fullscreen | `sigma-agent "fullscreen"` |
| Close window | `sigma-agent "close window"` |
| Cycle layout | `sigma-agent "cycle layout"` |

### Settings → Appearance

| GUI Action | CLI Command |
|---|---|
| Dark Mode | `sigma-agent "set dark mode"` |
| Light Mode | `sigma-agent "set light mode"` |
| High Contrast | `sigma-agent "set high contrast"` |
| Corner radius | `sigma-agent "settings set appearance corner_radius 8"` |
| Disable animations | `sigma-agent "settings set appearance animations false"` |
| Increase font size | `sigma-agent "accessibility large-text on"` |

### Settings → Network

| GUI Action | CLI Command |
|---|---|
| View interfaces | `sigma-netctl list` |
| Connect Wi-Fi | `sigma-netctl wifi <iface> <ssid> <pass>` |
| Disconnect Wi-Fi | `sigma-netctl down <iface>` |
| Set static IP | `sigma-netctl static <iface> <ip> <mask> <gw>` |
| Enable DHCP | `sigma-netctl dhcp <iface>` |
| Change DNS | `sigma-netctl dns <server>` |
| Connect VPN | `sigma-vpn connect <profile>` |
| Disconnect VPN | `sigma-vpn disconnect <profile>` |
| Enable firewall | `sigma-agent "settings set network firewall true"` |

### Settings → Accessibility

| GUI Action | CLI Command |
|---|---|
| High Contrast | `sigma-agent "accessibility high-contrast on"` |
| Screen Reader | `sigma-agent "accessibility screen-reader on"` |
| Reduce Motion | `sigma-agent "accessibility reduce-motion on"` |
| Large Text | `sigma-agent "accessibility large-text on"` |
| Colour Blind | `sigma-agent "accessibility colour-blind on"` |
| Sticky Keys | `sigma-agent "accessibility sticky-keys on"` |

### App Store

| GUI Action | CLI Command |
|---|---|
| Search packages | `sigma-pkg search <query>` |
| Install | `sigma-pkg install <name>` |
| Remove | `sigma-pkg remove <name>` |
| Update all | `sigma-pkg update` |
| List installed | `sigma-pkg list` |
| Package info | `sigma-pkg info <name>` |

### System Monitor

| GUI Action | CLI Command |
|---|---|
| Overview | `sigma-agent "system info"` |
| Processes | `sigma-top -1` |
| Kill process | `sigma-agent "kill process <pid>"` |
| Disk usage | `sigma-disks list` |
| Network speed | `sigma-netctl show <iface>` |

### File Manager

| GUI Action | CLI Command |
|---|---|
| Browse home | `sigma-agent "list ~"` |
| Create folder | `sigma-agent "run mkdir <name>"` |
| Delete file | `sigma-agent "run rm <file>"` |
| Move file | `sigma-agent "run mv <src> <dst>"` |
| View file | `sigma-agent "read <file>"` |
| Search files | `sigma-agent "find <query>"` |

### Desktop Controls

| GUI Action | CLI Command |
|---|---|
| Lock screen | `sigma-lock` |
| Screenshot | `sigma-screenshot` |
| Send notification | `sigma-agent "notify 'Build done' --body 'Done'"` |
| Shutdown | `sigma-agent "run shutdown -h now"` |
| Restart | `sigma-agent "run reboot"` |

> For the complete mapping of 60+ GUI actions, run: `sigma-agent mirror list`

---

## Usage

```bash
# Interactive REPL (like Claude Code)
sigma-agent

# One-shot command
sigma-agent "install sigma-edit"
sigma-agent "set dark mode"
sigma-agent "system info"

# Script file (.sa)
sigma-agent --script ~/setup.sa

# Pipe mode (stdin)
echo "system info" | sigma-agent --pipe
cat commands.txt | sigma-agent --pipe

# Dry-run (preview only)
sigma-agent --dry-run "install sigma-edit and configure it"

# Trust levels
sigma-agent --trust safe      # read-only operations
sigma-agent --trust standard  # default (reads + installs + settings)
sigma-agent --trust full      # all operations including shell

# Verbose (show ReAct reasoning)
sigma-agent --verbose "find and fix the error in main.rs"
```

---

## Subcommands

### `mirror` — GUI → CLI mapping

```bash
sigma-agent mirror list              # All 60+ GUI→CLI mappings
sigma-agent mirror list network      # Filter by keyword
sigma-agent mirror run "dark mode"   # Execute a GUI action
sigma-agent mirror count             # Show total mapped actions
sigma-agent mirror search vpn        # Search mappings
```

### `watch` — File watcher with AI suggestions (Aider-style)

```bash
sigma-agent watch .                         # Watch current directory
sigma-agent watch /home/user/code           # Watch a specific directory
sigma-agent watch . --ext .rs,.nim          # Watch only specific extensions
sigma-agent watch . --suggest               # Auto-suggest on changes
sigma-agent watch . --interval 500          # Custom poll interval (ms)
sigma-agent watch . --ignore target,.git    # Ignore directories
```

### `train` — Training data and fine-tuning pipeline

```bash
sigma-agent train seed           # Write built-in seed dataset
sigma-agent train build v1       # Build fine-tuning dataset (ChatML + Alpaca JSONL)
sigma-agent train stats          # Show training data statistics
sigma-agent train list           # List available datasets
sigma-agent train rate good      # Rate last interaction as good
sigma-agent train rate excellent # Rate as excellent (boosts fine-tuning weight)
sigma-agent train rate bad       # Mark as bad (excluded from training)
```

Fine-tuning with llama.cpp:
```bash
llama-finetune --model base.gguf \
  --train ~/.cache/sigma/agent_datasets/v1_chatml.jsonl \
  --output sigma-agent-v1.gguf
```

### `config` — Profile system

```bash
sigma-agent config                          # Show active config
sigma-agent config set model auto           # Set LLM model
sigma-agent config set trust standard       # Set default trust level
sigma-agent config set temperature 0.7      # Tune LLM temperature
sigma-agent config profile code             # Switch to code profile
sigma-agent config profiles                 # List all profiles
sigma-agent config alias k kill process     # Add command alias
sigma-agent config models                   # List downloaded GGUF models
sigma-agent config reset                    # Reset to defaults
```

Built-in profiles:
| Profile | Trust | Temperature | Use case |
|---|---|---|---|
| `default` | standard | 0.7 | General use |
| `code` | full | 0.1 | Precise code edits |
| `safe` | safe | 0.5 | Read-only, dry-run |

### `install` — Shell integration

```bash
# Auto-detects sigma-sh / bash / zsh / fish
sigma-agent install --shell-integration

# Force a specific shell
sigma-agent install --shell-integration --shell fish
sigma-agent install --shell-integration --shell zsh

# Uninstall
sigma-agent uninstall shell-integration
```

After installing, restart your shell or run `source ~/.sigma_agent_rc`. New shortcuts:

```bash
ai "your request"    # natural language OS commands
ai-dark              # switch to dark mode
ai-light             # switch to light mode
ai-sysinfo           # system overview
ai-procs             # running processes
ai-net               # network status
ai-disk              # disk usage
explain <command>    # AI explains a command
aifix <file>         # AI fixes errors in file
watch .              # file watcher + AI
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
| `accessibility` | a11y | Accessibility toggles |
| `vpn` | wireguard | VPN management |
| `disk` | df, du, storage | Disk management |

---

## Script Format (.sa files)

```bash
# sigma-agent script: dev-setup.sa
# Lines starting with # are comments

install sigma-edit
install sigma-terminal
set dark mode
accessibility large-text on
notify "Setup complete" --body "Your dev environment is ready"
run echo hello > /home/user/hello.txt
read /home/user/hello.txt
```

Run with:
```bash
sigma-agent --script dev-setup.sa
```

---

## LLM Backend Setup

### Option 1: sigma-ai (recommended — sovereign, no cloud)

```bash
sigma-pkg install sigma-ai
# sigma-ai runs as a daemon: /run/sigma/ai.sock
# Automatically used if available
```

### Option 2: Ollama (easy setup)

```bash
curl -fsSL https://ollama.ai/install.sh | sh
ollama pull tinyllama    # ~700MB
# sigma-agent auto-detects Ollama on localhost:11434
```

### Option 3: llama.cpp (any GGUF model)

```bash
sigma-pkg install llama-cpp
# Place a GGUF model at:
# ~/.cache/sigma/models/tinyllama-1.1b-chat-q4_0.gguf
```

### Fallback: Offline mode

When no LLM is available, sigma-agent uses built-in responses for common SigmaOS topics and falls back to direct tool execution.

---

## Agentic Multi-step Reasoning

For complex goals, sigma-agent breaks them down automatically using the ReAct pattern:

```
σ ~/code › install sigma-edit and configure dark mode

  Planning multi-step task...

  Step 1: install sigma-edit
  ✓ Installed sigma-edit 1.2.0

  Step 2: configure dark mode
  ✓ appearance.theme = zenith-dark
```

Multi-step triggers: `"X and Y"`, `"set up X"`, `"setup X"`

---

## Code Editing (Aider-style)

```bash
# Edit a file with AI
sigma-agent "fix src/main.rs add error handling"
sigma-agent "refactor userland/agent/sigma_agent.rs extract tool registry"
sigma-agent "review kernel/sched/mlfq.rs"

# With explicit code_edit tool
sigma-agent-core --trust full --verbose "code_edit file=src/main.rs instruction='add error handling'"
```

Code edits show a coloured diff before applying:
```diff
--- a/src/main.rs
+++ b/src/main.rs
 fn process(input: &str) -> Result<String, Error> {
-    let val = risky_op(input);
-    Ok(val)
+    let val = risky_op(input)?;
+    Ok(val)
 }
```

---

## File Structure

```
userland/agent/
├── main.rs                         ← Rust binary entry point (sigma-agent-core)
├── sigma_agent.rs                  ← 10 core tools (Rust)
├── sigma_agent_core.rs             ← Intent parser + Agent + REPL
├── sigma_agent_tools_ext.rs        ← 10 extended tools (AI, code, GUI)
├── sigma_llm.rs                    ← LLM backends (llama.cpp/Ollama/sigma-ai/null)
├── sigma_agent_planner.rs          ← ReAct planner + command suggestor
├── sigma_agent_code.rs             ← Code editing + diff + git (Aider-style)
├── sigma_agent_session.nim         ← Session manager + memory + streaming
├── sigma_agent_main.nim            ← CLI master entry point + subcommand router
├── sigma_agent_config.nim          ← Profile system + model management
├── sigma_agent_training.nim        ← Training data + fine-tuning pipeline
├── sigma_agent_gui_mirror.nim      ← 60+ GUI→CLI complete mapping
├── sigma_agent_watch.nim           ← File watcher + proactive AI suggestions
├── sigma_agent_shell_integration.nim ← Shell hooks + keybindings + aliases
├── Cargo.toml                      ← Rust crate manifest (sigma-agent-core)
├── sigma_agent.nimble              ← Nim package definition
├── sigma_agent_ci.yml              ← CI: build/lint/test/seed-dataset
└── README.md                       ← Developer documentation
```

---

## Build & Install

```bash
# Build Nim CLI (sigma-agent)
nim c -d:release --opt:speed -o:sigma-agent sigma_agent_main.nim
cp sigma-agent /usr/bin/

# Build Rust engine (sigma-agent-core) — optional, improves accuracy
cargo build --release -p sigma-agent-core
cp target/release/sigma-agent-core /usr/bin/

# Or install via sigma-pkg (when published)
sigma-pkg install sigma-agent

# Setup shell integration
sigma-agent install --shell-integration
```

---

## CI/CD

The sigma-agent CI pipeline (`.github/workflows/sigma_ci.yml` + `userland/agent/sigma_agent_ci.yml`) runs:
1. **Build**: Nim CLI + Rust engine
2. **Lint**: `nim check` all Nim files
3. **Test tools**: Smoke tests for all 20 tools
4. **GUI mirror test**: Validates all 60+ GUI→CLI mappings
5. **Seed dataset**: Generates `seed_samples.jsonl` as CI artifact

---

## Inspiration

| Project | What we took |
|---|---|
| [Claude Code](https://github.com/anthropics/claude-code) | ReAct loop, streaming, tool calling, REPL design |
| [Aider](https://github.com/Aider-AI/aider) | Git-aware code editing, file watching, diff display |
| [llama.cpp](https://github.com/ggml-org/llama.cpp) | Local LLM inference, GGUF model format, ChatML prompts |
| [ai-shell](https://github.com/BuilderIO/ai-shell) | Natural language → shell commands, error fixing |
| [copilot-cli](https://github.com/github/copilot-cli) | Shell integration, `??` explain, `!` execute patterns |
| [azure-cli](https://github.com/Azure/azure-cli) | Comprehensive command surface, subcommand structure |
| [openclaw](https://github.com/openclaw/openclaw) | GUI parity principle: every GUI action = CLI equivalent |
| [Hermes IDE](https://github.com/hermes-hq/hermes-ide) | IDE-style agent with file context awareness |

---

*Sovereign AI — no telemetry, no cloud required, privacy-first.*
*See also: [Architecture Overview](Architecture-Overview) · [Zenith Desktop](Zenith-Desktop) · [sigma-cli man page](sigma-cli-man-page)*
