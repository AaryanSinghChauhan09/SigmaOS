# Diagnostics and Crash Reporting

SigmaOS implements a comprehensive diagnostics and crash reporting subsystem in Rust. It captures kernel panics, process crashes, hardware faults, and system anomalies — then feeds them to the AI crash analyzer for automatic root-cause analysis and fix suggestions.

---

## Architecture Overview

```
 Crash / Fault Event
       │
 ┌─────▼────────────────────────────────────────────┐
 │          Crash Capture Layer (src/crash/)          │
 │  Kernel Oops │ Process SIGSEGV │ Watchdog timeout  │
 └─────────────────────┬────────────────────────────┘
                       │
 ┌─────────────────────▼────────────────────────────┐
 │          Crash Dump Writer                        │
 │  ELF core dump │ Kernel minidump │ Structured log │
 └─────────────────────┬────────────────────────────┘
                       │
 ┌─────────────────────▼────────────────────────────┐
 │          AI Crash Analyzer (src/ai/)              │
 │  Root cause inference │ CVE correlation            │
 │  Fix suggestion │ Regression detection             │
 └─────────────────────┬────────────────────────────┘
                       │
 ┌─────────────────────▼────────────────────────────┐
 │          Reporting                                │
 │  Local log │ Notification │ Optional telemetry    │
 └────────────────────────────────────────────────  ┘
```

---

## Crash Types Handled

| Crash Type | Source | Action |
|-----------|--------|--------|
| Kernel panic | `src/crash/` | Save kdump, reboot |
| Kernel oops | Trap handler | Log + continue |
| Process SIGSEGV | Signal delivery | Core dump + AI analysis |
| Process SIGABRT | libc abort | Core dump + stack trace |
| Hardware MCE | Machine Check Exception | Log + offline CPU core |
| Watchdog timeout | Kernel watchdog | Log + restart service |
| OOM kill | Memory pressure | Log victim + freed memory |
| Driver crash | Driver watchdog | Reload driver, log |

---

## Kernel Crash Dump (kdump)

### Configuration
```toml
[kdump]
enabled = true
crashkernel = "256M"         # Reserved RAM for crash kernel
dump_path = "/var/crash/"
compression = "lz4"
max_dumps = 5                # Rotate oldest
```

### Capture Process
1. Primary kernel panics
2. NMI triggers secondary crash kernel (pre-loaded at boot)
3. Crash kernel captures RAM snapshot via `/dev/mem`
4. Writes ELF vmcore to `/var/crash/YYYY-MM-DD-HH:MM/`
5. AI analyzer reads vmcore immediately after write

### Analyzing a Dump
```bash
sigma-crash analyze /var/crash/2025-10-04/vmcore
```
Output:
```
╔══════════════════════════════════════════════╗
║  SigmaOS Crash Analysis Report               ║
╠══════════════════════════════════════════════╣
║  Crash type: NULL pointer dereference        ║
║  Function:   sigma_net_rx_handler+0x2a4      ║
║  Module:     sigma-ethernet                  ║
║  Likely cause: RCU read lock not held        ║
║  Similar CVE: CVE-2024-12345 (fixed in 1.1)  ║
║  Suggested fix: Update sigma-ethernet driver  ║
╚══════════════════════════════════════════════╝
```

---

## Process Crash Reporting

### Core Dump
- ELF format with all thread backtraces
- Saved to `~/crashes/<process>-<pid>-<timestamp>.core`
- Symbolicated automatically using DWARF debug info

### GUI Notification
When a graphical app crashes:
```
⚠ Application Crashed: sigma-browser
  The app stopped unexpectedly.
  [View Details]  [Report]  [Dismiss]
```

Clicking "View Details" shows:
- Backtrace
- Last 100 log lines
- AI-suggested fix
- Link to relevant wiki page

### Automatic Restart Policy
```toml
[crash.policy]
max_restarts = 3
restart_window_sec = 60
action_after_max = "notify"   # notify | disable | ai-recover
```

---

## System Health Watchdog

### Hardware Watchdog
- Kicks NMI watchdog every 30s
- If kernel hangs (no kick), triggers panic + kdump

### Service Watchdog
- sigma-init monitors all service PIDs
- Missing heartbeat → restart with exponential backoff

### AI Anomaly Watchdog
- Continuously monitors CPU/RAM/IO/net metrics
- Detects anomalies (e.g., runaway process, memory leak)
- Proactively alerts before crash occurs

---

## Kernel Tracing (`src/tracing/`)

### Ftrace Equivalent
```bash
# Trace function calls in the kernel
sigma-trace function --filter "sigma_fs_*" --duration 5s

# Trace scheduler events
sigma-trace sched --pid 1234 --duration 10s

# Live trace output
sigma-trace live --events page_fault,irq_handler
```

### eBPF-Style Probes
```bash
# Attach probe to kernel function
sigma-probe attach sigma_net_rx_handler --on-entry "log args"

# Attach probe to userspace function
sigma-probe attach --pid 1234 --function main
```

---

## Logging System

### Log Levels
| Level | Value | Use |
|-------|-------|-----|
| Emergency | 0 | System unusable |
| Alert | 1 | Immediate action required |
| Critical | 2 | Critical conditions |
| Error | 3 | Error conditions |
| Warning | 4 | Warning conditions |
| Notice | 5 | Normal but significant |
| Info | 6 | Informational |
| Debug | 7 | Debug messages |

### Log Storage
- Structured JSON logs: `/var/log/sigma/kernel.jsonl`
- Per-service logs: `/var/log/sigma/<service>.log`
- Log rotation: 7 days retention, max 500 MB per service
- Queryable: `sigma-log query --level error --since 1h`

---

## Diagnostics CLI

```bash
# Full system diagnostic report
sigma-diag report

# Check specific subsystem
sigma-diag check storage
sigma-diag check network
sigma-diag check memory

# Live performance view
sigma-diag live

# Export diagnostics bundle (for support)
sigma-diag bundle --output ~/sigma-diagnostics.tar.zst
```

---

## Privacy: Crash Telemetry

All crash reports are **opt-in**:
```toml
[telemetry]
enabled = false              # off by default
include_core_dump = false    # never send full core dumps
anonymize = true             # strip usernames, paths
endpoint = "https://telemetry.sigmaos.org/v1/crash"
```

If enabled:
- Only sends: crash type, function name, module, SigmaOS version
- No personal data, no file contents, no passwords

---

## Source Files

| File | Description |
|------|-------------|
| `src/crash/` | Crash capture and dump |
| `src/ai/crash_analyzer.rs` | AI-powered root cause analysis |
| `src/tracing/` | Kernel trace and probes |
| `src/logging/` | Structured log system |
| `src/diagnostics/` | Diagnostic report tools |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/crash/`, `src/ai/crash_analyzer.rs`, `src/diagnostics/`
> - Update crash type table when new fault handlers are added
> - Keep AI analysis example output current
> - Add new `sigma-diag check` subcommands as they are implemented
