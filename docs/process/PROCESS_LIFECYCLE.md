# SigmaOS Process Lifecycle Policy

## Authoritative Lifecycle State Machine

SigmaOS defines one authoritative lifecycle contract for process management:

```text
fork/clone -> exec -> run -> signal -> exit -> zombie -> wait/reap -> cleanup
```

## Lifecycle States and Requirements

| State | Required Behavior |
|---|---|
| **NEW** | PID allocated, address space and credentials initialized. |
| **READY** | Placed in scheduler run queue, waiting for CPU allocation. |
| **RUNNING** | Executes on CPU with preemption and quantum time-slice active. |
| **SLEEPING** | Blocked on a wait queue (I/O, futex, event) with a defined wakeup path. |
| **STOPPED** | Suspended by signal (`SIGSTOP`, `SIGTSTP`) or debugger ptrace. |
| **ZOMBIE** | Execution halted, exit status retained until consumed by parent/reaper. |
| **REAPED** | PID, kernel objects, fd references, and accounting completely released. |

## Legal State Transitions

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

## Invariants and Lifecycle Rules

1. **No Direct Transition from NEW to ZOMBIE**:
   A process in state `NEW` cannot move directly to `ZOMBIE` without executing proper resource cleanup.
2. **Minimal Zombie Retainage**:
   A process in state `ZOMBIE` retains only its exit code/status and minimal process table accounting data.
3. **Complete Resource Deallocation on Reap**:
   Upon transition to `REAPED`, all retained kernel structures, file descriptors, address space mappings, and PIDs are released.
4. **Deterministic Orphan Adoption**:
   Orphaned child processes whose parent exits before reaping must be adopted by the designated init process or subreaper (`PID 1`).
