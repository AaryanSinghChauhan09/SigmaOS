# SigmaOS Signal Delivery Policy

## Signal Management Sets

Every process maintains bitmasks for:
- **Pending Signals**: Signals sent to the process but not yet delivered.
- **Blocked Signals (Signal Mask)**: Signals currently masked from delivery.
- **Ignored Signals**: Signals configured with `SIG_IGN` disposition.

## Delivery Ordering & Determinism

1. Standard signals (`SIGINT`, `SIGTERM`, `SIGKILL`, etc.) are delivered in deterministic numeric order or priority order.
2. Unblocked pending signals are checked and delivered:
   - On return from syscall to userland.
   - On return from kernel interrupt handlers.
   - During block/sleep state transitions.

## Immutable Signals

- **`SIGKILL` (9)** and **`SIGSTOP` (19)** cannot be caught, ignored, or blocked.
- Delivery of `SIGKILL` immediately terminates the process and transitions it to `ZOMBIE`.
- Delivery of `SIGSTOP` immediately halts execution and transitions it to `STOPPED`.

## Signal-Safe Wakeup & Race Handling

- A signal delivered while a process is in `SLEEPING` state interrupts sleep (returning `EINTR` if interruptible) and wakes the process into `READY`.
- Simultaneous signal delivery during `exit()` processing evaluates terminal status deterministically without crashing kernel state structures.
