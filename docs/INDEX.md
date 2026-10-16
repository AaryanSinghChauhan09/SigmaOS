# SigmaOS Master Documentation Index

Welcome to the SigmaOS Architecture and Subsystem Development Master Index. This document indexes all major subsystem development plans, architectural specifications, PR proposals, and operational matrices across `docs/` and `wiki/`.

---

## Architectural & Development Plans (`docs/`)

- [Dispatcher Architecture & Development Plan](DISPATCHER_PLAN.md) - Master Architecture & Support Matrix Plan defining boundaries between scheduler, context-switch, interrupt handling, and deferred work; 8 invariants, 16-PR roadmap, periodic maintenance cadences, component rules, PR template, and merge gates.
- [Signal Handler Architecture & Development Plan](SIGNAL_HANDLER_PLAN.md) - POSIX.1-2024 signal handler architecture, required safety rules, 18-PR roadmap, x86_64 frame & sigreturn validation rules, periodic maintenance schedule, and merge gates.
- [Signal Path Audit & ABI Profile](SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE.md) - Subsystem component audit, POSIX signal numbers and default actions, data structure ABI layouts, and x86_64 user frame layout.
- [Physical Memory Manager Plan](PHYSICAL_MEMORY_MANAGER_PLAN.md) - Two-stage physical memory management architecture, 11 PMM rules & invariants, 20-PR sequence roadmap, periodic maintenance schedule, cross-component implementation rules, and merge gates.
- [Process Manager Plan](PROCESS_MANAGER_PLAN.md) - Process Manager Master Architecture, 18-PR sequence roadmap, design and safety rules, periodic maintenance PR cadence, component rules, PR template, merge gates, and completion criteria.
- [System Utilities Development Plan](SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md) - Baseline rules, POSIX compatibility, 18-PR roadmap, component rules, periodic maintenance framework, and merge gates.
- [Filesystem & Storage Parity Proposal](PR_PROPOSAL_FILESYSTEMS_AND_STORAGE_PARITY.md) - Multi-tiered storage engines, zswap writeback cache, and OverlayFS mount simulation.
- [Toolchain & Development Ecosystem Proposal](PR_PROPOSAL_TOOLCHAIN_AND_DEVELOPMENT_PARITY.md) - Zero-dependency toolchain engines, compilers, build orchestration, and dynamic tracing.

---

## Core Wiki Subsystem Guides (`wiki/`)

- [Dispatcher Plan Mirror](../wiki/DISPATCHER_PLAN.md)
- [Signal Handler Plan](../wiki/SIGNAL_HANDLER_PLAN.md)
- [Signal Path Audit & ABI Profile](../wiki/SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE.md)
- [System Utilities Plan](../wiki/SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md)
- [Architectural Pillars](../wiki/Architectural-Pillars-Components.md)
- [Kernel Functions & ABI](../wiki/Kernel-Functions-and-ABI-Components.md)
- [Kernel Thread & ThreadPool](../wiki/Kernel-Thread-and-ThreadPool-Components.md)
- [POSIX Signal Handling](../wiki/POSIX-Signal-Handling-Components.md)
- [Process Lifecycle & Subreaper](../wiki/Process-Lifecycle-and-Subreaper-Components.md)
- [Physical Memory Manager Plan Mirror](../wiki/PHYSICAL_MEMORY_MANAGER_PLAN.md)
- [Process Manager Plan Mirror](../wiki/PROCESS_MANAGER_PLAN.md)
- [Resource Governor & Cgroups](../wiki/Resource-Governor-and-Cgroups-Components.md)
- [Sigma Boot & Secure Enclave](../wiki/Sigma-Boot-Components.md)
- [Sigma Sandbox Confinement](../wiki/Sigma-Sandbox-Confinement-Components.md)
- [SigPkg Universal Package Manager](../wiki/SigPkg-Universal-Package-Manager-Components.md)
- [Subsystem Integration](../wiki/Subsystem-Integration-Components.md)
- [System Governance & RFC Framework](../wiki/System-Governance-and-RFC-Components.md)
