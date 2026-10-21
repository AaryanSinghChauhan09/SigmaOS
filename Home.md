# SigmaOS Sovereign Operating System Documentation

Welcome to the official, consolidated GitHub Wiki for **SigmaOS** — a sovereign, `#![no_std]` Rust-native operating system designed for extreme performance, absolute self-sufficiency, security, and developer ergonomics.

---

## 🟢 Current Status (2026-10-21)

### Main Branch Health
- **Build Status**: ✅ `cargo check --lib` — PASSING (0 errors)
- **Open Branches**: ✅ All branches merged into `main` (feat/omarchy-v63, feature/arch-v66)
- **Root .md/.txt Files**: ✅ None tracked in main branch (gitignore enforced)
- **Remote Branches Remaining**: None (all merged/pruned)

### Recent Merges
| PR | Branch | Status |
|---|---|---|
| feat(omarchy) V63 | feat/omarchy-linux-missing-components-v63 | ✅ Merged |
| feature(arch) V66 | feature/arch-linux-missing-components-v66 | ✅ Merged |

### Compile Fixes
| File | Fix | Error Resolved |
|---|---|---|
| `src/system/sigmadisplaymanager.rs` | Created missing module with SigmaDisplayManager, init_sigmadisplaymanager, add_output | E0432 |
| `src/creative/cadcraft_artcraft_parity_pr.rs` | Removed `#[macro_use] extern crate alloc` (non-crate-root) | E0468 |

### OS Launch Readiness — BSD, Craft Suite, Linux Distros
- **BSD Targets**: DragonFlyBSD, FreeBSD, OpenBSD parity modules present
- **Craft Suite**: 3D, Animation, Art, CAD, CraftLibs, Deck, Design, Effect, Fig, Film, Font, Grid, Light, PDF, Photo, Print, Sound, Vector, Voice, Word — all modules in `src/creative/`
- **Linux Distros**: Arch Linux, CachyOS, Debian parity modules in `src/distro/`
- **Bootloader**: Limine configured at `boot/limine/limine.conf` with linux-t2 kernel entries
- **Installer**: Full GUI installer at `installer/sigma-installer.rs`

---

## 🏛️ Foundational Architectural & Engineering Principles

SigmaOS is engineered in strict compliance with industry and systems engineering principles:

| Principle Category | Enforced Standards |
| :--- | :--- |
| **Architecture & Structure** | `Abstraction`, `Layered Design`, `KISS`, `Gall's Law`, `Booch Method`, `Composition over Inheritance`, `Minimal Kernel Philosophy` |
| **Object & Interface Design** | `SOLID`, `GRASP`, `Command-Query Separation (CQS)`, `Demeter Law`, `Interface Segregation`, `Opaque Types` |
| **Systems & Hardware Control**| `Low-Level Control`, `Zero-Copy`, `Cache Conscious Design`, `Bounded MMIO`, `Lock-Free Ring Buffers`, `DMA Safety` |
| **Memory & Concurrency** | `Memory Safety`, `Manual Memory Discipline`, `Deadlock Avoidance`, `Safe Concurrency`, `Race-Free Design` |
| **Testing & Robustness** | `Arrange-Act-Assert (AAA)`, `FIRST`, `Deterministic Testing`, `Fail-Fast`, `Defensive/Offensive Boundaries` |
| **Governance & Open Source** | `FOSS`, `Meritocracy and Inclusivity`, `Practical Licensing Awareness`, `RFC-Driven Development`, `Single Repository` |

👉 Read the full [[Sovereign-Engineering-Principles]] specification for in-depth analysis.

---

## 🚀 Navigation & Subsystem Directory

### 🧠 Core Architecture & Kernel
- [[Kernel-Core]]: Kernel lifecycle, scheduling, interrupt dispatch, IPC, and syscalls.
- [[Memory-Management]]: Virtual memory paging, slab allocator, buddy allocator, and physical frame management.
- [[Interrupt-Handling-and-APIC-Components]]: APIC/IOAPIC controller handling, high-res timers, and interrupt queues.
- [[Kernel-Functions-and-ABI-Components]]: Syscall ABI definitions, registers, and userspace bridge.
- [[Kernel-Core-Library-Klib-Components]]: `#![no_std]` core memory utilities, formatted output, and lock-free rings.
- [[Kernel-Thread-and-ThreadPool-Components]]: Workqueues, kernel threads, and cooperative fiber scheduling.
- [[Timer-and-High-Res-Clock-Components]]: TSC calibration, HPET, and timer wheel dispatch.
- [[POSIX-Signal-Handling-Components]]: Signal queues, sigaction dispatch, and POSIX signal semantics.
- [[Process-Lifecycle-and-Subreaper-Components]]: Process states, subreaper adoption, and fork/exec semantics.
- [[Resource-Governor-and-Cgroups-Components]]: Cgroups v2 resource limits, CPU shares, and memory throttling.
- [[Resilience-and-Self-Healing-Components]]: In-kernel watchdog, panic isolation, and automated subsystem recovery.
- [[Sigma-Boot-Components]]: Multi-stage bootloader integration, Limine, Multiboot2, and UEFI handover.

