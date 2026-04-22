# Sovereign Journal Shard

**Parity:** systemd-journal · syslog · Windows Event Log · macOS ASL  
**Location:** `kernel/modules/core/SovereignJournalShard.c`  
**Standard:** Zenith Industrial Sovereignty v1.0

---

## Overview

The Sovereign Journal Shard provides native, zero-dependency structured kernel event logging for SigmaOS. It absorbs the systemd-journal, syslog, and Windows Event Log USPs by implementing a lock-free circular ring buffer with 8-level priority filtering, ANSI-coloured severity output, and `journalctl -f`-style streaming.

---

## Architecture

```
Ring Buffer: 64 entries (circular, auto-overwrite oldest)
  Entry: seq | timestamp_us | level | unit | message

Log Levels (syslog-compatible):
  0=EMERG  1=ALERT  2=CRIT  3=ERR  4=WARN  5=NOTICE  6=INFO  7=DEBUG

Boot Entries (pre-seeded):
  [INFO]    sigma_kernel   "Sovereign kernel journal online."
  [NOTICE]  sigma_sched    "Zen Scheduler armed and ready."
  [INFO]    sigma_tele     "eBPF probes seated."
  [WARNING] sigma_oom      "Memory pressure elevated at boot."
```

---

## CLI Reference — `sigma-journal`

| Sub-command | Action |
|---|---|
| `sigma-journal write <level_num> <unit> <msg>` | Write a structured event entry (0=EMERG…7=DEBUG) |
| `sigma-journal follow <min_level_num>` | Stream all entries at or above the minimum priority |
| `sigma-journal audit` | Print ring buffer stats and full event stream |

---

## Design Philosophy

- **Lock-Free Ring**: Circular overwrite eliminates allocation and mutex overhead.
- **CRIT+ Immediate**: `EMERG`, `ALERT`, `CRIT` entries bypass the buffer and print immediately.
- **Structured Fields**: Every entry carries `seq`, `timestamp_us`, `unit`, and `message` for machine-readable analysis.

---

## Synchronization State

`GLOBAL MESH ACTIVE` — Synchronized with `AaryanSinghChauhan09/SigmaOS`.
