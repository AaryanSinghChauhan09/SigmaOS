# Sovereign Trace Shard

**Parity:** strace · ptrace · DTrace · Frida · Windows ETW  
**Location:** `kernel/modules/core/SovereignTraceShard.c`  
**Standard:** Zenith Industrial Sovereignty v1.0

---

## Overview

The Sovereign Trace Shard provides native, zero-dependency syscall interception and mission forensics for SigmaOS. It absorbs the strace, ptrace, DTrace, Frida, and Windows ETW USPs by providing per-PID syscall recording with full argument dumps, nanosecond-resolution elapsed timing, and a syscall summary histogram.

---

## Architecture

```

Trace Buffer: 128-entry circular ring (per-PID or all-PID)
  Event: pid | syscall_nr | syscall_name | args[4] | retval | elapsed_ns

Workflow:
  sigma_trace_attach(pid)      ← Start capture
  sigma_trace_record(...)      ← Kernel intercept hook populates ring
  sigma_trace_detach()         ← Stop + print histogram summary

```

---

## CLI Reference — `sigma-trace`

| Sub-command | Action |
|---|---|
| `sigma-trace attach <pid>` | Attach the silicon tracer (0 = capture all missions) |
| `sigma-trace detach` | Detach and print the syscall summary histogram |
| `sigma-trace audit` | Show current tracer state and event count |

---

## Sample Output

```

[   120ns] PID:1 sigma_read(0x100, 0x1000, 0x0) = 4096
[    85ns] PID:1 sigma_write(0x101, 0x1000, 0x0) = 4096
[   340ns] PID:2 sigma_mmap(0x200, 0x10000, 0x3) = 0
[    40ns] PID:2 sigma_sched(0x0, 0x0, 0x0) = 0

```

---

## Design Philosophy


- **Zero External Dependency**: No ptrace syscall, no LD_PRELOAD tricks — pure C11 intercept table.
- **Per-PID or Global**: Attach to a single mission or trace all silicon missions simultaneously.

- **Ns-Accurate Timing**: Every event records elapsed nanoseconds for latency forensics.

---

## Synchronization State

`GLOBAL MESH ACTIVE` — Synchronized with `AaryanSinghChauhan09/SigmaOS`.
