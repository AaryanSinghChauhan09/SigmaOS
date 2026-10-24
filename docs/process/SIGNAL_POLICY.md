# SigmaOS Signal Delivery Policy

## Signal Subsystem Specification

Signal management in SigmaOS guarantees deterministic signal delivery, safety, and POSIX conformance:

1. **Pending and Blocked Signal Sets**:
   - Each process tracks pending signal bitmasks and blocked signal bitmasks (`sigprocmask`).
   - Signal delivery evaluates `pending & ~blocked`.
2. **Unblockable Signals**:
   - `SIGKILL` and `SIGSTOP` cannot be caught, blocked, or ignored.
   - Attempting to block `SIGKILL` or `SIGSTOP` via mask updates is ignored or rejected.
3. **Signal Delivery Ordering**:
   - Standard signals (1-31) delivered in deterministic numerical priority order.
   - Real-time signals delivered in FIFO queued order.
4. **Signal Inheritance**:
   - `fork`: Child inherits parent's signal mask and pending sets (cleared pending).
   - `exec`: Retains blocked signal mask; resets caught signal handlers to `SIG_DFL`.
5. **Signal-Safe Wakeup**:
   - Delivering a non-ignored signal to a process in `SLEEPING` state transitions it to `READY` to process the signal.
6. **Zombie Cleanup on Terminal Signal**:
   - Unhandled terminal signals (`SIGKILL`, `SIGSEGV`, `SIGTERM`) transition process directly to `ZOMBIE` and trigger `SIGCHLD` to parent.
