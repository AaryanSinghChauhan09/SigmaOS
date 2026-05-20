# SigmaOS Zenith — Home

Welcome to the **SigmaOS Zenith v15.2** knowledge base. SigmaOS is an industrial-grade, sovereign microkernel operating system built on a 600-shard C++ singleton lattice — targeting x86_64, ARM64, and RISC-V without any monolithic Linux or Windows dependency.

---

## 🚀 Quick Navigation

| Section | Description |
| :--- | :--- |
| [Architecture Overview](Architecture-Overview) | Kernel shard map, HAL, and Ring-0/3 dispatch pipeline |
| [Kernel](Kernel) | CFS scheduling, NUMA, slab allocator, real-time class |
| [HAL](HAL) | x86_64 / ARM64 / RISC-V hardware abstraction |
| [Syscall Dispatcher](SyscallDispatcher) | Modular O(1) C dispatch table |
| [Storage](Storage) | VFS, ZFS-inspired CoW, SovereignCloudFS |
| [Desktop](Desktop) | Zenith UI, Vulkan compositor, SovereignThemeEngine |
| [Tools](Tools) | Professional calculators, forensics, developer tools |
| [Branches](Branches) | 12-branch taxonomy and improvement roadmap |
| [Onboarding Guide](Onboarding_Guide) | Build instructions and coding standards |
| [Problems](Problems) | Bug ledger and resolution log |
| [Improvements Inspired by Linux Distros](SigmaOS-Improvements-Inspired-by-Linux-Distros) | Layered OS maturity roadmap and multi-distro strategic synthesis |
| [Sovereign Launch & Expansion Roadmap](SigmaOS-Development-Roadmap-and-Architecture) | Industrial development roadmap and architecture specification mapping key branches |
| [RFC Template](RFC_Template) | Proposal format for new subsystem features |

---

## ⚡ Core Capabilities

- **Shard-Aware CFS Scheduler** — NUMA-balanced with inline assembly context switches
- **O(1) Slab Allocator** — Lockless, fragmentation-free, power-of-2 bucket design
- **Lock-Free SPSC IPC** — Zero-copy ring buffer for inter-shard messaging
- **SovereignVulkanLayer** — Direct SPIR-V GPU shader routing without Vulkan SDK
- **Modular C Syscall Dispatcher** — 256-slot registry with runtime handler registration
- **Post-Quantum Cryptography** — Dilithium-5 attested boot and IPC signatures
- **Glassmorphic Desktop** — Zenith UI with hardware-composited glassmorphism

---

> **Σ SigmaOS**: Absolute Sovereignty. Singularity Achieved.
