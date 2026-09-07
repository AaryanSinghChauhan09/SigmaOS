# AI Agent Process Management in SigmaOS

## Overview

SigmaOS treats AI agent workloads (e.g. Herdr agentic subagents, code assistants, automated task executors) as first-class process entities managed under `src/kernel/process.rs`, `src/ai/`, and `src/desktop/omarchy_omakase.rs`.

This document specifies the process lifecycle, scheduling policies, IPC, and security sandboxing required for AI agents.

---

## Process Lifecycle & Creation

```
Agent Request → AgenticWorkstationOrchestrator
                     │
                     ▼
          ProcessBuilder::new("herdr-agent")
                     │
                     ▼
          Capability Sandbox (Pledge/Unveil)
                     │
                     ▼
          Cgroup Attachment (sigma-agent.service)
                     │
                     ▼
       BORE+EEVDF Interactive Hybrid Scheduler
```

### Spawning Agent Subprocesses

```rust
use sigmaos::process::{ProcessBuilder, SandboxPolicy, CapSet};

let agent_proc = ProcessBuilder::new("/usr/bin/herdr-agent")
    .args(&["--role", "code-generator", "--task-id", "42"])
    .uid(1001)
    .gid(1001)
    .capabilities(CapSet::minimal())
    .sandbox(SandboxPolicy::strict())
    .cgroup("/sys/fs/cgroup/system.slice/sigma-agent.service")
    .spawn()?;
```

---

## Cgroups v2 Resource Allocation

AI Agent processes are governed under the dedicated `sigma-agent.service` controller slice:

```
/sys/fs/cgroup/system.slice/sigma-agent.service/
├── cpu.max = 80000 100000        # 80% CPU cap
├── memory.high = 1536M           # Proactive memory reclamation threshold
├── memory.max = 2048M            # Hard OOM limit
└── pids.max = 64                 # Subagent process thread limit
```

---

## Sandboxing & Capability Restrictions

All AI agent processes are restricted via OpenBSD pledge & unveil primitives:

```rust
// Limit agent syscalls to stdio, read/write workspace, and local network socket
agent_proc.pledge(&["stdio", "rpath", "wpath", "inet", "dns"])?;

// Expose workspace path exclusively
agent_proc.unveil("/userland/workspace", "rwc")?;
agent_proc.unveil("/tmp", "rwc")?;
agent_proc.unveil_finalize()?;
```

---

## IPC & Multi-Agent Coordination

AI agents communicate across process boundaries using `ZeroCopyIpcChannel` or `AndroidBinderIpc`:

```rust
use sigmaos::process::ZeroCopyIpcChannel;

let mut channel = ZeroCopyIpcChannel::new(agent_pid_a, agent_pid_b)?;
channel.send_message(b"TASK_COMPLETED: Code generation finished successfully.")?;
```

---

## Interactivity Scheduling Priority

AI agents driving real-time user interfaces (e.g. IDE inline code completion) are scheduled with high interactivity scores under the FreeBSD ULE / BORE hybrid scheduler, whereas long-running background batch agents run under `SCHED_BATCH` to avoid starving desktop rendering workloads.
