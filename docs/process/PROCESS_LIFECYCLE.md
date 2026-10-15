# SigmaOS Process Lifecycle Policy (`docs/process/PROCESS_LIFECYCLE.md`)

## 1. Overview & Objective

SigmaOS treats process management as a safe, observable, and testable lifecycle subsystem. This specification defines the authoritative process lifecycle state machine and transition invariants.

## 2. Authoritative Lifecycle State Machine

Every process in SigmaOS moves through an authoritative set of states:

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

```text
fork/clone -> exec -> run -> signal -> exit -> zombie -> wait/reap -> cleanup
```

## 3. State Invariants & Behavior Contracts

| State | Required Behavior | Allowed Transitions |
|---|---|---|
| **NEW** | PID allocated, kernel structures, memory map, credentials, and fd table initialized. Cannot transition directly to ZOMBIE without cleanup. | `READY` |
| **READY** | Enqueued in scheduler run queue; eligible for CPU dispatch. | `RUNNING`, `ZOMBIE` |
| **RUNNING** | Active execution on CPU core; preemption and slice timer active. | `READY`, `SLEEPING`, `STOPPED`, `ZOMBIE` |
| **SLEEPING** | Blocked on a wait queue (I/O, timer, lock) with explicit wakeup path. | `READY`, `STOPPED`, `ZOMBIE` |
| **STOPPED** | Execution suspended via signal (`SIGSTOP`) or debugger; retains memory state. | `READY`, `ZOMBIE` |
| **ZOMBIE** | Terminated process; releases address space and file descriptors, retains exit status and minimal accounting data until reaped. | `REAPED` |
| **REAPED** | Terminal state. PID, process table entry, kernel objects, and accounting released. | None (Terminal) |

## 4. Zombie & Orphan Reaper Invariants

1. **No Permanent Zombies**: No process may remain in the `ZOMBIE` state indefinitely after its parent exits.
2. **Orphan Adoption Policy**: When a parent process exits before its children, an init/subreaper policy (`PID 1` or designated subreaper) immediately adopts all orphan processes.
3. **Deterministic Reaping**: The subreaper periodically or asynchronously reaps zombie orphans via `waitpid(-1, ...)` loops.

## 5. Verification & Testing

Lifecycle states and state transitions are tested and enforced in `tests/process/lifecycle/lifecycle_test.rs` and `src/process/state.rs`.
