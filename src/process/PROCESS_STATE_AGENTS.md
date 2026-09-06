# AI Agent Development Instructions for Process State & Blocked Queue Subsystem (`src/process/blocked_state.rs`)

This module manages blocked process wait queues, I/O wait states, timer sleep queues, mutex wait lists, and thread context wake-up signaling for SigmaOS.

## Subsystem Architecture & Directives

1. **Blocked State Transitions & Wait Queues (`blocked_state.rs`)**
   - Valid wait reasons: `IoWait`, `TimerSleep`, `MutexWait`, `SignalWait`, `EventWait`, `FutexWait`.
   - Thread state changes from `Running` -> `Blocked` must append the process PCB handle to the corresponding wait queue atomically.

2. **Futex & Futex Wait/Wake Semantics (`LinuxFutexEngine` integration)**
   - Waking threads via `sys_futex(FUTEX_WAKE)` or signal interrupts must atomically update process state from `Blocked` -> `Ready` and requeue the task onto the scheduler's run queue.
   - Prevent lost wakeup race conditions by checking futex val atomically under lock before suspending thread execution.

3. **Timer Sleep & Timeout Precision**
   - Timer wait queues must be ordered by target expiration timestamp (`u64` nanoseconds).
   - Expired timer tasks must be unblocked during tick interrupts without allocating dynamic heap memory inside IRQ handlers.

4. **Lock Safety & Deadlock Prevention**
   - Always release `BlockedStateQueue` locks prior to yielding CPU control via `yield_now()` or `context_switch()`.
