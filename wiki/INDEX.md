# SigmaOS Wiki Architecture Index

Welcome to the SigmaOS Subsystem Documentation & Master Plan Wiki Index.

## Subsystem Architecture Plans

- [Dispatcher Plan](DISPATCHER_PLAN.md) — Master Architecture, 8 Core Invariants, 16-PR Roadmap & Support Matrix for Scheduler-Dispatcher-IRQ boundary separation, context switching, interrupt flow, and deferred work execution.
- [Physical Memory Manager Plan](PHYSICAL_MEMORY_MANAGER_PLAN.md) — Two-stage physical memory allocation, 11 invariants, and 20-PR sequence roadmap.
- [Interrupt Handler Plan](INTERRUPT_HANDLER_PLAN.md) — 5-stage decoupled interrupt pipeline, multi-arch vector contracts, and 17-PR sequence roadmap.
- [Process Manager Plan](PROCESS_MANAGER_PLAN.md) — State machine transitions, subreaper adoption, process lifecycle, and 18-PR sequence roadmap.
- [Thread Manager Plan](thread-manager.md) — Thread lifecycle states, ownership contracts, wait channel wakeups, and CpuContext.
- [Signal Handler Plan](SIGNAL_HANDLER_PLAN.md) — POSIX.1-2024 signal semantics, ABI profile, signal-frame layout, and 18-PR sequence roadmap.
- [IPC Handler Plan](ipc-handler.md) — Pipes, FIFOs, UNIX sockets, shared memory, message queues, and futexes.
- [Scheduler Status & Invariants](SCHEDULER_STATUS_AND_INVARIANTS.md) — CFS, BORE, EEVDF, and RT policy invariants and 18-PR sequence roadmap.
- [Process Synchronization Plan](PROCESS_SYNCHRONIZATION_PLAN.md) — Wait queues, spinlocks, semaphores, futexes, flock, and lockdep.
- [System Utilities Development Plan](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md) — Core POSIX utilities, command registry framework, and 18-PR sequence roadmap.
