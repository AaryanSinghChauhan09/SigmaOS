# SigmaOS Wiki Documentation Index

This index serves as the master directory for all architecture, subsystem plans, and development specifications within the SigmaOS Wiki.

---

## Kernel Architecture & Subsystem Plans

1. [Physical Memory Manager Plan](PHYSICAL_MEMORY_MANAGER_PLAN.md) - Physical memory management, two-stage allocation, and 20-PR roadmap.
2. [Dispatcher Plan](DISPATCHER_PLAN.md) - Kernel dispatcher architecture, boundary definitions, run-state transitions, and 16-PR sequence.
3. [Process Manager Plan](PROCESS_MANAGER_PLAN.md) - Process lifecycle, state machine transitions, service supervision, and 18-PR sequence.
4. [Interrupt Handling Plan](INTERRUPT_HANDLER_PLAN.md) - Decoupled 5-stage interrupt pipeline, multi-arch support matrix, context rules, and 17-PR sequence roadmap.
5. [Signal Handling Plan](SIGNAL_HANDLER_PLAN.md) - POSIX signal dispatch, signal frame handling, and development roadmap.
6. [CPU Scheduler Plan](SCHEDULER_DEVELOPMENT_PLAN.md) - Multitasking scheduler policy, EEVDF/BORE/SchedExt governors, and invariants.
7. [Process Synchronization Plan](PROCESS_SYNCHRONIZATION_PLAN.md) - Mutexes, futexes, spinlocks, semaphores, and synchronization primitives.
8. [System Utilities Development Plan](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md) - POSIX userland utilities, CLI engines, and 18-PR sequence.

---

## Inter-Process Communication (IPC) & Userspace

1. [IPC Handler Specification](ipc-handler.md) - Canonical IPC primitives, channels, shared memory, and development phases.
2. [Thread Manager Specification](thread-manager.md) - Kernel and user thread execution contracts and lifecycle management.
