# SigmaOS Process Management Architecture & AI Agent Guidelines

## Overview

SigmaOS process management (`src/kernel/process.rs`, `src/kernel/scheduler.rs`, `src/process/`) provides a hybrid BORE+EEVDF scheduler, cgroups v2 resource controllers, capability-bounded sandboxing (`pledge`/`unveil`), zero-copy IPC channels, and dedicated process policies for autonomous AI agent workloads.

---

## 1. Process Lifecycle

```
fork() / spawn() → Ready → Running → Blocked → Ready
                                │
                            Terminated → Zombie → Reaped
```

---

## 2. Hybrid BORE+EEVDF Scheduler

The scheduler combines:
- **EEVDF** (Earliest Eligible Virtual Deadline First): Guarantees fair CPU time distribution.
- **BORE** (Burst-Oriented Response Enhancer): Dynamically boosts interactive task responsiveness (e.g. desktop UI, IDE completions).

### Priority Classes & Agent Policies

| Class | Nice Range | Description | AI Agent Application |
|-------|------------|-------------|----------------------|
| `SCHED_FIFO` | -20 to -1 | Real-time FIFO (no preemption) | Hardware IRQ handlers, audio streams |
| `SCHED_RR` | -20 to -1 | Real-time round-robin | Real-time sensor processing |
| `SCHED_NORMAL` | 0 | Default timesharing | Interactive AI assistants (e.g. Herdr CLI, copilot completion) |
| `SCHED_BATCH` | 1 to 19 | CPU-bound batch jobs | Background AI indexing, codebase analysis subagents |
| `SCHED_IDLE` | 20 | Runs only when CPU is idle | Deep model pre-caching, telemetry log archiving |

---

## 3. Cgroups v2 Resource Governance for AI Agents

All AI agent processes spawned in SigmaOS are automatically assigned to the `/sys/fs/cgroup/system.slice/sigma-agent.service` controller slice:

```
/sys/fs/cgroup/
├── user.slice/
│   └── user-1000.slice/
│       └── sigma-session.scope/
└── system.slice/
    ├── sigma-daemon.service/
    └── sigma-agent.service/
        ├── cpu.max = 80000 100000     (80% CPU cap)
        ├── memory.high = 1536M        (proactive memory reclamation)
        ├── memory.max = 2048M         (hard OOM limit)
        └── pids.max = 64              (max nested subagent threads)
```

---

## 4. Capability Sandboxing (`pledge` & `unveil`)

AI agent processes execute under OpenBSD-inspired capability restrictions:

```rust
use sigmaos::process::{ProcessBuilder, SandboxPolicy, CapSet};

// Spawn capability-gated agent subprocess
let child = ProcessBuilder::new("/usr/bin/herdr-agent")
    .args(&["--task", "refactor-module"])
    .uid(1000).gid(1000)
    .capabilities(CapSet::minimal())
    .sandbox(SandboxPolicy::strict())
    .spawn()?;

// Restrict syscall promises & filesystem view
child.pledge(&["stdio", "rpath", "wpath", "inet", "dns"])?;
child.unveil("/userland/workspace", "rwc")?;
child.unveil("/tmp", "rwc")?;
child.unveil_finalize()?;
```

---

## 5. POSIX Signals & Process Termination

Standard POSIX signals govern agent process state transitions:

| Signal | Number | Default Action | AI Agent Handling |
|--------|--------|----------------|-------------------|
| `SIGTERM` | 15 | Terminate | Graceful subagent state checkpoint & shutdown |
| `SIGKILL` | 9 | Terminate | Immediate process termination (cannot be caught) |
| `SIGSTOP` | 19 | Stop | Pause subagent execution thread |
| `SIGCONT` | 18 | Continue | Resume paused subagent thread |
| `SIGCHLD` | 17 | Ignore | Parent orchestrator reaps zombie subagents |

---

## 6. Zero-Copy Inter-Process Communication (IPC)

AI agents coordinate across process boundaries using zero-copy IPC message queues (`ZeroCopyIpcChannel`) or secure Binder handles (`AndroidBinderIpc`):

```rust
use sigmaos::process::ZeroCopyIpcChannel;

let mut channel = ZeroCopyIpcChannel::new(parent_agent_pid, child_agent_pid)?;
channel.send_message(b"SUBTASK_COMPLETED: Unit tests passed 100%.")?;
```
