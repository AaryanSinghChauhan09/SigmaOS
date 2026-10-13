# SigmaOS Process Lifecycle Contract

## Overview

SigmaOS treats process management as an authoritative lifecycle subsystem. Every process moves through a deterministic sequence of state transitions from creation to final resource reclamation.

## Authoritative Lifecycle State Machine

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

```text
fork/clone -> exec -> run -> signal -> exit -> zombie -> wait/reap -> cleanup
```

## State Behaviors and Requirements

| State | Required Behavior |
|---|---|
| **NEW** / Created | PID allocated, address space initialized, credentials assigned, parent relationship registered. Cannot transition directly to `ZOMBIE` without cleanup. |
| **READY** / Runnable | Placed in the scheduler run queue awaiting CPU execution time. |
| **RUNNING** | Active CPU execution with defined preemption and quantum slice handling. |
| **SLEEPING** | Blocked on a kernel wait queue (I/O, futex, event) with a deterministic wakeup path. Lost wakeups are prevented via queue locking invariants. |
| **STOPPED** | Execution suspended by job control signal (`SIGSTOP`, `SIGTSTP`, `SIGTTIN`, `SIGTTOU`) or debugger tracing. |
| **ZOMBIE** | Terminated process retaining only exit status, signal metadata, and minimal accounting. All memory maps, file descriptors, and capabilities released. |
| **REAPED** | PID returned to allocator (after delay/cooldown), process table entry purged, kernel structures freed. |

## Rules and Invariants

1. **No Orphan Zombies:** When a parent process exits before its children, all child processes are atomically reparented to `init` (PID 1) or an explicit subreaper (`PR_SET_SUBREAPER`).
2. **Deterministic Reaping:** Only the designated parent or adopted reaper may consume exit status via `wait`/`waitpid`.
3. **Single Reap Guarantee:** A zombie process entry can be reaped exactly once. Secondary `wait` calls on the same PID return error.
4. **Clean Reclamation:** Transition to `REAPED` frees all process structures, active PID reservations, and resource accounting trackers.
