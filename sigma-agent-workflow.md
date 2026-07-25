# sigma-agent workflow — n8n-style Automation Engine

> Automate any OS task using YAML pipelines, event triggers, and natural language.
> Sovereign. No external services required.

---

## Overview

`sigma-agent workflow` is SigmaOS's built-in automation engine — inspired by n8n, Claude Code multi-step execution, and azure-cli automation runbooks. Define workflows in YAML (or generate them from plain English), trigger them on schedules, events, or manually, and let sigma-agent execute every step.

```
σ ~/code › workflow run dev-workflow --verbose

Σ Workflow: dev-workflow  [manual]
  Build, test, and notify on completion
  Steps: 4

  → Step: build
    Action: run cargo build --release
  [1/4] build                     ✓  42.3s

  → Step: test
    Action: run cargo test
  [2/4] test                      ✓  8.1s

  [3/4] review                    ✓  2.4s
  [4/4] done                      ✓  0.1s

  ✓ PASS  Workflow: dev-workflow
```

---

## Quick Start

```bash
# Install all 8 built-in templates
sigma-agent workflow install --all

# List installed workflows
sigma-agent workflow list

# Run a workflow
sigma-agent workflow run weekly-backup

# Preview without executing
sigma-agent workflow run weekly-backup --dry-run

# Generate from natural language
sigma-agent workflow create "back up my code every Friday night"

# Start background scheduler (runs all schedule/event workflows)
sigma-agent workflow scheduler
```

---

## Workflow YAML Format

Workflows are stored as `.yaml` files in `~/.config/sigma/agent/workflows/`.

```yaml
name: my-workflow
description: "What this workflow does"
enabled: true
trigger: schedule=daily 06:00

env:
  BACKUP_DIR: /backup/home
  KEEP_DAYS: "7"

steps:
  - name: step-one
    action: "sigma-agent natural language command"
    on_fail: stop        # stop | continue | notify
    timeout: 60          # seconds, default 60
    retries: 0           # retry count on failure

  - name: step-two
    action: "run some shell command"
    condition: "exit_code_of(step-one) == 0"
    on_fail: notify

  - name: notify-done
    action: "notify 'Done' 'Workflow complete'"
    on_fail: continue
```

### Trigger Formats

| Trigger | Description |
|---|---|
| `manual` | Run on demand only (default) |
| `schedule=daily 06:00` | Every day at 06:00 |
| `schedule=every friday 22:00` | Every Friday at 22:00 |
| `schedule=*/30min` | Every 30 minutes |
| `schedule=*/1h` | Every hour |
| `schedule=weekly` | Once a week |
| `cpu>90` | When CPU load average exceeds 90% |
| `disk<10` | When free disk space drops below 10% |
| `pkg_update` | When package updates are available |
| `boot` | On every system boot (once per session) |
| `file:/path/to/watch` | When a specific file changes |
| `network:down` | When a network interface goes down |

### Step Conditions

Conditions let steps run only when previous steps succeeded or produced specific output:

```yaml
# Only run if 'build' step exited with code 0
condition: "exit_code_of(build) == 0"

# Only run if 'build' step succeeded
condition: "success_of(build)"

# Only run if 'scan' output contains "critical"
condition: "output_contains(scan, 'critical')"
```

### on_fail Options

| Value | Behaviour |
|---|---|
| `stop` | Halt workflow immediately (default) |
| `continue` | Log error but continue to next step |
| `notify` | Send critical desktop notification then stop |

---

## Built-in Templates

Install with `sigma-agent workflow install <name>` or `sigma-agent workflow install --all`.

### `weekly-backup`
Backs up `~/Code` and `~/Documents` every Friday at 22:00.
```bash
sigma-agent workflow install weekly-backup
sigma-agent workflow run weekly-backup --dry-run
```

### `daily-update`
Updates all packages and runs a security scan daily at 06:00.

### `cpu-alert`
Fires when CPU load exceeds 90% — diagnoses the cause and sends a critical alert.

### `low-disk-alert`
Fires when free disk space drops below 10% — alerts and suggests cleanup.

### `dev-workflow`
Manual workflow: `cargo build` → `cargo test` → security review → notify.

### `security-hardening`
Manual workflow: full security scan → enable firewall → disable telemetry → policy recommendations.

### `on-boot-setup`
Runs `sigma-agent doctor` and syncs knowledge from GitHub on every boot.

### `pkg-update-notify`
Notifies you when package updates are available.

---

## Natural Language → Workflow

Generate a complete YAML workflow from a plain English description:

