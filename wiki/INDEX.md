# SigmaOS Technical Documentation Index

## Kernel Architecture & Subsystem Development Plans

- [Physical Memory Manager Plan](PHYSICAL_MEMORY_MANAGER_PLAN.md) - Master Architecture, Support Matrix, and 20-PR Plan for physical memory management.
- [Dispatcher Development Plan](DISPATCHER_PLAN.md) - Dispatcher Master Architecture, Context Switching, Run-State Transitions, and 16-PR Plan.
- [Process Manager Development Plan](PROCESS_MANAGER_PLAN.md) - Process Lifecycle, Task Group Supervision, State Machine, and 18-PR Sequence Roadmap.
- [IPC Handler Development Plan](ipc-handler.md) - Inter-Process Communication Specification, Microkernel IPC Endpoints, Channels, and 10-PR Roadmap.
- [Interrupt Handler Plan](INTERRUPT_HANDLER_PLAN.md) - Master Architecture, Support Matrix, Decoupled 5-Stage Pipeline, and 17-PR Sequence Roadmap for x86_64 IDT/APIC, AArch64 GICv3, and RISC-V PLIC.
- [Thread Manager Plan](thread-manager.md) - Thread Lifecycle, Stack Guards, Context Switches, and Execution Models.
- [Signal Handler Plan](SIGNAL_HANDLER_PLAN.md) - Signal Management Architecture, POSIX Compatibility Profile, and 18-PR Sequence Roadmap.
- [CPU Scheduler Development Plan](SCHEDULER_DEVELOPMENT_PLAN.md) - CFS, BORE, EEVDF, Real-Time FIFO/RR, and SchedExt BPF Governor Roadmap.
- [Process Synchronization Plan](PROCESS_SYNCHRONIZATION_PLAN.md) - Synchronization Primitives, Futex, Wait Queues, Spinlocks, and Lockdep Architecture.

## Userspace & Subsystem Development Plans

- [System Utilities Development Plan](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md) - POSIX Coreutils Baseline, CLI Parsing Engines, and 18-PR Sequence Roadmap.
