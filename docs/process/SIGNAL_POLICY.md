# SigmaOS Signal Delivery & Handling Policy

## Signal Sets & Ordering

Each process maintains:
- **Pending Set:** Signals delivered but not yet processed or handled.
- **Blocked Mask:** Signals currently deferred from delivery.
- **Ignored Mask:** Signals configured to be ignored (`SIG_IGN`).

Signals are evaluated in standard priority order (standard unblockable signals first, followed by real-time signals in queue order).

## Key Signal Invariants

1. **Unblockable Signals:** `SIGKILL` and `SIGSTOP` cannot be caught, blocked, or ignored.
2. **Signal-Safe Wakeup:** Delivering a signal to a process in `SLEEPING` state safely interrupts sleep/wait queues and returns `EINTR` or executes signal handler without lost wakeups.
3. **Inheritance Across Fork/Exec:**
   - `fork`: Child inherits signal mask, pending signals (cleared for child), and signal dispositions.
   - `exec`: Custom signal handlers are reset to `SIG_DFL`; ignored signals remain ignored (`SIG_IGN`); signal mask is preserved.
4. **Exit Races:** Signals delivered during process termination/exit are discarded safely after the process transitions past active execution.
