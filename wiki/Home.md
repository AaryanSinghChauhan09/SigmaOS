# Home - SigmaOS Subsystem Documentation

Welcome to the SigmaOS Subsystem Documentation Home.

## Quick Links to Core Plans

- **Process Manager**: [PROCESS_MANAGER_PLAN.md](PROCESS_MANAGER_PLAN.md) — Master Architecture, 18-PR proposal roadmap, process lifecycle contracts, PID 1 child reaping, service supervision, and restart policies.
- **Physical Memory Manager**: [PHYSICAL_MEMORY_MANAGER_PLAN.md](PHYSICAL_MEMORY_MANAGER_PLAN.md) — Two-stage physical memory allocation (early memblock -> runtime allocator), 11 invariants, and 20-PR roadmap.
- **Dispatcher**: [DISPATCHER_PLAN.md](DISPATCHER_PLAN.md) — Dispatcher boundaries (context save/restore, run-state transitions, deferred work dispatch), 8 invariants, and 16-PR roadmap.
- **Interrupt Handling**: [INTERRUPT_HANDLER_PLAN.md](INTERRUPT_HANDLER_PLAN.md) — 5-stage decoupled interrupt pipeline, multi-arch vector contracts, and 17-PR roadmap.
- **Thread Manager**: [thread-manager.md](thread-manager.md) — Thread lifecycle states, CPU context contracts, wait-channel wakeups, and stack guard checks.
- **CPU Scheduler**: [SCHEDULER_DEVELOPMENT_PLAN.md](SCHEDULER_DEVELOPMENT_PLAN.md) & [SCHEDULER_STATUS_AND_INVARIANTS.md](SCHEDULER_STATUS_AND_INVARIANTS.md) — Policy invariants, CFS, EEVDF, BORE, Real-Time FIFO/RR, and 18-PR roadmap.
- **Process Synchronization**: [PROCESS_SYNCHRONIZATION_PLAN.md](PROCESS_SYNCHRONIZATION_PLAN.md) — 5-layer synchronization model, wait queues, spinlocks, semaphores, futexes, flock, and 17-PR roadmap.
- **Signal Handling**: [SIGNAL_HANDLER_PLAN.md](SIGNAL_HANDLER_PLAN.md) — POSIX.1-2024 signal delivery semantics and maintenance plan.
- **IPC Handler**: [ipc-handler.md](ipc-handler.md) — IPC boundaries, anonymous pipes, FIFOs, UNIX sockets, shared memory, and futexes.
- **System Utilities**: [SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md) — POSIX coreutils compatibility, CLI command family rules, and 18-PR roadmap.
