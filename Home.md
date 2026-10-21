# SigmaOS Sovereign Operating System Documentation

Welcome to the official, consolidated GitHub Wiki for **SigmaOS** — a sovereign, `#![no_std]` Rust-native operating system designed for extreme performance, absolute self-sufficiency, security, and developer ergonomics.

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
