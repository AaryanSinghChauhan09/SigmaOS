# SigmaOS Wiki

Welcome to the **SigmaOS Wiki** — the comprehensive knowledge base, architectural specification, and engineering roadmap for SigmaOS.

---

## 🗺️ Strategic Roadmaps & Operational Guidelines
- 🚀 **[[Future Development Roadmap|FUTURE-DEVELOPMENT-ROADMAP]]**: 100-milestone master architecture, zero-dependency sovereign design, and release cadence.
- 📋 **[[What Works & What Needs Attention|WHAT_IS_WORKING_AND_NOT_WORKING]]**: AI agent verification manual, active status matrix, and subsystem audit results.
- 📚 **[[Self-Sufficiency Encyclopedia V56|15-Sovereign-OS-Self-Sufficiency-Encyclopedia-V56]]**: Complete sovereign operating system self-sufficiency manual.
- 🧬 **[[Tri-Agent 500-Repos Absorption Master Plan|20-Tri-Agent-500-Repos-Absorption-Master-Plan]]**: Full absorption architecture for 500 open-source repositories.
  - 📖 **[[500 Repositories Absorption Catalog|20-Tri-Agent-500-Repos-Absorption-Catalog]]**
  - 🗺️ **[[500 Repositories Implementation Roadmap|20-Tri-Agent-500-Repos-Implementation-Roadmap]]**

---

## ⚙️ Kernel Subsystems & Architectural Specifications
- 🔄 **[[Process Manager Plan|PROCESS_MANAGER_PLAN]]**: Lifecycle state machines, process tables, POSIX process group semantics, and verification gates.
- 🔒 **[[Process Synchronization Plan|PROCESS_SYNCHRONIZATION_PLAN]]**: Concurrency primitives, futexes, reader-writer locks, and deadlock prevention.
- ⏱️ **[[CPU Scheduler Development Plan|SCHEDULER_DEVELOPMENT_PLAN]]**: CFS, RT scheduler, EEVDF, BORE, and energy-aware task placement.
  - 📊 **[[Scheduler Status & Invariants|SCHEDULER_STATUS_AND_INVARIANTS]]**
- 🧠 **[[Physical Memory Manager Plan|PHYSICAL_MEMORY_MANAGER_PLAN]]**: Buddy allocator, page frame accounting, slab caches, and NUMA-aware paging.
- ⚡ **[[Dispatcher Plan|DISPATCHER_PLAN]]**: Context-switching, interrupt dispatch, IPC handoff, and deferred work queues.
- ⚡ **[[Interrupt Handler Plan|INTERRUPT_HANDLER_PLAN]]**: Vector table management, APIC/IDT dispatch, and MSI-X hardware routing.
- 📡 **[[Signals Architecture Plan|26-Signal-Development-And-Maintenance-Plan]]**: Real-time signal queues, POSIX sigaction ABI, and signal handler profiling.
  - 🔍 **[[Signal Handler Audit & ABI Profile|SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE]]**
- 🛡️ **[[System Call Architecture Plan|36-SigmaOS-Syscall-Development-Plan]]**: Fast syscall dispatcher, register preservation ABI, and seccomp/pledge enforcement.
- 📬 **[[IPC Handler Architecture|ipc-handler]]**: High-performance zero-copy channels, capabilities, and synchronous rendezvous.
- 🔌 **[[Device Driver Development Plan|35-SigmaOS-Device-Driver-Development-Plan]]**: Clean-room hardware abstraction, PCIe, virtio, NVMe, and USB drivers.
- 🌐 **[[Network Development Plan|28-Network-Development-And-Maintenance-Plan]]**: Zero-copy TCP/UDP, BPF/XDP filtering, wireguard, and multi-table routing.
- 💻 **[[Universal CLI Development Plan|27-SigmaOS-CLI-Development-Plan]]**: Terminal REPL, syntax highlighting, command autocompletion, and pipeline execution.
  - 🐚 **[[Command Interpreter Plan|34-SigmaOS-Command-Interpreter-Plan]]**
- 🧰 **[[System Utilities Development Plan|26-System-Utilities-Development-Plan]]**: POSIX coreutils, diagnostic tools, and system administrative binaries.
- 🖥️ **[[SigmaOS GUI Development Plan|30-SigmaOS-GUI-Development-Plan]]**: Zenith compositor, Wayland protocols, and GPU accelerated windowing.

---

## 🌐 Distribution Parity & Interoperability Roadmaps
- 🎯 **[[Omarchy Linux & Desktop Parity AI Agent Roadmap|40-Omarchy-Linux-And-Sovereign-Desktop-Parity-AI-Agent-Roadmap]]**
- 🛡️ **[[Linux & BSD Kernel, Security, Virtualization AI Agent Roadmap|41-Linux-BSD-Kernel-Security-And-Virtualization-AI-Agent-Roadmap]]**
- 🦀 **[[Rust OSDev Ecosystem Synthesis & Parity|42-Rust-OSDev-Ecosystem-Synthesis-And-Parity]]**
- 📦 **[[Universal Linux & BSD Subsystem Interoperability Gateway|PR_PROPOSAL_UNIVERSAL_LINUX_BSD_SUBSYSTEM_INTEROPERABILITY]]**
- 💡 **[[Open Source OS Inspiration V72 PR Suite|47-Open-Source-OS-Inspiration-V72-PR-Suite]]**
- 💡 **[[Open Source OS Inspiration V73 PR Suite|48-Open-Source-OS-Inspiration-V73-PR-Suite]]**