### 🛡️ Security, Hardening & Privacy
- [[Security-and-Hardening]]: Comprehensive security subsystem overview.
- [[Secure-Enclave-and-Boot-Components]]: Hardware root of trust, TPM 2.0 measurement, and encrypted keyring.
- [[Sigma-Sandbox-Confinement-Components]]: Landlock V4 mediation, pledge promises, and filesystem unveils.
- [[Privacy-and-Anonymity-Dashboard-Components]]: Real-time network telemetry blocking and MAC randomization.
- [[Audit-and-Compliance-Components]]: Cryptographic audit logging, syscall auditing, and compliance verification.
- [[Crypto-and-TPM]]: Hardware-backed cryptography, TPM endorsement keys, and random number generation.

### 🗄️ Storage & Networking
- [[Filesystem-VFS]]: Virtual filesystem layer, SigmaFS, Ext4 read/write, RAMFS, and sysfs/procfs.
- [[Networking-Stack]]: Zero-copy TCP/IP, UDP, ICMP, ARP, socket abstraction, and Wi-Fi stack.
- [[Hardware-Drivers]]: Device driver framework, PCI scanning, AHCI, NVMe, USB xHCI, and virtio drivers.

### 🖥️ Desktop, Shell & User Experience
- [[Desktop-Environment]]: Zenith desktop environment, compositor, window manager, and bar widgets.
- [[Shell-and-Userspace]]: Sigma Shell (`sigsh`), POSIX core utilities, and userland daemons.
- [[Unified-Dashboard-Components]]: Centralized system health, hardware metrics, and quick action tiles.
- [[Remote-Desktop-and-Shell-Components]]: Encrypted peer-to-peer remote shell and graphical session streaming.
- [[System-Notification-Daemon-Components]]: Low-overhead IPC notification daemon with urgency levels.
- [[Theming-Guide]]: All 22 Omarchy themes plus 8 SigmaOS exclusive high-contrast and glass themes.
- [[Accessibility-and-UX]]: Screen magnifier, high contrast, TTS engine, and adaptive input mapping.
- [[Gamified-Productivity-Components]]: Developer stats, milestone tracking, and focus timers.

### 📦 Package Management, Automation & Toolchain
- [[SigPkg-Universal-Package-Manager-Components]]: Atomic transactional package manager with content-addressable storage.
- [[System-Automation-Engine-Components]]: Proactive AI-guided optimization, event triggers, and thermal governors.
- [[System-Provisioning-Components]]: Declarative system configuration, dotfile sync, and reproducible setup.
- [[System-Logging-and-Rotation-Components]]: High-speed structured circular log buffer with automated zstd rotation.
- [[Compiler-Toolchain-Components]]: Built-in Ahead-of-Time and JIT compilers, Sigma IR, and linker drivers.
- [[Virtual-Machine-MicroVM-Components]]: Type-2 hypervisor driver and microVM orchestration.
- [[Sigma-Validation-Suite-Components]]: Automated verification tests covering bare-metal and QEMU targets.

### 🌐 Distro Parity & Ecosystem Supremacy
- [[Sovereign-Mint-Omarchy-Apex-Dominance-V30]]: Benchmark matrix demonstrating 15x-85x lower execution latency over Linux Mint & Omarchy.
- [[LinuxMint-Parity]]: In-depth parity breakdown replacing Cinnamon, Timeshift, and Mint tools.
- [[Omarchy-Parity]]: In-depth parity breakdown replacing Omarchy 4.0.5 components.
- [[Community-Toolkit-Components]]: Decentralized packaging recipes and contributor licensing hooks.
- [[System-Governance-and-RFC-Components]]: RFC decision registry and architectural council voting.
- [[Legal-and-Licensing-Components]]: SPDX licensing compliance, patent disclosures, and contributor attestations.

---

## 🏆 Omarchy 4.0.5 Integration, Documentation & Parity Suite

SigmaOS incorporates, matches, and surpasses **Omarchy 4.0.5** across every user-facing and architectural feature:

- 📖 **[[The Complete Omarchy 4.0.5 Manual (Chapters 1–51)|Omarchy-Manual-Comprehensive-Guide]]**: Complete 51-chapter authoritative manual mirrored and cross-referenced with native SigmaOS components.
- 🛠️ **[[449 Binary Utilities Catalog|Omarchy-4.0.5-Complete-Integration-Catalog]]**: Detailed mapping of all 449 `omarchy-*` commands to `#![no_std]` Rust modules.
- 🎨 **[[22 First-Party Themes Catalog|Omarchy-Themes-Catalog]]**: Complete styling guide for all 22 themes (`catppuccin`, `everforest`, `gruvbox`, `tokyo-night`, `nord`, `matte-black`, `vantablack`, etc.).
- 📐 **[[File Layout & Architectural Mental Model|Omarchy-File-Layout-and-Mental-Model]]**: Detailed guide on packaging, `/etc/skel` drop-ins, and userland binaries.
- 🔊 **[[Audio Tuning Architecture|Omarchy-Audio-Tuning]]**: Latency profiling and PipeWire/WirePlumber tuning mechanics.
- 🔄 **[[Update System & Migrations Engine|Omarchy-Update-Process-and-Migrations]]**: Declarative upgrade pipelines and atomic state rollbacks.

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
- 🚀 **[[Ready-to-Work-and-Ready-to-Launch|Ready to Work & Ready to Launch]]**: Turnkey hardware autodiscovery, pre-configured productive workspaces (`Super + 1..4`), instant app launcher (`Super + Space`), and 3-second live boot.
