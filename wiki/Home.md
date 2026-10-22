# Welcome to the SigmaOS Wiki

Welcome to the official technical documentation and architecture wiki for **SigmaOS**.

## Architectural Plans & Subsystem Specifications

### Kernel Core Architecture
- **[Process Manager Plan](PROCESS_MANAGER_PLAN.md):** Process lifecycle, state transitions, hierarchy, and child reaping.
- **[Thread Manager Plan](thread-manager.md):** Canonical thread lifecycle, context state, and wait channel handling.
- **[Dispatcher Plan](DISPATCHER_PLAN.md):** Task context switching, preemption hooks, privilege return paths, and deferred work queues.
- **[Scheduler Plan](SCHEDULER_DEVELOPMENT_PLAN.md):** Multi-policy CPU scheduler (CFS, BORE, EEVDF, Real-Time FIFO/RR, SchedExt).
- **[Interrupt Handler Plan](INTERRUPT_HANDLER_PLAN.md):** 5-stage interrupt handling pipeline, controller flow, and APIC/GIC/PLIC abstractions.
- **[Signal Handler Plan](SIGNAL_HANDLER_PLAN.md):** Signal delivery engine, sigaction ABI compliance, and frame verification.
- **[Process Synchronization Plan](PROCESS_SYNCHRONIZATION_PLAN.md):** Concurrency primitives, spinlocks, semaphores, futexes, and lockdep diagnostics.
- **[Physical Memory Manager Plan](PHYSICAL_MEMORY_MANAGER_PLAN.md):** Two-stage physical page allocation and bitmap memory tracking.
- **[IPC Handler Plan](ipc-handler.md):** Inter-process communication channels, endpoints, and zero-copy shared memory.

### Userspace & System Services
- **[System Utilities Development Plan](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md):** Coreutils replacements, POSIX compatibility, and CLI utilities.
- **[SigmaOS GUI Development Plan](SIGMAOS_GUI_DEVELOPMENT_PLAN.md):** Zenith desktop environment, Hyprland/wlroots compositor integration, and Waybar/Walker widgets.
- **[SigmaOS CLI Development Plan](SIGMAOS_CLI_DEVELOPMENT_PLAN.md):** System CLI router, interactive shells, and system diagnostics.
- **[Command Interpreter Plan](34-SigmaOS-Command-Interpreter-Plan.md):** High-performance command interpreter engine.
- **[Device Driver Development Plan](35-SigmaOS-Device-Driver-Development-Plan.md):** Sovereign driver framework, PCI enumeration, and NVMe/AHCI/network drivers.
- **[Network Development Plan](28-Network-Development-And-Maintenance-Plan.md):** Network stack architecture, socket routing, and eBPF packet processing.
- **[System Apps Development Plan](32-System-Apps-Development-Plan.md):** System administration applications and graphical control panels.