---

## 🔱 BSD Subsystem Parity (DragonFly, FreeBSD, OpenBSD)
SigmaOS natively synthesizes the most advanced primitives across the BSD ecosystem:
- 🐉 **[[DragonFlyBSD-Ecosystem-Parity|DragonFly BSD]]**: HAMMER2 filesystem with instant atomic snapshots, lightweight kernel threads (LWKT), and lock-free token messaging.
- 😈 **[[FreeBSD-Ecosystem-Parity|FreeBSD]]**: Capsicum capability rights sandboxing, Jails process isolation, and kqueue/kevent edge-triggered kernel event dispatch.
- 🐡 **[[OpenBSD-Ecosystem-Parity|OpenBSD]]**: Uncompromising security mitigations with `pledge()` syscall restrictions, `unveil()` path concealment, W^X memory guarantees, and stateful PF firewalling.
- 📊 **[[Universal BSD Synthesis Matrix|BSD-Universal-Synthesis-and-Parity]]**: Master cross-comparison across all BSD families.

---

## 🎨 The 20-Module Craft Creative & Office Productivity Suite
SigmaOS includes a comprehensive, hardware-accelerated creative and office suite built on `#![no_std]` Rust and `CraftLibs`:
- 🧊 **[[1. 3DCraft|3DCraft-Modeling-and-Rendering]]**: 3D polygon modeling, NURBS curves, and BVH-accelerated ray tracing.
- 🎞️ **[[2. AniCraft|AniCraft-Animation-Studio]]**: 2D animation, onion skinning, timing charts, and 2D IK skeletal rigging.
- 🖌️ **[[3. ArtCraft|ArtCraft-Digital-Painting]]**: Digital painting with sub-millisecond stylus latency and dynamic brush mixing.
- 📐 **[[4. CADCraft|CADCraft-Parametric-Engineering]]**: Parametric mechanical 2D sketching, constraint solving, and 3D B-Rep solid modeling.
- 🧬 **[[5. CraftLibs|CraftLibs-Unified-Creative-Foundation]]**: Shared color spaces (sRGB, Display P3, CMYK), SIMD vector math, and undo/redo DAGs.
- 📊 **[[6. DeckCraft|DeckCraft-Presentations]]**: Slide deck authoring with GPU transitions and dual-display presenter mode.
- 📱 **[[7. DesignCraft|DesignCraft-UI-UX-Design]]**: Vector UI/UX interface prototyping and responsive flexbox auto-layout.
- 💥 **[[8. EffectCraft|EffectCraft-Visual-Effects-Compositor]]**: Node-based VFX compositing, optical flow motion estimation, and green screen keying.
- 📈 **[[9. FigCraft|FigCraft-Technical-Diagrams]]**: Technical flowcharts, UML diagrams, and automatic obstacle-avoiding line routing.
- 🎬 **[[10. FilmCraft|FilmCraft-Video-Editor]]**: Non-linear multi-track video editing, proxy playback, and 3D LUT color grading.
- 🔤 **[[11. FontCraft|FontCraft-Typography-Studio]]**: Vector typography design, Bézier curve editing, and OpenType table compilation.
- 📉 **[[12. GridCraft|GridCraft-Reactive-Spreadsheets]]**: Reactive formula dependency DAG evaluating 400+ functions over 10M+ virtualized cells.
- 💡 **[[13. LightCraft|LightCraft-Lighting-and-Color]]**: DMX512 stage lighting, Kelvin color temperature control, and cue sequences.
- 📄 **[[14. PDFCraft|PDFCraft-Document-Engine]]**: PDF 2.0 / PDF-A rendering, AcroForms filling, and cryptographic X.509 signing.
- 📷 **[[15. PhotoCraft|PhotoCraft-Raw-Photo-Studio]]**: Non-destructive RAW camera development, demosaicing algorithms, and lens correction.
- 📰 **[[16. PrintCraft|PrintCraft-Desktop-Publishing]]**: Multi-page DTP layout, CMYK color plate separations, and bleed/slug crop marks.
- 🎧 **[[17. SoundCraft|SoundCraft-Digital-Audio-Workstation]]**: Multi-track DAW, sub-3ms latency audio pipeline, and MIDI 2.0 sequencing.
- ✒️ **[[18. VectorCraft|VectorCraft-Vector-Illustration]]**: Scalable vector graphics (SVG2), Boolean path operations, and gradient meshes.
- 🎙️ **[[19. VoiceCraft|VoiceCraft-Speech-and-Dictation]]**: Real-time on-device neural Whisper speech recognition and expressive TTS synthesis.
- 📝 **[[20. WordCraft|WordCraft-Typographic-Document-Processor]]**: Rich typographical word processor with automatic footnotes and DOCX/ODT support.

