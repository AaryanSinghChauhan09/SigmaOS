# SigmaOS Documentation Index

Welcome to the SigmaOS Architecture & Development Documentation.

## Core System Architecture & Subsystem Plans

- [IPC Handler Master Architecture & Plan](ipc-handler.md) — Formally specified, testable IPC handler subsystem covering pipes, FIFOs, UNIX sockets, shared memory, message queues, event wakeups, signals control plane, descriptor passing, and access control policies.
- [Dispatcher Master Architecture & Plan](DISPATCHER_PLAN.md) — Context save/restore, run-state transitions, interrupt/syscall return paths, and deferred kernel work dispatch.
- [Process Manager Architecture Plan](PROCESS_MANAGER_PLAN.md) — Process/thread hierarchy, service supervision, state transitions, child reaping, and adoption.
- [Interrupt Handling Master Architecture & Plan](INTERRUPT_HANDLER_PLAN.md) — 5-stage decoupled interrupt pipeline across x86_64, AArch64, and RISC-V.
- [Physical Memory Manager Architecture & Plan](PHYSICAL_MEMORY_MANAGER_PLAN.md) — Two-stage physical memory management (early boot memblock to runtime allocator).
- [Process Synchronization Plan](PROCESS_SYNCHRONIZATION_PLAN.md) — 5 architectural synchronization layers, futexes, semaphores, spinlocks, and lockdep.
- [Signal Development & Maintenance Plan](SIGNAL_HANDLER_PLAN.md) — Asynchronous signal delivery, sigaction, signalfd, and POSIX signal parity.
- [System Utilities Development Plan](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md) — POSIX system utilities, argument parsing, and CLI interfaces.
- [Scheduler Development Plan](SCHEDULER_DEVELOPMENT_PLAN.md) — CPU scheduler subsystem across CFS, BORE, EEVDF, Real-Time FIFO/RR, and SchedExt.
