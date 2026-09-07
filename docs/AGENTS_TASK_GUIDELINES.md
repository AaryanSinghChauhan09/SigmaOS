# SOVEREIGN AI AGENT TASK GUIDELINES & TASK MANAGEMENT SPECIFICATION

## 1. Executive Overview & Architectural Intent

Autonomous AI engineering agents operating within **SigmaOS** require rigorous task management protocols inspired by industrial Linux kernel schedulers (EEVDF, BORE, SCHED_DEADLINE) and BSD subsystem abstractions (`kqueue`, `capsicum`, `pledge`, `unveil`). AI agent tasks represent discrete units of execution, optimization, verification, and code synthesis across the operating system kernel, userland utilities, and driver shards.

This specification establishes mandatory guidelines, task lifecycle state machines, execution scheduling rules, resource quota enforcement, and failure recovery patterns governing all AI agent task operations in SigmaOS.

---

## 2. AI Agent Task Lifecycle State Machine

Every AI agent task in SigmaOS undergoes explicit, deterministic state transitions. Unregistered or out-of-order state transitions trigger immediate task isolation and state rollback.

```
       +------------------+
       |   Task Created   |
       +--------+---------+
                |
                v
       +------------------+
  +--->|     Pending      |
  |    +--------+---------+
  |             |
  | (Retry/     v (Scheduler Dispatch)
  | Resume) +------------------+
  +---------|     Running      |---+
            +--------+---------+   |
                |        |         | (Fault / Quota Exceeded)
 (IO / Sync     |        |         v
  Wait)         v        |   +------------------+
       +------------------+  |     Evicted      |
       |     Blocked      |  +------------------+
       +------------------+        |
                |                  v
                +------------> +------------------+
                               |      Failed      |
                               +------------------+
                                       ^
                                       |
 (Execution Succeeded)                 | (Verification Failed)
       +------------------+            |
       |    Completed     |------------+
       +------------------+
```

### 2.1 State Transition Definitions

1. **Pending**: The task is registered in the AI Agent Task Dispatch Queue (`src/kernel/sched/task.rs`), with initial memory, capabilities, and dependencies resolved.
2. **Running**: The task has been allocated CPU quantum under EEVDF/BORE scheduling rules and is actively executing under `pledge`/`unveil` sandboxing constraints.
3. **Blocked**: The task is awaiting asynchronous I/O completion, IPC channel synchronization, or hardware interrupt dispatch via BSD `kqueue(2)`.
4. **Completed**: The task executed to completion, passed mandatory self-verification checks, and committed its atomic state change.
5. **Evicted**: The task exceeded cgroups v2 resource limits (CPU time, memory ceiling, file descriptor cap) or violated capability permissions, forcing immediate context termination.
6. **Failed**: The task encountered an unhandled exception or failed post-execution integrity verification, triggering atomic state rollback (`rollback()`).

---

## 3. Linux & BSD Distro-Inspired Task Scheduling & Event Dispatch

### 3.1 EEVDF / BORE Scheduler Integration
AI agent tasks are scheduled using Earliest Eligible Virtual Deadline First (EEVDF) augmented with Burst-Oriented Response Enhancer (BORE) latency weighting:
* **Virtual Deadline Calculation**: $V_i = E_i + \frac{Q_i}{w_i}$, where $E_i$ is eligibility time, $Q_i$ is task slice, and $w_i$ is weight derived from capability level.
* **Latency Sensitivity**: Interactive AI tasks (e.g., Zenith compositor frame updates) receive BORE priority boosts to guarantee sub-millisecond response latency.
* **Batch Autonomous Tasks**: Deep optimization tasks (e.g., kernel PGO or bytecode transpilation) run under low-weight BORE slices to prevent starvation of userland threads.

### 3.2 BSD `kqueue(2)` Asynchronous Event Loop
Agent tasks register file descriptor, process, signal, and timer events with the BSD `kqueue` subsystem (`src/kernel/kqueue.rs`):
```rust
// BSD kqueue task event registration pattern
let mut kq = KQueue::new();
kq.register_event(ident, EVFILT_READ, EV_ADD | EV_ENABLE, note, data)?;
```
Tasks must not block synchronously on I/O. All wait states must suspend execution via `kqueue` filters to maximize multicore CPU throughput.

---

## 4. Task Isolation, Sandboxing & Resource Quotas

### 4.1 Fine-Grained Capability Sandboxing
All agent tasks operate under OpenBSD-inspired `pledge(2)` and `unveil(2)` sandboxing (`src/security/sigma_unveil.rs`):
* **Pledge Promises**: Tasks explicitly drop unneeded system call promises (e.g., `pledge("stdio rpath wpath")`). Attempting an unpromised syscall generates a `SIGABRT` and task eviction.
* **Unveil Path Scoping**: Tasks restrict filesystem visibility to declared target paths (`unveil("/src", "r")`, `unveil("/build", "rwc")`). Path traversal outside unveiled locations returns `ENOENT` or `EACCES`.

### 4.2 Linux Cgroups v2 Resource Enforcement
Tasks are governed by strict cgroups v2 resource envelopes:
* **Memory Limits**: Max physical allocation threshold (`memory.high` / `memory.max`). Exceeding `memory.max` triggers task eviction without kernel panics.
* **CPU Quotas**: Bandwidth limits (`cpu.max = "50000 100000"` for 50% single-core allocation).
* **I/O Bandwidth**: Read/write byte rate throttling on storage blocks.
* **PID Limits**: Maximum child process spawning cap (`pids.max`).

---

## 5. Task Failure Recovery & Atomic Rollback Protocol

When an AI agent task fails or experiences eviction:
1. **Interrupt Signal Trap**: Catch `SIGSEGV`, `SIGBUS`, `SIGXCPU`, or system abort signals.
2. **State Graph Rollback**: Revert system declarative state graph (`src/system/state.rs`) to the pre-task snapshot ID in $O(1)$ time.
3. **RAM Scrubbing**: Scrub all dirty memory allocations occupied by the task using volatile 0x00 overwrites.
4. **Audit Logging**: Write structural diagnostic records to the defensive security audit log (`src/security/defensive_audit.rs`).
5. **Automated Quota Adjustment**: If task failed due to resource eviction, scale memory/time quotas by 1.5x before queueing retry.

---

## 6. Implementation Checklist for AI Agents

When implementing or modifying any task-handling subsystem in SigmaOS, AI agents must adhere to the following checklist:

- [ ] Task state transitions adhere strictly to Section 2 state machine rules.
- [ ] Task execution paths enforce zero dynamic heap allocations on hot scheduling paths.
- [ ] Sandboxing restricts system call surface via `pledge` and filesystem paths via `unveil`.
- [ ] Asynchronous event waits utilize `kqueue` without blocking hardware interrupt lines.
- [ ] Failure paths trigger state rollback and zero-memory scrubbing before task destruction.
- [ ] Unit tests cover normal completion, quota eviction, and fault recovery paths.
