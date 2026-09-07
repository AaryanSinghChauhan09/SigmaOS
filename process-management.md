# SigmaOS Process Management

## Overview

SigmaOS process management (`src/kernel/process.rs`, `src/kernel/scheduler.rs`) handles:
- Process creation, scheduling, and termination
- Thread management (kernel and user threads)
- Signals and inter-process communication
- Resource limits via cgroups v2
- Process sandboxing via capabilities

## Process Lifecycle

```
fork()/spawn() → Ready → Running → Blocked → Ready
                                      ↓
                                  Terminated → Zombie → Reaped
```

## Scheduler: BORE+EEVDF

The scheduler combines:
- **EEVDF** (Earliest Eligible Virtual Deadline First): fair CPU distribution
- **BORE** (Burst-Oriented Response Enhancer): interactive responsiveness

### Priority Classes

| Class | Nice | Description |
|-------|------|-------------|
| SCHED_FIFO | -20 to -1 | Real-time FIFO (no preemption) |
| SCHED_RR | -20 to -1 | Real-time round-robin |
| SCHED_NORMAL | 0 | Default timesharing |
| SCHED_BATCH | 1 to 19 | CPU-bound batch jobs |
| SCHED_IDLE | 20 | Only runs when nothing else will |

## Cgroups v2 Integration

Process resource limits are enforced via cgroups v2 hierarchy:

```
/sys/fs/cgroup/
├── user.slice/
│   └── user-1000.slice/
│       └── sigma-session.scope/
└── system.slice/
    └── sigma-daemon.service/
```

### Resource Controllers

| Controller | Resource | Example Limit |
|------------|----------|---------------|
| cpu | CPU time | `cpu.max = 50000 100000` (50%) |
| memory | RAM + swap | `memory.max = 2G` |
| io | Block I/O | `io.max = 8:0 rbps=10485760` |
| pids | Process count | `pids.max = 256` |

## Process Sandboxing

SigmaOS uses capability-based sandboxing similar to OpenBSD's pledge/unveil:

```rust
// Restrict process to only network + stdio operations
process.pledge(&["stdio", "inet", "dns"])?;

// Reveal only specific filesystem paths
process.unveil("/etc/resolv.conf", "r")?;
process.unveil("/tmp", "rwc")?;
process.unveil_finalize()?;
```

## Signals

Standard POSIX signals are supported:

| Signal | Number | Default Action | Description |
|--------|--------|----------------|-------------|
| SIGTERM | 15 | Terminate | Graceful shutdown request |
| SIGKILL | 9 | Terminate | Immediate kill (cannot be caught) |
| SIGSTOP | 19 | Stop | Pause execution |
| SIGCONT | 18 | Continue | Resume execution |
| SIGSEGV | 11 | Core dump | Invalid memory access |
| SIGCHLD | 17 | Ignore | Child process state changed |

## Process Creation

```rust
// Fork a new process
let pid = sigma::process::fork()?;
if pid == 0 {
    // Child process
    sigma::process::exec("/usr/bin/myapp", &["arg1"], &env)?;
} else {
    // Parent: wait for child
    let status = sigma::process::waitpid(pid)?;
}

// Spawn with options (preferred API)
let child = ProcessBuilder::new("/usr/bin/myapp")
    .args(&["--verbose"])
    .env("PATH", "/usr/bin:/bin")
    .uid(1000).gid(1000)
    .capabilities(CapSet::minimal())
    .sandbox(SandboxPolicy::strict())
    .spawn()?;
```
