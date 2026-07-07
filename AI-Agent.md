# AI Agent Features

> SigmaOS v15.0 "Zenith" — Local AI Subsystem Reference

## Overview

SigmaOS ships with a fully **offline, local AI agent** stack. No cloud dependency, no telemetry. All inference runs on-device using quantized models (default: `phi-2-q4.gguf`).

---

## Components

### 1. AI Agent (`agents/sigma_ai_agent.rs`)

The core NL→CLI translator and log analyzer.

**Features:**
- Natural language to shell command translation
- Contextual error log analysis with root-cause explanations
- Conversation history tracking (up to 2048 tokens context)
- Pluggable model backend (default: llama.cpp-compatible GGUF)

**Example:**
```
$ sigma-ai "update all packages"
→ sigpkg upgrade

$ sigma-ai "why is nginx failing?"
→ Root cause: Port 80 is already in use by another process.
   Fix: Run `sigpkg ps | grep :80` to find and stop the conflicting service.
```

---

### 2. Workflow Engine (`agents/sigma_workflow_engine.rs`)

Node-based automation engine (inspired by n8n).

**Node Types:**

| Type | Description |
|---|---|
| `TriggerCron` | Schedule-based trigger (cron expression) |
| `TriggerFile` | Trigger when file changes |
| `ActionShell` | Execute shell command |
| `ActionAiAnalyze` | Pass data to AI agent for analysis |
| `ConditionContains` | Branch on string match |
| `OutputLog` | Write result to structured log |

**Workflow Definition (JSON):**
```json
{
  "id": 1,
  "name": "Auto-Restart Failed Services",
  "nodes": [
    { "id": 1, "type": "TriggerFile", "path": "/var/log/journal.db", "next": [2] },
    { "id": 2, "type": "ActionAiAnalyze", "next": [3] },
    { "id": 3, "type": "ConditionContains", "value": "failed", "next": [4] },
    { "id": 4, "type": "ActionShell", "cmd": "sigma-init restart $SERVICE" }
  ]
}
```

---

### 3. Adaptive CLI (`agents/sigma_adaptive_cli.rs`)

Wraps any shell command and intercepts errors to suggest corrections:

```
$ sigpkg instll firefox
  Error: Unknown command 'instll'
  Suggestion: Did you mean 'install'?
  Run: sigpkg install firefox? [Y/n]
```

---

### 4. Error Explainer (`agents/sigma_error_explainer.rs`)

Hooks into the kernel panic handler and crash reporter:

- Parses kernel panic strings, segfault addresses, and OOM messages
- Generates a plain-English explanation of the crash
- Logs structured explanation to `sigma_journal`

---

## Model Configuration

Default model: `/usr/share/sigma-ai/models/phi-2-q4.gguf`

To change the model:
```bash
sigma-ai-config set model /path/to/custom.gguf
```

Supported formats: GGUF (Q4_K_M, Q5_K_M, Q8_0)

---

## Privacy Guarantee

- **No network access**: All inference is local
- **No logging to disk** by default (opt-in journaling)
- **Conversation cleared** on session end unless `--persist` flag is set
