# SigmaOS Process Lifecycle Policy

## Authoritative Lifecycle State Machine

SigmaOS defines one authoritative process lifecycle state machine:

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

## State Definitions & Required Behaviors

| State | Required Behavior |
|---|---|
| **NEW** | PID allocated, address space and credentials initialized. Cannot move directly to ZOMBIE without full resource cleanup. |
| **READY** | Enqueued in scheduler run queue awaiting CPU slice allocation. |
| **RUNNING** | Actively executing on CPU with defined preemption and quantum slice boundaries. |
| **SLEEPING** | Blocked on a kernel wait queue with guaranteed race-free wakeup path upon event arrival or signal delivery. |
| **STOPPED** | Execution suspended by signal (`SIGSTOP`, `SIGTSTP`) or debugger attach. |
| **ZOMBIE** | Execution terminated; process retains only exit status code and minimal accounting data until consumed by parent. |
| **REAPED** | Terminal state; PID, kernel task entries, file descriptor references, and memory accounting are fully released. |

## Invariants & Rules

1. **Direct Transition Safety:** A process in `NEW` state must never transition directly to `ZOMBIE` without completing process structure initialization and resource cleanup.
2. **Zombie Accounting:** A zombie retains only its exit code, termination signal (if any), PID, and minimal accounting info. All memory maps, handle tables, and open files must be freed upon entering `ZOMBIE`.
3. **Deterministic Orphan Adoption:** No process may remain a zombie indefinitely. When a parent process terminates, any child processes (including zombies) are adopted by the system init/reaper daemon (PID 1) or designated subreaper.
4. **Single-Reap Assurance:** Reaping transitions a zombie to `REAPED` atomically, releasing the PID and kernel process table slot. Duplicate reaping requests must fail safely with an explicit error.
