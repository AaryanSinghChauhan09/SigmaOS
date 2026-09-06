# AI Agent Process Management Wiki — SigmaOS Documentation

## Overview

In **SigmaOS**, autonomous AI coding agents, subagent worker threads, and assistant processes operate as managed tasks under the kernel process manager (`src/kernel/process.rs`), AI orchestrator (`src/ai/`), and process cancellation & termination governor (`ProcessCancellationAndTerminationManager`).

This document provides a comprehensive reference on AI agent process architecture, cgroup isolation, capability sandboxing, zero-copy IPC messaging, and scheduling priorities.

---

## 1. Process Architecture & Spawning

AI agents are created using `ProcessBuilder` with strict security defaults:

```rust
use sigmaos::process::{CapSet, ProcessBuilder, SandboxPolicy};

let agent_process = ProcessBuilder::new("/usr/bin/herdr-agent")
    .args(&["--subagent", "compiler-checker", "--parent-pid", "1001"])
    .uid(1001)
    .gid(1001)
    .capabilities(CapSet::minimal())
    .sandbox(SandboxPolicy::strict())
    .cgroup("/sys/fs/cgroup/system.slice/sigma-agent.service")
    .spawn()?;
```

---

## 2. Resource Allocation (Cgroups v2)

Agents execute under the `/sys/fs/cgroup/system.slice/sigma-agent.service` controller slice:

| Resource Controller | Configuration | Purpose |
|---------------------|---------------|---------|
| `cpu.max` | `80000 100000` | Limits agent processes to 80% CPU time max |
| `memory.high` | `1536M` | Triggers proactive memory reclamation |
| `memory.max` | `2048M` | Enforces hard OOM memory ceiling |
| `pids.max` | `64` | Restricts max nested subagent threads |

---

## 3. Capability Sandboxing (`pledge` & `unveil`)

All agent processes are sandboxed via OpenBSD-style syscall filtering:

```rust
// Restrict agent to standard I/O, file reading/writing, and network sockets
agent_process.pledge(&["stdio", "rpath", "wpath", "inet", "dns"])?;

// Reveal specific directory trees
agent_process.unveil("/userland/workspace", "rwc")?;
agent_process.unveil("/tmp", "rwc")?;
agent_process.unveil_finalize()?;
```

---

## 4. Zero-Copy IPC & Subagent Communication

Subagents communicate with parent orchestrator threads via zero-copy message queues (`ZeroCopyIpcChannel`) or secure Binder handles (`AndroidBinderIpc`):

```rust
use sigmaos::process::ZeroCopyIpcChannel;

let mut channel = ZeroCopyIpcChannel::new(orchestrator_pid, subagent_pid)?;
channel.send_message(b"TASK_STATUS: Code generation completed without errors.")?;
```

---

## 5. Scheduling Priority & Preemption

- **Interactive Agents** (GUI assistance, IDE completion): Scheduled with high interactivity scores under the FreeBSD ULE / BORE hybrid scheduler.
- **Background Batch Agents** (Codebase analysis, test generation): Scheduled under `SCHED_BATCH` or `SCHED_IDLE` to prevent user interface jitter.