---

## 🐧 Linux Ecosystem Parity & Ready to Launch
- 🏹 **[[Arch-Linux-Ecosystem-Parity|Arch Linux]]**: Pacman package database, sandboxed unprivileged AUR builds, and rolling release purity.
- ⚡ **[[CachyOS-Ecosystem-Parity|CachyOS]]**: Low-latency BORE/EEVDF scheduler, x86-64-v3/v4 AVX optimizations, and ZSTD-compressed ZRAM swap.
- 🌀 **[[Debian-Ecosystem-Parity|Debian]]**: Rock-solid stability, multi-architecture dependency resolution, and strict DFSG compliance.
- 🌿 **[[LinuxMint-Parity|Linux Mint]]**: Turnkey user convenience, Mint tools, Timeshift snapshot rollback, and desktop ease of use.
- 👑 **[[Omarchy-Parity|Omarchy Linux]]**: Sovereign multi-table routing, browser theme synchronization, and unified distro experience.
- 🚀 **[[Ready-to-Work-and-Ready-to-Launch|Ready to Work & Ready to Launch]]**: Turnkey hardware autodiscovery, pre-configured productive workspaces (`Super + 1..4`), instant app launcher (`Super + Space`), and 3-second live boot.

---

## 📁 Core Architecture & Subsystem Guides
- [[Kernel-Core|Kernel Core Microarchitecture]]
- [[Memory-Management|Memory Management Subsystem]]
- [[Security-and-Hardening|Security, Hardening & Mandatory Access Control]]
- [[Desktop-Environment|Zenith Desktop Environment & Compositor]]
- [[Package-Management|Universal Package Management (SigPkg, ALPM, APT, DNF)]]
- [[Networking-Stack|TCP/IP & Sovereign Networking Stack]]
- [[Filesystem-VFS|VFS, Journaling & Filesystem Architecture]]
- [[Hardware-Drivers|Hardware Drivers & Bus Architecture]]
- [[Init-and-Services|Init Systems & Service Supervision]]
- [[IPC-and-Syscalls|IPC & System Call Architecture]]
- [[Container-Virtualization|Containerization & Virtualization]]
- [[AI-and-ML-Engine|On-Device AI/ML Engine & Accelerators]]
- [[Shell-and-Userspace|Universal Shell & Userspace Utilities]]
- [[Toolchain-and-Dev|Compiler Toolchains & Development SDK]]
- [[Monitoring-Observability|Monitoring, Observability & Tracing]]
- [[Crypto-and-TPM|Cryptography & TPM 2.0 Hardening]]
- [[Backup-and-Recovery|Backup, Snapshots & Disaster Recovery]]
- [[Cloud-and-Cluster|Clustering & Distributed Services]]
- [[Performance-Benchmarks|Performance Benchmarking & Latency Profiles]]
- [[Theming-Guide|Zenith Theming & Visual Customization]]
- [[Installer-Guide|Live ISO, Installation & Bootstrapping]]

---

### 📁 GitHub Sub-Repositories
- 📦 **[[Sub-Repositories-and-Components|Complete Sub-Repositories & Component Catalog (108 Repositories)]]**: Full architecture, role, and upstream inspirations for all 108 modular repositories.

SigmaOS is organized into multiple specialized GitHub repositories for better modularity and maintainability:

- **[[SigmaOS|https://github.com/AaryanSinghChauhan09/SigmaOS]]**: Main microkernel OS - Core kernel, shell, drivers, and system utilities
- **[[SigmaOS-kernel|https://github.com/AaryanSinghChauhan09/SigmaOS-kernel]]**: Kernel subsystem - Microkernel core, hardware abstraction, device drivers
- **[[SigmaOS-desktop|https://github.com/AaryanSinghChauhan09/SigmaOS-desktop]]**: Desktop environment - Zenith compositor, theming, window management, shell
- **[[SigmaOS-packages|https://github.com/AaryanSinghChauhan09/SigmaOS-packages]]**: Package manager - ALPM, APT, DNF, SigPkg implementations, repository sync
- **[[SigmaOS-security|https://github.com/AaryanSinghChauhan09/SigmaOS-security]]**: Security subsystem - SELinux, AppArmor, capabilities, pledge/unveil, security hardening
- **[[SigmaOS-system|https://github.com/AaryanSinghChauhan09/SigmaOS-system]]**: System subsystem - Initialization, process management, system utilities
- **[[SigmaOS-network|https://github.com/AaryanSinghChauhan09/SigmaOS-network]]**: Network subsystem - TCP/IP stack, netfilter, network namespaces, BPF/XDP
- **[[SigmaOS-filesystem|https://github.com/AaryanSinghChauhan09/SigmaOS-filesystem]]**: Filesystem subsystem - Ext4, journaling, VFS, storage management
- **[[SigmaOS.wiki|https://github.com/AaryanSinghChauhan09/SigmaOS.wiki]]**: Official GitHub Wiki documentation repository

---

### 📜 Copyright & License
Copyright © 2026 AaryanSinghChauhan09. SigmaOS is released under the MIT License. See [LICENSE](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/LICENSE) for details.
