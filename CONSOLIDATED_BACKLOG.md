# 📋 SigmaOS Consolidated Feature & Architecture Backlog

This document establishes the official discovery inventory, triage classification matrix, reference architecture mapping, and 90-day execution milestone plan for all feature proposals, TODOs, and roadmaps documented across the SigmaOS repository and wiki.

---

## 1. 🔍 Discovery & Backlog Inventory Matrix

The table below summarizes key roadmap proposals extracted from `.md` files and `wiki/` pages across the codebase:

| Item ID | Title / Proposal | Primary Source Location | Summary / Description | Status | Domain / Classification | Priority | Reference Pattern / Upstream |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **BLK-01** | Microkernel Cap & S-AMNESIA Volatile Sandbox | `EVALUATION_AND_FUTURE_VISION.md` | Hardware-enforced volatile memory sandboxing and Capability-Based Access Control (CBAC) for process isolation. | Proposed | Security / Microkernel | **P0 - Blocker** | seL4, FreeBSD Capsicum |
| **BLK-02** | Sovereign Memory Compactor (`SovereignMemoryCompactor`) | `src/distro/linux_bsd_inspirations.rs` | Proactive LRU page eviction and background memory defragmentation engine. | Implemented / Hardened | Kernel (`mm`) | **P0 - Blocker** | Linux `kswapd`, FreeBSD `vm_pageout` |
| **BLK-03** | S-AUR Peer-to-Peer & S-ABS SIMD Compiler | `wiki/Arch-Linux-and-AUR-Parity.md` | Decentralized P2P package verification and AVX-512 targeted compilation cache. | Partially Implemented | Package Mgmt (`sigpkg`) | **P1 - High** | Arch Linux (AUR / makepkg), Gentoo Portage |
| **BLK-04** | Advanced Multi-Class Scheduler (CFS/Real-Time/EDF) | `src/kernel/sched/scheduler.rs` | Completely Fair Scheduling, 140-priority RT arrays, and Earliest Deadline First (SCHED_DEADLINE) execution. | Implemented / Hardened | Kernel (`sched`) | **P0 - Blocker** | Linux CFS / BORE, FreeBSD ULE |
| **BLK-05** | Wayland Zenith Compositor Surface Layers | `src/desktop/zenith_compositor.rs` | Hardware-accelerated software surface compositing, Master-Stack / BSP window layout engines, and themes. | Implemented / Hardened | Desktop / Compositor | **P2 - Medium** | wlroots, Sway, Wayland Protocols |
| **BLK-06** | BSD kqueue to epoll KPI Bridge & Aux Vectors | `src/compatibility/chimera_linux.rs` | Bridge layer mapping BSD kqueue event filters to Linux epoll and providing auxiliary vector dynamic linking. | Implemented / Hardened | Userland / Compatibility | **P1 - High** | Chimera Linux, FreeBSD userland KPI |
| **BLK-07** | QEMU/KVM IOPS Throttle & SPICE Integration | `src/virtualization/vm_manager.rs` | Dynamic IOPS bandwidth limits, QXL VRAM allocation, and SPICE remote desktop display server support. | Implemented / Hardened | Virtualization | **P1 - High** | QEMU / KVM, Firecracker MicroVMs |
| **BLK-08** | Sovereign Podcast Recording & RSS Publisher | `src/audio/podcast.rs` | GarageBand/Anchor parity multi-track audio engine with mastering effects and iTunes/Anchor RSS XML generation. | Implemented / Hardened | Audio / Multimedia | **P3 - Low** | GarageBand, Anchor.fm, Audacity |
| **BLK-09** | Debian Distro Compatibility Subsystem | `src/package/debian.rs` | Parsing and handling of DEB packages, control blocks, sources.list, and dpkg status entries for APT simulation. | Implemented / Hardened | Package Mgmt (`sigpkg`) | **P1 - High** | Debian dpkg, Ubuntu APT |
| **BLK-10** | Local LLM PagedAttention & Grammar Logits Processor | `src/ai/llm.rs` | vLLM-inspired KV cache block allocation, llama.cpp speculative decoding, and Outlines grammar constraints. | Implemented / Hardened | AI / Automation | **P2 - Medium** | vLLM, llama.cpp, Outlines |
| **BLK-11** | Multiboot2 Specification Loader & MBI Parser | `src/boot/multiboot2.rs` | Aligned header validation, MBI tag parsing (cmdline, bootloader, mmap, framebuffer), and compliant boot loader. | Implemented / Hardened | Boot / Firmware | **P0 - Blocker** | Multiboot2 Spec, GRUB2 |
| **BLK-12** | antiX Live Persistence & Snapshot Engine | `src/compatibility/antix.rs` | Static/dynamic Live overlay persistence, ISO snapshot remastering, and network switcher CLI tools. | Implemented / Hardened | Userland / Compatibility | **P2 - Medium** | antiX Linux, MX Linux live-usb |

---

## 2. 📊 CSV Representation