```bash
# Print to stdout
sigma-agent workflow create "back up home folder every Friday"

# Save to file
sigma-agent workflow create "run security audit nightly at 23:00" -o nightly-audit.yaml

# Generate and install immediately
sigma-agent workflow create "monitor CPU and alert when high" -o cpu-watch.yaml
cp cpu-watch.yaml ~/.config/sigma/agent/workflows/

# Generate and run immediately
sigma-agent workflow create "build and test my project" -o /tmp/build.yaml
sigma-agent workflow run /tmp/build.yaml
```

Works offline with rule-based planner. When sigma-agent daemon is running, uses LLM for richer workflows.

---

## Scheduler

The scheduler checks all workflow triggers every 60 seconds:

```bash
# Start in foreground (Ctrl+C to stop)
sigma-agent workflow scheduler

# Start via daemon (recommended)
sigma-agent daemon start  # daemon includes scheduler automatically

# Check what would fire right now
sigma-agent workflow check
```

The scheduler:
- Runs schedule-based workflows at their configured time
- Fires event-based workflows when conditions are met (CPU/disk/pkg)
- Debounces event triggers (minimum 5 minutes between firings)
- Logs every run to `~/.cache/sigma/agent/workflow_runs/`
- Appends to audit log at `~/.cache/sigma/agent/workflow_audit.log`

---

## Run History & Audit

```bash
# Show last 20 runs
sigma-agent workflow history

# Show runs for a specific workflow
sigma-agent workflow history weekly-backup

# Show audit log (every action logged)
sigma-agent workflow audit
```

Every workflow run is saved as JSON in `~/.cache/sigma/agent/workflow_runs/`.
Every action (create, run, enable, disable) is appended to the audit log.

---

## Workflow Management

```bash
sigma-agent workflow list                  # list all workflows
sigma-agent workflow enable weekly-backup  # enable
sigma-agent workflow disable cpu-alert     # disable (won't auto-trigger)
sigma-agent workflow delete my-workflow    # delete permanently
sigma-agent workflow edit weekly-backup    # open YAML in $EDITOR
sigma-agent workflow templates             # list built-in templates
```

---

## Integration with Other sigma-agent Features

Workflow steps can use **any sigma-agent command**:

```yaml
steps:
  # Security advisor
  - name: audit
    action: "security scan"

  # Multi-agent diagnosis
  - name: diagnose
    action: "multi --agent sysadmin 'why is system slow'"

  # Memory
  - name: remember
    action: "memory add 'last backup ran successfully' --pattern"

  # Notification
  - name: alert
    action: "notify 'Workflow' 'Step complete' --critical"

  # Raw shell
  - name: compress
    action: "run tar -czf /backup/code.tar.gz /home/user/Code"

  # Explain what happened
  - name: explain-error
    action: "explain --error 'cargo build' 'linker error'"
    condition: "exit_code_of(build) != 0"
```

---

## Example: Full Dev Pipeline

```yaml
name: dev-pipeline
description: "Full build, test, security check, deploy"
enabled: true
trigger: manual

steps:
  - name: fetch
    action: "run git pull --rebase"
    on_fail: stop

  - name: build
    action: "run cargo build --release"
    on_fail: stop
    timeout: 300
    retries: 1

  - name: test
    action: "run cargo test"
    on_fail: notify
    condition: "exit_code_of(build) == 0"

  - name: security
    action: "security scan --quick"
    on_fail: continue

  - name: remember-success
    action: "memory add 'last build succeeded on main branch' --pattern"
    on_fail: continue
    condition: "success_of(test)"

  - name: done
    action: "notify 'Pipeline complete' 'Build, test, security all passed'"
    on_fail: continue
    condition: "success_of(test)"
```

---

## File Locations

| Path | Purpose |
|---|---|
| `~/.config/sigma/agent/workflows/*.yaml` | Workflow definitions |
| `~/.cache/sigma/agent/workflow_runs/*.json` | Run history (JSON) |
| `~/.cache/sigma/agent/workflow_schedule.json` | Next-run timestamps |
| `~/.cache/sigma/agent/workflow_audit.log` | Audit trail (every action) |

---

## Inspiration

| Project | What we took |
|---|---|
| [n8n](https://n8n.io) | Node-based pipelines, event triggers, YAML format |
| [Claude Code](https://github.com/anthropics/claude-code) | Multi-step task execution, step conditions |
| [azure-cli](https://github.com/Azure/azure-cli) | `az automation runbook`, scheduled jobs |
| [Aider](https://github.com/Aider-AI/aider) | Automated code fix pipelines |
| [OpenClaw](https://github.com/openclaw/openclaw) | Event-driven agent actions |

---

*See also: [sigma-agent](sigma-agent) · [Architecture Overview](Architecture-Overview) · [Security Model](Security-Model)*
