# SigmaOS Signal Delivery Policy (`docs/process/SIGNAL_POLICY.md`)

## 1. Scope & Objective

This document specifies POSIX signal management in SigmaOS, including signal sets (pending, blocked, ignored), immutability rules, signal delivery ordering, and wait-queue wakeup safety.

## 2. Signal Management Model

Each process contains:
- **Pending Signal Mask**: Bitmask of delivered signals awaiting handling.
- **Blocked Signal Mask**: Bitmask of signals currently blocked (`sigprocmask`).
- **Signal Dispositions**: Actions associated with signals (`SIG_DFL`, `SIG_IGN`, or custom handler function pointer).

## 3. Mandatory Delivery Invariants

1. **Uncatchable Signals**: `SIGKILL` (9) and `SIGSTOP` (19) cannot be caught, ignored, or blocked. Delivery is immediate and mandatory.
2. **Signal Mask Inheritance**: `fork` duplicates the signal mask and dispositions. `exec` preserves blocked masks but resets caught signal dispositions to `SIG_DFL`.
3. **Wait Queue Safe Wakeup**: A process blocked in `SLEEPING` state receiving an unblocked signal is immediately woken up to `READY` state to deliver the signal (preventing lost wakeups).
4. **Terminal Signal Exit**: Terminal signals transition the process to `ZOMBIE` state and set its exit code to signal termination status (e.g., `128 + signal`).

## 4. Verification

Signal blocking, uncatchable guards, and wait queue wakeup safety are verified in `tests/signal/signal_test.rs`.
