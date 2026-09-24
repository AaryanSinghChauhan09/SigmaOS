# SigmaOS Process Management Architecture & Development Roadmap

## Overview

SigmaOS process management (`src/kernel/process.rs`, `src/kernel/scheduler.rs`, `src/process/`, `src/process/sovereign_process_engine.rs`) provides a hybrid BORE+EEVDF scheduler, cgroups v2 resource controllers, capability-bounded sandboxing (`pledge`/`unveil`), zero-copy IPC channels, randomized PID allocation, and dedicated process policies for autonomous AI agent workloads.

---

## 🔍 Process Management Gap Analysis & Strategic Roadmap

### Current Capabilities & Development Gaps
SigmaOS process management incorporates seL4 capability isolation, CachyOS BORE+EEVDF scheduling, and OpenBSD `pledge`/`unveil` sandboxing. However, achieving process management parity with production Linux (Linux kernel process trees, cgroups v2, Seccomp-BPF) and BSD (FreeBSD rctl/procstat, OpenBSD pledge/unveil) requires addressing key gaps:

1. **PID Allocation Security**: Standard sequential PID allocation is vulnerable to PID recycling race conditions. SigmaOS implements `SovereignPidAllocator` with pseudo-random PID assignment and delayed PID reuse queues.
2. **Process Hierarchy & Group Governance**: Maintaining explicit parent/child process trees, process group IDs (`pgid`), session IDs (`sid`), and POSIX job control terminal bindings.
3. **POSIX Real-Time Signals & Graceful Escalation**: Real-time signal queues (`SIGRTMIN`..`SIGRTMAX`), `sigqueue` payloads, and `SIGTERM`-to-`SIGKILL` timeout escalation watchers (`SigtermGracefulWatcher`).
4. **cgroup v2 Resource Controllers**: Hierarchical CPU quota, memory RSS limit (`memory.high`/`memory.max`), process count limit (`pids.max`), and I/O weight controllers.
5. **Seccomp-BPF & Capability Sandboxing**: Combining OpenBSD `pledge`/`unveil` bitmask checks with Linux Seccomp-BPF system call filter policies.

---

### 📊 Process Management Gap Dashboard

| Feature Area | Current State (SigmaOS) | Target State (Linux & BSD Standards) |
|---|---|---|
| **PID Allocator** | `SovereignPidAllocator` randomized allocation | Randomized PID allocation with delayed reuse queue |
| **Process Tree** | Basic parent/child tracking | Full `pgid`/`sid` process group governance & job control |
| **Signals Engine** | Standard POSIX signals | Real-time `sigqueue` + `SIGTERM`-to-`SIGKILL` graceful escalation watcher |
| **Resource Control** | cgroups v2 integration | Hierarchical CPU/memory/pids/io cgroups v2 slice controllers |
| **Sandboxing** | `pledge` & `unveil` API | `pledge`/`unveil` + Seccomp-BPF syscall policy enforcement |
| **IPC Channels** | Zero-copy IPC & Binder | Zero-copy IPC + shared memory ring buffers + capability pass-through |

---

### 🚀 3-Phase Process Management Development Roadmap

#### Phase 1: Process Hierarchy & Signal Escalation (0–6 Months)
- **Randomized PID Allocation**: Implement delayed PID reuse queues in `SovereignPidAllocator` to prevent PID recycling vulnerabilities.
- **Process Group & Session Governance**: Maintain parent/child process trees, process group IDs (`pgid`), session IDs (`sid`), and terminal job control bindings.
- **Graceful Signal Escalation**: Implement `SigtermGracefulWatcher` to handle `SIGTERM` graceful shutdown timeouts with automatic `SIGKILL` escalation.

#### Phase 2: cgroup v2 Controllers & Capability Isolation (6–12 Months)
- **cgroup v2 Controller Slices**: Enforce hierarchical CPU quotas (`cpu.max`), proactive memory limits (`memory.high`/`memory.max`), and thread caps (`pids.max`).
- **Seccomp-BPF Syscall Filtering**: Implement Seccomp-BPF filter policies (`SeccompFilterPolicy`) to restrict accessible system call numbers per process.
- **Bitmask Capability Enforcement**: Enforce OpenBSD `pledge` and `unveil` bitmask checks inside the system call dispatch path.

#### Phase 3: Real-Time Signal Queues & Subagent Orchestration (12–18 Months)
- **POSIX Real-Time Signals**: Support real-time signals (`SIGRTMIN`..`SIGRTMAX`), `sigqueue` data payloads, and `sigwaitinfo` synchronous signal consumption.
- **Subagent Process Orchestration**: Integrated process lifecycle manager for autonomous AI agent tasks with resource cap monitoring.
- **Zero-Copy Binder IPC**: Zero-copy IPC message queues and Android-style Binder handle passing for inter-process communication.

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