```csv
Item_ID,Title,Source_Location,Summary,Status,Domain,Priority,Upstream_Reference
BLK-01,Microkernel Cap & S-AMNESIA Volatile Sandbox,EVALUATION_AND_FUTURE_VISION.md,Hardware-enforced volatile memory sandboxing and Capability-Based Access Control,Proposed,Security / Microkernel,P0 - Blocker,seL4 / FreeBSD Capsicum
BLK-02,Sovereign Memory Compactor,src/distro/linux_bsd_inspirations.rs,Proactive LRU page eviction and background memory defragmentation,Implemented,Kernel (mm),P0 - Blocker,Linux kswapd / FreeBSD vm_pageout
BLK-03,S-AUR Peer-to-Peer & S-ABS SIMD Compiler,wiki/Arch-Linux-and-AUR-Parity.md,Decentralized P2P package verification and AVX-512 targeted compilation cache,Partially Implemented,Package Mgmt (sigpkg),P1 - High,Arch Linux / Gentoo Portage
BLK-04,Advanced Multi-Class Scheduler,src/kernel/sched/scheduler.rs,Completely Fair Scheduling and SCHED_DEADLINE,Implemented,Kernel (sched),P0 - Blocker,Linux CFS / FreeBSD ULE
BLK-05,Wayland Zenith Compositor Surface Layers,src/desktop/zenith_compositor.rs,Software surface compositing and window tiling layouts,Implemented,Desktop / Compositor,P2 - Medium,wlroots / Sway
BLK-06,BSD kqueue to epoll KPI Bridge,src/compatibility/chimera_linux.rs,Bridge layer mapping BSD kqueue event filters to Linux epoll,Implemented,Userland / Compatibility,P1 - High,Chimera Linux / FreeBSD
BLK-07,QEMU/KVM IOPS Throttle & SPICE Integration,src/virtualization/vm_manager.rs,Dynamic IOPS limits and SPICE display server integration,Implemented,Virtualization,P1 - High,QEMU / KVM / Firecracker
BLK-08,Sovereign Podcast Recording & RSS Publisher,src/audio/podcast.rs,Multi-track audio engine and iTunes/Anchor RSS XML generation,Implemented,Audio / Multimedia,P3 - Low,GarageBand / Anchor.fm
BLK-09,Debian Distro Compatibility Subsystem,src/package/debian.rs,DEB packages and dpkg status database simulation,Implemented,Package Mgmt (sigpkg),P1 - High,Debian dpkg / Ubuntu APT
BLK-10,Local LLM PagedAttention & Grammar Logits,src/ai/llm.rs,KV cache block allocation and grammar constrained logits,Implemented,AI / Automation,P2 - Medium,vLLM / llama.cpp
BLK-11,Multiboot2 Specification Loader,src/boot/multiboot2.rs,MBI tag parsing and compliant bootloader,Implemented,Boot / Firmware,P0 - Blocker,Multiboot2 / GRUB2
BLK-12,antiX Live Persistence & Snapshot Engine,src/compatibility/antix.rs,Live overlay persistence and ISO remastering,Implemented,Userland / Compatibility,P2 - Medium,antiX Linux / MX Linux
```

---

## 3. 🎯 Triage & Classification Structure

Backlog tasks are prioritized across 4 operational tiers:

- **P0 - Blocker / Critical Path**: Core bootloaders (`Multiboot2`, `UEFI`), CPU memory allocators/compactors (`SovereignMemoryCompactor`), multi-class scheduler (`sched`), and capability sandboxing (`Pledge`/`Unveil`).
- **P1 - High Priority (Distro Parity & Compatibility)**: Distro compatibility shims (`Debian`, `Chimera`, `Fedora`, `antiX`), virtual machine hypervisor drivers (`QemuBackend`), and package management (`sigpkg`).
- **P2 - Medium Priority (Desktop & AI Stack)**: Wayland Zenith Compositor surface rendering, Desktop notifications, local LLM inference engines (`PagedAttention`), and performance governors (`CachyOS` P-State scaling).
- **P3 - Low Priority / Developer Tooling**: `sigmatools` compliance & QA suites (`SigmaQA`, `SigmaCertify`), audio recording/mastering pipelines (`Sovereign Podcast`), and documentation generators.

---

## 4. 📅 90-Day Execution Milestone Plan

### Sprint 0 (Weeks 1–2): Inventory & Infrastructure Baseline
- Establish automated backlog inventory tracking across repository wiki pages and markdown specifications.
- Configure multi-architecture QEMU runner workflows (`x86_64`, `aarch64`, `riscv64`).
- Validate zero-warning compilation baseline across all feature flags.

### Sprint 1 (Weeks 3–6): Kernel Hardening & Security Isolation
- Implement S-AMNESIA volatile memory sandboxing RFC specifications.
- Extend `Pledge` and `Unveil` path validation with zero-allocation slice inspection.
- Harden Multiboot2 and UEFI secure boot trust chain verification.

### Sprint 2 (Weeks 7–10): Compatibility & Userland Parity
- Expand Debian/APT and Arch/AUR package specification resolvers in `sigpkg`.
- Polish Zenith Compositor tiling algorithms (BSP / Master-Stack layout refinement).
- Refine local LLM PagedAttention memory management for resource-constrained hardware profiles.

### Sprint 3 (Weeks 11–13): Testing, Compliance & Release
- Execute full test matrix via `./run_sigma_tests.sh` and `rustc --test` suites.
- Validate FIPS 140-3 cryptographic compliance benchmarks using `SigmaCertify`.
- Publish SigmaOS Developer Preview bootable ISO images and VM appliances.
