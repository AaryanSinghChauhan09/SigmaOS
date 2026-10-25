# SigmaOS Master Documentation Index

Welcome to the SigmaOS Master Documentation Index. Below is the directory of master architecture plans and development specifications for core kernel and userspace subsystems.

## Core Kernel Architecture & Subsystem Plans

- [Dispatcher Architecture Plan](DISPATCHER_PLAN.md) — Boundary definitions between scheduler, context-switch, interrupt handling, and deferred work; 8 invariants, 16-PR sequence roadmap, maintenance cadences, component rules, PR review template, and merge gates.
- [Interrupt Handler Architecture Plan](INTERRUPT_HANDLER_PLAN.md) — 5-stage decoupled interrupt pipeline across x86_64, AArch64, and RISC-V.
- [Physical Memory Manager Plan](PHYSICAL_MEMORY_MANAGER_PLAN.md) — Two-stage physical memory allocation, memblock, and buddy/bitmap allocators.
- [Process Manager Plan](PROCESS_MANAGER_PLAN.md) — Process lifecycle management, state transitions, child reaping, and service graphs.
- [Process Synchronization Plan](PROCESS_SYNCHRONIZATION_PLAN.md) — Synchronization primitives, futex, spinlocks, completions, and lockdep diagnostics.
- [Scheduler Development Plan](SCHEDULER_DEVELOPMENT_PLAN.md) & [Scheduler Status and Invariants](SCHEDULER_STATUS_AND_INVARIANTS.md) — Core CPU scheduling policies (CFS, BORE, EEVDF, Real-Time).
- [Signal Handler Plan](SIGNAL_HANDLER_PLAN.md) & [Signal Audit Profile](SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE.md) — Capability-gated async signals and POSIX signal compatibility.
- [Thread Manager Specification](thread-manager.md) — Thread lifecycle states, context, stack guards, and ownership.
- [IPC Handler Specification](ipc-handler.md) — Inter-process communication channels, endpoints, message buffers, and shared memory.

## Userspace & System Utilities Plans

- [System Utilities Development Plan](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md) — POSIX system utilities baseline, coreutils, and CLI tools.
- [SigmaOS GUI Development Plan](SIGMAOS_GUI_DEVELOPMENT_PLAN.md) — Zenith Desktop environment, shell, and window manager integration.
