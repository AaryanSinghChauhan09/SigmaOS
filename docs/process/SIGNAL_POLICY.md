# SigmaOS Signal Delivery Policy

## Overview

Signals in SigmaOS provide asynchronous and synchronous process notifications, handling interruptibility, terminal actions, signal masks, and signal frame delivery.

## Delivery Contracts and Set Management

1. **Signal Sets:**
   - **Pending:** Signals delivered to process or thread waiting for execution.
   - **Blocked (Masked):** Signals deferred from delivery until unmasked via `sigprocmask`.
   - **Ignored:** Signals dropped immediately upon delivery without queuing.
2. **Immutability of Terminal Signals:**
   - `SIGKILL` and `SIGSTOP` cannot be caught, blocked, or ignored.
3. **Signal Mask Inheritance:**
   - Signal masks are inherited across `fork`.
   - Executed binaries (`exec`) inherit the signal mask, but caught signal handlers are reset to default (`SIG_DFL`).
4. **Signal-Safe Wakeup:**
   - Delivering an unmasked signal to a process in `SLEEPING` state interrupts sleep (returning `EINTR` where applicable) and transitions process to `READY`.
5. **Exit/Termination Race Prevention:**
   - Signals arriving during process exit or zombie transition are handled safely without lock inversion or double-free errors.
