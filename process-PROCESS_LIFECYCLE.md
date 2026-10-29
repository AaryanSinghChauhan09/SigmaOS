# SigmaOS Process Lifecycle Policy

## State Machine Definition

SigmaOS enforces a deterministic 7-state process lifecycle:

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

## State Specification

| State | Description & Requirements |
|---|---|
| `NEW` | PID allocated, address space, file descriptor table, and credentials initialized. Cannot move directly to `ZOMBIE` without proper cleanup. |
| `READY` | Placed in the scheduler run queue awaiting CPU execution time slice. |
| `RUNNING` | Executing instructions on CPU core. Preemption and time slice expiration monitored. |
| `SLEEPING` | Blocked waiting on I/O, timer, lock, or wait queue with an explicit wakeup path. |
| `STOPPED` | Suspended via signal (`SIGSTOP`, `SIGTSTP`) or debugger break. |
| `ZOMBIE` | Process terminated; retains exit code, minimal accounting, and parent linkage until reaped. |
| `REAPED` | Terminal state; PID, kernel objects, memory, and descriptor references completely released. |

## Legal State Transitions

1. **`NEW -> READY`**: Process creation completed successfully.
2. **`READY -> RUNNING`**: Scheduler context-switches process onto CPU core.
3. **`RUNNING -> READY`**: Time slice expired or preempted by higher priority task.
4. **`RUNNING -> SLEEPING`**: Blocked on I/O or wait queue.
5. **`SLEEPING -> READY`**: Event/wait-queue wakeup triggered.
6. **`RUNNING -> STOPPED`** / **`SLEEPING -> STOPPED`**: Suspended by `SIGSTOP`.
7. **`STOPPED -> READY`**: Resumed by `SIGCONT`.
8. **`* -> ZOMBIE`**: Exit syscall or fatal signal termination.
9. **`ZOMBIE -> REAPED`**: Parent or subreaper consumes exit status via `wait`/`waitpid`.

## System Invariants

- **No Orphan Zombies**: Orphaned processes are automatically adopted by PID 1 (`init`) or a designated subreaper upon parent death.
- **Single Reap Guarantee**: Concurrent or duplicate `wait`/`reap` calls return a typed error (`ProcessStateError::InvalidStateTransition`) rather than double-reaping.
- **Memory & Resource Cleanup**: Transition to `REAPED` guarantees complete removal of process table entries and deallocation of PID.
