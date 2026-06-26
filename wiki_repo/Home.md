# SigmaOS Zenith — Home

<div align="center">

**v15.2 [ZENITH-SINGULARITY]** · Silicon-Direct · Zero-Dependency · Post-Quantum

</div>

---

## 🚀 Quick Navigation

| Section | Description |
| :--- | :--- |
| [Architecture Overview](Architecture-Overview) | Ring-0/3 layout, HAL, shard map, boot sequence |
| [Kernel Internals](SigmaOS-Kernel-Internals) | CFS scheduler, NUMA, slab allocator, real-time class |
| [HAL](HAL) | x86_64 / ARM64 / RISC-V hardware abstraction |
| [Syscall Dispatcher](SyscallDispatcher) | Modular O(1) C dispatch table, 256-slot registry |
| [Storage](Storage) | VFS, ZFS-inspired CoW, SovereignCloudFS |
| [Networking](Networking-Shard) | TCP/IP stack, DNS resolver, loopback NIC |
| [Security Framework](Sovereign-Security-Framework) | PQC, Dilithium-5 boot chain, MAC policies |
| [Desktop UI](ZENITH_UI) | Zenith glassmorphic compositor, Vulkan layer |
| [Branch Guide](Branch-Guide) | 12-branch taxonomy, targets, and status |
| [Getting Started](Getting-Started) | Build instructions, toolchain setup, QEMU |
| [Contributing](Contributor-Guidelines) | PR process, code style, commit conventions |
| [Onboarding Guide](Onboarding_Guide) | Coding standards and environment setup |
| [Problems & Bugs](Problems) | Active bug ledger and resolution log |
| [RFC Template](RFC_Template) | Proposal format for new subsystem features |
| [Changelog](CHANGELOG) | Release history and version notes |
| [Roadmap](Roadmap) | Strategic improvement sequence |
| [Competitor Comparison](Competitor-Comparison) | Distro gap analysis and surpass strategy |
| [Phase A Checklist](Phase-A-Execution-Checklist) | File-level execution tracker |
| [Phase 7–8 Roadmap](Phase-7-8-Roadmap) | Automation, CLI, GUI, branch parity |
| [Feature Matrix](Feature-Matrix) | Branch subsystem parity |
| [Zenith GUI Toolkit](Zenith-GUI-Toolkit) | Compositor, tiling, profiles |
| [Automation & CLI](Automation-CLI-Engine) | sigma_automation + sigma-cli |
| [Containers](Containers-Orchestrator) | sigma-pod native orchestration |
| [Boot Resilience](Bootloader-Resilience) | Safe Mode + Fix-it menu |
| [Phase B Checklist](Phase-B-Execution-Checklist) | Unified automation + CLI + GUI |
| [Phase C Checklist](Phase-C-Execution-Checklist) | Meta-distro subsystem integration |
| [Meta-Distro Engine](Meta-Distro-Unified-Engine) | Competitor → subsystem map |
| [Meta-Distro Registry](Meta-Distro-Registry) | `sigma_meta_distro_init()` hub |
| [Problems Manifest](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CURRENT_PROBLEMS_MANIFEST.md) | Active bugs and Phase A status |

---

## ⚡ Core Capabilities

| 🔧 Subsystem | ✅ Implementation | 🎯 Advantage |
| :--- | :--- | :--- |
| **CFS Scheduler** | NUMA-balanced, inline ASM context switch | Zero-drift thread scheduling |
| **Slab Allocator** | O(1) lockless, power-of-2 buckets | Fragmentation-free heap |
| **SPSC IPC** | Lock-free zero-copy ring buffer | Sub-microsecond inter-shard messaging |
| **S-HAL** | x86_64 · ARM64 · RISC-V | Single codebase for 3 ISAs |
| **VulkanLayer** | Direct SPIR-V GPU routing | No SDK overhead |
| **Syscall Table** | 256-slot O(1) C dispatch | Runtime handler registration |
| **PQC Engine** | Dilithium-5 attestation | Post-quantum secure boot |
| **Zenith Desktop** | Glassmorphic Vulkan compositor | Hardware-accelerated UI |
| **SovereignVFS** | ZFS-inspired CoW + OverlayFS | Atomic rollback, zero data loss |
| **Container Runtime** | Static sandbox with locked FS | App-store grade isolation |

---

## 🌿 Branch Architecture (12 Targets)

| Branch | Archetype | Status |
| :--- | :--- | :--- |
| `main` | Stable Production | ✅ Active |
| `release/standalone` | Bare-Metal Desktop | 🔨 Development |
| `release/rtos` | Real-Time Embedded | 🔨 Development |
| `release/mobile` | Energy-Aware Mobile | 🔨 Development |
| `release/microkernel` | Ultra-Minimal (120 shards) | ✅ Test-verified |
| `release/dual-boot` | Co-operative Boot | 🔨 Development |
| `release/distributed` | Cluster-Native | 🔨 Development |
| `release/cloud` | Headless Virtualization | 🔨 Development |
| `release/browser` | WebAssembly Runtime | 🔨 Development |
| `release/app` | App-Store Sandbox | 🔨 Development |
| `performance-optimized` | SIMD AVX-512 / Neon | 🔬 Experimental |
| `gh-pages` | Static Web Portal | ✅ Live |

See [Branch Guide](Branch-Guide) for the full per-branch breakdown.

---

## 📐 Architecture at a Glance

```
┌──────────────────────────────────────────┐
│  Ring-3 Userland                         │
│  sigma-sh │ Zenith Desktop │ sigma-forge  │
└──────────────────┬───────────────────────┘
                   │ syscall / SYSRET
┌──────────────────▼───────────────────────┐
│  SyscallDispatcher  (256-slot O(1))      │
└──────────────────┬───────────────────────┘
                   │
┌──────────────────▼───────────────────────┐
│  Sovereign Kernel Lattice (Ring-0)       │
│  CFS · Slab · VFS · SPSC IPC · PQC      │
└──────────────────┬───────────────────────┘
                   │
┌──────────────────▼───────────────────────┐
│  S-HAL                                   │
│  x86_64 APIC │ ARM64 GIC │ RISC-V PLIC  │
└──────────────────┬───────────────────────┘
                   │
          ⚙️ Physical Hardware
```

---

## 🤝 Contributing

Read [CONTRIBUTING.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTING.md) for the full guide including branch strategy, commit conventions, and PR process.

**Key rules at a glance:**
- No `stdlib.h` / `stdio.h` in Ring-0 kernel shards
- Use bounded string ops (`strncpy`, `snprintf`) — never `strcpy`
- All PRs must pass `npm run test` (82 green tests required)
- Commit format: `type(scope): message`

---

> **Σ SigmaOS** — Absolute Sovereignty. Singularity Achieved.
> *v15.2 [ZENITH-SINGULARITY] · Build-Verified · 100% Silicon Purity*
