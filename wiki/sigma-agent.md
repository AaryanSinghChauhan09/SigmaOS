# sigma-agent — AI CLI Agent for SigmaOS

> Every GUI action, accessible from the terminal via natural language.
> Sovereign. Local. Learns from you. Automates everything.

---

## What it does

`sigma-agent` is SigmaOS's native AI CLI agent — 35 modules covering:
natural language → OS commands, n8n-style workflow automation, security auditing,
RLHF fine-tuning, multi-agent specialisation, persistent memory, voice input,
self-diagnosis, smart completions, benchmarking, and plugin extensions.

```
σ ~/code › workflow run weekly-backup

Σ Workflow: weekly-backup  [manual]
  Backup Code and Documents every Friday night
  Steps: 4

  [1/4] backup-code                ✓  892ms
  [2/4] backup-docs                ✓  341ms
  [3/4] disk-check                 ✓  45ms
  [4/4] done                       ✓  12ms

  ✓ PASS  Workflow: weekly-backup
```

---

## Quick Start

```bash

# Install

nim c -d:release --opt:speed -o:sigma-agent sigma_agent_main.nim
cp sigma-agent /usr/bin/

# Verify

sigma-agent doctor

# Set up shell integration

sigma-agent install --shell-integration

# Install workflow templates

sigma-agent workflow install --all

# Start background daemon (knowledge sync + completions)

sigma-agent daemon start
```

---

## All Subcommands (20 subcommands, 35 modules)

| Subcommand | What it does |
|---|---|
| *(no args)* | Interactive REPL |
| `"<command>"` | One-shot natural language |
| `--script <file>` | Run a `.sa` script |
| `--pipe` | Read commands from stdin |
| `doctor` | Self-diagnosis (like `claude doctor`) |
| `update` | Self-update from GitHub releases |
| `daemon` | Background HTTP service + knowledge sync |
| `context` | Live OS state snapshot |
| `security` | Security audit + policy advisor |
| `learn` | RLHF feedback + DPO fine-tuning |
| `multi` | Multi-agent routing (6 specialists) |
| `voice` | Voice input (Whisper STT) |
| `memory` | Persistent facts/prefs (CLAUDE.md style) |
| `script-gen` | NL → `.sa` script generator |
| `explain` | Explain commands/concepts (copilot-cli `??`) |
| **`workflow`** | **n8n-style automation pipelines** |
| `plugin` | Skill extension system |
| `complete` | LLM-powered tab completion |
| `tui` | Dashboard, fuzzy picker, interactive diff |
| `benchmark` | 40-test quality benchmark suite |
| `notify` | Desktop notifications + event watcher |
| `train` | Training dataset + GitHub sync + A/B |
| `watch` | File watcher + AI suggestions |
| `mirror` | GUI→CLI mapping explorer (60+) |
| `config` | Profile system + model management |
| `install` | Shell integration |

---

## Workflow Automation (n8n-style)

```bash

# Install all 8 built-in templates

sigma-agent workflow install --all

# Templates: weekly-backup, daily-update, cpu-alert, low-disk-alert,

#            dev-workflow, security-hardening, on-boot-setup, pkg-update-notify

# Run a workflow

sigma-agent workflow run weekly-backup
sigma-agent workflow run weekly-backup --dry-run
sigma-agent workflow run dev-workflow --verbose

# Generate from natural language

sigma-agent workflow create "backup home folder every Friday"
sigma-agent workflow create "run security scan every night at 23:00" -o nightly.yaml

# Manage

sigma-agent workflow list
sigma-agent workflow enable weekly-backup
sigma-agent workflow disable cpu-alert
sigma-agent workflow history
sigma-agent workflow audit

# Background scheduler (checks triggers every 60s)

sigma-agent workflow scheduler
```

Workflow YAML format:
```yaml
name: my-workflow
description: "What this does"
enabled: true
trigger: schedule=daily 06:00    # or: manual, cpu>90, disk<10, pkg_update, boot

steps:
  - name: update
    action: "run sigma-pkg update"
    on_fail: notify
    timeout: 300
    retries: 1
  - name: scan
    action: "security scan"
    condition: "exit_code_of(update) == 0"
    on_fail: continue
  - name: done
    action: "notify 'Done' 'Update and scan complete'"
```

Trigger formats: `manual`, `schedule=daily HH:MM`, `schedule=every friday 22:00`,
`schedule=*/30min`, `cpu>90`, `disk<10`, `pkg_update`, `boot`, `file:/path`

---

## Files (35 modules)

```
userland/agent/
├── main.rs + Cargo.toml              Rust engine (sigma-agent-core)
├── sigma_agent.rs/.core.rs/...       Rust tool implementations
├── sigma_agent_main.nim              CLI master entry (35 modules)
├── sigma_agent_workflow.nim          n8n-style workflow engine  ← NEW
├── sigma_agent_memory.nim            Persistent memory (CLAUDE.md)
├── sigma_agent_script_gen.nim        NL → .sa script generator
├── sigma_agent_explain.nim           Explain mode (copilot-cli ??)
├── sigma_agent_daemon.nim            HTTP daemon + /v1/complete
├── sigma_agent_context.nim           OS context engine
├── sigma_agent_security.nim          Security advisor
├── sigma_agent_learn.nim             RLHF + DPO fine-tuning
├── sigma_agent_multi.nim             Multi-agent orchestration
├── sigma_agent_voice.nim             Voice input (Whisper)
├── sigma_agent_plugin.nim            Plugin skill system
├── sigma_agent_autocomplete.nim      Smart tab completion
├── sigma_agent_tui.nim               TUI components
├── sigma_agent_benchmark.nim         Benchmark suite (40 tests)
├── sigma_agent_notify.nim            Notifications + events
├── sigma_agent_doctor.nim            Self-diagnosis
├── sigma_agent_update.nim            Self-update
├── sigma_agent_training.nim          Training + sync + A/B
├── sigma_agent_gui_mirror.nim        60+ GUI→CLI mappings
├── sigma_agent_watch.nim             File watcher
├── sigma_agent_shell_integration.nim Shell hooks
├── sigma_agent_session.nim           Session manager
├── sigma_agent_config.nim            Profile system
├── sigma_agent_seed_v2.jsonl         55 training samples
├── sigma_agent.nimble                Package (v15.1.0)
├── sigma_agent_ci.yml                12-job CI pipeline
└── README.md
```

---

## Build

```bash
cd userland/agent
nim c -d:release --opt:speed -o:sigma-agent sigma_agent_main.nim
cargo build --release -p sigma-agent-core
cp sigma-agent /usr/bin/
cp ../../target/release/sigma-agent-core /usr/bin/
```

Or: `sigma-pkg install sigma-agent`

---

## Training

```bash
sigma-agent train seed          # 65+ built-in samples (v1 + v2)

sigma-agent train sync          # pull GitHub wiki → samples

sigma-agent learn rate good     # rate interactions as you use it

sigma-agent learn build v1      # build fine-tuning dataset

sigma-agent learn finetune tinyllama-1.1b sigma-v1
```

---

### Sovereign AI — local inference, no telemetry, no external APIs.

### Docs: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/sigma-agent
