# SigmaOS Wiki Master Documentation Index

Welcome to the SigmaOS GitHub Wiki Encyclopedia and Subsystem Reference Index.

---

## Core Development Plans & Architecture

- [Dispatcher Architecture & Development Plan](DISPATCHER_PLAN.md) - Master Architecture & Support Matrix Plan defining boundaries between scheduler, context-switch, interrupt handling, and deferred work; 8 invariants, 16-PR roadmap, periodic maintenance cadences, component rules, PR template, and merge gates.
- [Signal Handler Architecture & Development Plan](SIGNAL_HANDLER_PLAN.md) - POSIX.1-2024 signal handler architecture, required safety rules, 18-PR roadmap, x86_64 frame & sigreturn validation rules, periodic maintenance schedule, and merge gates.
- [Signal Path Audit & ABI Profile](SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE.md) - Subsystem component audit, POSIX signal numbers and default actions, data structure ABI layouts, and x86_64 user frame layout.
- [System Utilities Development Plan](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md) - Master Architecture, POSIX compatibility baseline, 18-PR roadmap, component rules, periodic maintenance framework, and review merge gates.
- [System Utilities Plan Mirror](SYSTEM_UTILITIES_PLAN.md) - Alternate index page for system utilities architecture.

---

## Subsystem Architecture References

- [Architectural Pillars & Components](Architectural-Pillars-Components.md)
- [Audit & Compliance Components](Audit-and-Compliance-Components.md)
- [Compiler & Toolchain Parity](Compiler-Toolchain-Components.md)
- [Interrupt Handling & APIC Architecture](Interrupt-Handling-and-APIC-Components.md)
- [Kernel Core Library (Klib)](Kernel-Core-Library-Klib-Components.md)
- [Kernel Functions & Syscall ABI](Kernel-Functions-and-ABI-Components.md)
- [Kernel Thread & ThreadPool Manager](Kernel-Thread-and-ThreadPool-Components.md)
- [POSIX Signal Handling Subsystem](POSIX-Signal-Handling-Components.md)
- [Process Lifecycle & Subreaper Engine](Process-Lifecycle-and-Subreaper-Components.md)
- [Resource Governor & Cgroups Engine](Resource-Governor-and-Cgroups-Components.md)
- [Secure Enclave & Boot Verification](Secure-Enclave-and-Boot-Components.md)
- [SigPkg Universal Package Manager](SigPkg-Universal-Package-Manager-Components.md)
- [Sigma Bootloader & Multiboot2 Engine](Sigma-Boot-Components.md)
- [Sigma Sandbox Confinement & Landlock](Sigma-Sandbox-Confinement-Components.md)
- [Sigma Validation & Test Suite](Sigma-Validation-Suite-Components.md)
- [Subsystem Integration Matrix](Subsystem-Integration-Components.md)
- [System Logging & Rotation Daemon](System-Logging-and-Rotation-Components.md)
- [System Notification Daemon](System-Notification-Daemon-Components.md)
- [Timer & High-Resolution Clock Services](Timer-and-High-Res-Clock-Components.md)
