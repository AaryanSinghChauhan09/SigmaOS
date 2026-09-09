# 🚀 SigmaOS — The Post-Linux Sovereign Operating System

**SigmaOS** is a sovereign, zero-dependency operating system written in Rust, designed to defeat Linux and BSD distros through superior architecture, security, and performance.

## 🚀 Core Features

- **Zero-Dependency Kernel (`#![no_std]`)**: No external third-party crates, pure Rust `alloc::` primitives for sovereign operations.
- **Multi-Architecture Support**: x86_32, x86_64, aarch64, riscv64, loongarch64, powerpc64, s390x.
- **Universal Package Management**: Native Sigma-pkg with cross-distro adapters (.deb, .rpm, PKGBUILD, ebuild, apk, snap, flatpak, hpkg).
- **OpenBSD pledge/unveil Security**: Capability-based sandboxing and path unveil for process isolation.
- **FreeBSD Jails & ZFS BootEnv**: Container-level isolation and boot environment management.
- **Illumos Zones & DTrace**: Solaris-inspired containerization and dynamic tracing framework.
- **NixOS Content-Addressed Store**: Hermetic package storage with atomic garbage collection.
- **Linux io_uring Parity**: Asynchronous I/O engine for high-performance networking and storage.
- **Zero-Trust Access Control & MAC:** Discretionary (DAC), Mandatory Access Control (MAC LSM Inode/Ptrace/Socket hooks), and Role-Based (RBAC) security enforcers.
- **Zenith Desktop & Sovereign Media Suite:** Built-in zero-dependency multimedia tools, video editor (SigmaCut), audio DSP, and responsive UI components.
- **AI Agents Master Guide:** [[AI_AGENTS_GUIDE]] - Authoritative reference for autonomous coding agents and subagents.
- **AI Agents UX Management Guide:** [[AI_AGENTS_UX_MANAGEMENT_GUIDE]] - Interface, visual layout, and UX guidelines for autonomous AI agents.
- **AI Agents Time Management Guide:** [[AI_AGENTS_TIME_MANAGEMENT_GUIDE]] - Timekeeping primitives, clock sync, and temporal architecture for autonomous AI agents.
- **AI Agents Security Management Guide:** [[AI_AGENTS_SECURITY_MANAGEMENT_GUIDE]] - Capability sandboxing, PQC attestation, MAC, and digital forensics for autonomous AI agents.
- **AI Agents Procedure Call Management Guide:** [[AI_AGENTS_PROCEDURE_CALL_MANAGEMENT_GUIDE]] - Syscall dispatchers, FFI bindings, zero-copy IPC ring channels, and RPC for autonomous AI agents.
- **AI Agents Ballooning Management Guide:** [[AI_AGENTS_BALLOONING_MANAGEMENT_GUIDE]] - VirtIO memory ballooning, RAM inflation/deflation, and hypervisor overcommit management for AI agents.
- **AI Agents Boot Management Guide:** [[AI_AGENTS_BOOT_MANAGEMENT_GUIDE]] - UEFI/BIOS handoff, Multiboot2, Secure Boot verification, boot optimization, and init handoff for AI agents.
- **AI Agent Carry Flag Management Guide:** [[AI_AGENT_CARRY_FLAG_MANAGEMENT]] - Hardware status flags, bignum arithmetic carry chains, and ALU emulation for AI agents.
- **AI Agent C-SCAN Policy Management Guide:** [[AI_AGENT_CIRCULAR_SCAN_POLICY_MANAGEMENT]] - Circular SCAN elevator disk scheduling, LBA sector ordering, and wrap-around semantics for AI agents.
- **AI Agent Cloned Process Management Guide:** [[AI_AGENT_CLONED_PROCESS_MANAGEMENT]] - POSIX fork, clone flags (CLONE_VM, CLONE_FILES, CLONE_THREAD), and job object inheritance for AI agents.
- **AI Agent Commands Management Guide:** [[AI_AGENT_COMMANDS_MANAGEMENT]] - Sovereign command suite, privilege delegation (sudo/doas), task monitoring, sysctl, and multi-distro CLI for AI agents.
- **AI Agent Time Sharing System Management Guide:** [[AI_AGENT_TIME_SHARING_SYSTEM_MANAGEMENT]] - Quantum time slicing, POSIX SCHED_RR, EEVDF virtual deadlines, and MLFQ priority decay for AI agents.
- **AI Agent Semaphores Management Guide:** [[AI_AGENT_SEMAPHORES_MANAGEMENT]] - IPC namespace counting semaphores, System V IPC, eventfd EFD_SEMAPHORE, and NT semaphores for AI agents.
- **AI Agent Semaphores Operation Management Guide:** [[AI_AGENT_SEMAPHORES_OPERATION_MANAGEMENT]] - Atomic wait (P/down), signal (V/up), SEM_UNDO auto-reversal, and wait queue wakeups for AI agents.
- **AI Agent Consolidation Ratio Management Guide:** [[AI_AGENT_CONSOLIDATION_RATIO_MANAGEMENT]] - VirtIO memory ballooning, RAM overcommit ratios, KSM page deduplication, and VM density for AI agents.
- **AI Agent Context Data Operation Management Guide:** [[AI_AGENT_CONTEXT_DATA_OPERATION_MANAGEMENT]] - Context Virtual MMU page allocation, PawThreeLayerMemory live context pruning, and token budgeting for AI agents.
- **AI Agent Contiguous Allocation Operation Management Guide:** [[AI_AGENT_CONTIGUOUS_ALLOCATION_OPERATION_MANAGEMENT]] - CMA physical reservation, DMA buffer coalescing, and vmalloc virtual contiguity for AI agents.
- **AI Agent Data Operation Management Guide:** [[AI_AGENT_DATA_OPERATION_MANAGEMENT]] - Content-Addressed Storage DAG nodes, PQC data signing, transactional journaling, and Soft Updates for AI agents.
- **AI Agent Zones Operation Management Guide:** [[AI_AGENT_ZONES_OPERATION_MANAGEMENT]] - Solaris container zones, VNIC networking, FreeBSD VM zones, and thermal power zones for AI agents.
- **AI Agent C Language Elimination Guide:** [[AI_AGENT_C_LANGUAGE_ELIMINATION]] - C dependency reduction, malloc/free replacement, RAII memory safety, and pure Rust driver frameworks for AI agents.
- **Master Linux & BSD Distro Strategic Roadmap:** [[MASTER_LINUX_BSD_GAP_CLOSURE_STRATEGIC_PLAN]] - Master strategic plan, gap closure roadmap, 2026-2029+ timeline, and strategy to surpass Linux distros.
- **Linux & BSD Distro Components & Engineering Guidelines:** [[LINUX_BSD_DISTRO_COMPONENTS_AND_GUIDELINES]] - Cross-subsystem Linux and BSD inspirations, component maps, and engineering guidelines.
- **AI Agent Omarchy Navigation Management Guide:** [[AI_AGENT_OMARCHY_NAVIGATION_MANAGEMENT]] - Omarchy/Hyprland tiling layouts, window grouping, scratchpad overlays, and hotkeys for AI agents.
- **AI Agent Universal Package Management Guide:** [[AI_AGENT_UNIVERSAL_PACKAGE_MANAGEMENT]] - Foreign package adapters (.deb, .rpm, PKGBUILD, ebuild, apk, hpkg), dependency canonicalization, and Universal PM for AI agents.
- **AI Agent GitHub Workflows Management Guide:** [[AI_AGENT_GITHUB_WORKFLOWS_MANAGEMENT]] - CI/CD, multi-distro matrix, reproducible SBOMs, Cosign signing, and Pages auto-deploy workflows for AI agents.
- **AI Agent GitHub Wiki Management Guide:** [[AI_AGENT_GITHUB_WIKI_MANAGEMENT]] - Dual-repository wiki synchronization, Home.md index updates, and zero-drift documentation rules for AI agents.

---

## 🧩 Core Architecture & Features
- **Boot to Web**: Minimal Linux (Buildroot) base, boots directly into Chromium in ~3s.
- **Browser as Shell**: Workspaces, window management, and hardware interfaces powered by web apps.
- **Unix Philosophy for Web Apps**: PWAs gain raw access to pipes, spawn, mmap, and `/dev`.
- **Zero-Bloat Package Management**: Alpine packages installed directly via browser APIs.
- **Strict Capabilities System**: Websites must explicitly request hardware/file access.
- **Safe-Rust 12-Shard Microkernel**: Twelve shard taxonomy replacing 500+ legacy apps with native abstractions ([SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V22.md](SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V22.md)).

---

## 🛠️ Quick Start & Building
```bash
# Build the core library
cargo check --lib

# Run the native test suite
./run_sigma_tests.sh
```

For complete installation and compilation guides, refer to [INSTALL.md](INSTALL.md) and [BUILD.md](BUILD.md).
