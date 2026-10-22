# SigmaOS Documentation & Architecture Plans Index

## Kernel Architecture Plans

- **[Process Manager Plan](PROCESS_MANAGER_PLAN.md):** Process lifecycle, state transitions, process creation/exit, and parent-child hierarchy.
- **[Thread Manager Plan](thread-manager.md):** Thread lifecycle (New, Ready, Running, Blocked, Suspended, Zombie, Terminated), join/detach, wait channel wakeups, stack guard checks, CpuContext, and ownership contract.
- **[Dispatcher Plan](DISPATCHER_PLAN.md):** Context switching, execution state transitions, preemption hooks, syscall/interrupt return paths, and deferred kernel work queues.
- **[Scheduler Plan](SCHEDULER_DEVELOPMENT_PLAN.md):** CPU scheduler policies (CFS, BORE, EEVDF, Real-Time FIFO/RR, SchedExt) and policy selection algorithms.
- **[Interrupt Handler Plan](INTERRUPT_HANDLER_PLAN.md):** Decoupled 5-stage interrupt pipeline, IDT/APIC/GIC/PLIC flow handlers, and IRQ domain management.
- **[Signal Handler Plan](SIGNAL_HANDLER_PLAN.md):** POSIX & Sigma signal delivery, sigaction handling, sigreturn frames, and signal disposition.
- **[Process Synchronization Plan](PROCESS_SYNCHRONIZATION_PLAN.md):** Mutexes, semaphores, spinlocks, futexes, wait queues, and lock ordering invariants.
- **[Physical Memory Manager Plan](PHYSICAL_MEMORY_MANAGER_PLAN.md):** Bitmap physical memory manager, memblock early boot allocator, and physical page frame management.
- **[IPC Handler Plan](ipc-handler.md):** IPC channels, endpoints, shared memory regions, wait queues, and zero-copy transfer mechanisms.

## Subsystem Architecture Plans

- **[System Utilities Development Plan](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md):** POSIX coreutils, CLI utilities, system administration tools, and argument parsing engines.
- **[SigmaOS GUI Development Plan](SIGMAOS_GUI_DEVELOPMENT_PLAN.md):** Zenith desktop environment, compositor integration, display management, and windowing system.
- **[SigmaOS CLI Development Plan](SIGMAOS_CLI_DEVELOPMENT_PLAN.md):** Command-line router, interactive REPL, shell extensions, and terminal capabilities.
- **[Command Interpreter Plan](shell/COMMAND_INTERPRETER_PLAN.md):** Sovereign command interpreter, POSIX shell compatibility, subshells, and job control.
- **[Device Driver Development Plan](SIGMAOS_DEVICE_DRIVER_DEVELOPMENT_PLAN.md):** Device drivers, PCI bus scanning, NVMe/AHCI storage controllers, and driver framework.
- **[Network Development Plan](net/NETWORK_DEVELOPMENT_AND_MAINTENANCE_PLAN.md):** Dual IPv4/IPv6 network stack, socket interface, network interfaces, and TCP/UDP transport layers.
- **[System Apps Development Plan](SYSTEM_APPS_DEVELOPMENT_PLAN.md):** System applications, software manager, system monitor, and administrative GUIs.
