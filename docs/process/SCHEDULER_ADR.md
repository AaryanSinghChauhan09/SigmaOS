# Architectural Decision Record: Scheduler Subsystem Lifecycle State Machine

## Status
Accepted

## Context
SigmaOS requires a deterministic, thread-safe, and observable process scheduling model. Previously, scheduling was treated as an ad-hoc task without explicit state guarantees, leading to potential zombie leaks, unhandled orphan processes, and race conditions during context switches.

## Decision
Adopt a single authoritative 7-state POSIX lifecycle state machine for all scheduler entities:

```text
NEW -> READY -> RUNNING -> SLEEPING -> READY
READY -> RUNNING -> ZOMBIE
RUNNING -> STOPPED -> READY
ANY -> ZOMBIE -> REAPED
```

### Key Architectural Invariants
1. **Never Scheduled After Exit**: Processes transitioning to `ZOMBIE` are immediately detached from run queues and active CPU registers.
2. **Mandatory Reaping**: Every zombie must be reaped by its parent or automatically adopted and reaped by PID 1 (init).
3. **Lost-Wakeup Protection**: Sleeping transitions verify `wake_event_pending` and `pending_signals` atomically before suspending execution.
4. **Bounded Capacity**: All run queues and process tables enforce explicit upper limits to prevent kernel memory exhaustion.

## Consequences
- Guarantees zero zombie accumulation under process churn.
- Simplifies kernel debugging with explicit state machine invariant assertions.
- Connects task execution directly to validated boot-to-interactive-shell runtime paths.
