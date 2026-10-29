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

## 🌐 Universal Operating System & Subsystem Synthesis

SigmaOS enables users to natively experience the workflows, guarantees, and application paradigms of every major operating system, runtime, and productivity suite:

### 🧩 1. Multi-Kernel Synthesis (`src/kernel/`)
- 🐧 **Linux**: Monolithic multi-core performance, eBPF JIT filter pipelines, cgroups v2, and io_uring async I/O.
- 🦬 **GNU Hurd**: Capability-based Mach IPC message ports (`mach_port.rs`) and userspace filesystem translators (`hurd_translator.rs`).
- 🔄 **Minix 3**: Microkernel multi-server architecture with Reincarnation Server self-healing crashed device drivers (`minix_reincarnation.rs`).
- 🪟 **ReactOS / Windows NT**: Executive Object Manager namespace (`nt_object_manager.rs`) and PE32+ executable binary loader (`pe_loader.rs`).
- 🍏 **Apple XNU**: Mach core messaging + I/O Kit object-oriented C++ driver hierarchy translated to safe Rust (`xnu_iokit.rs`).
- 📖 Read more: [[Kernel-Architectures-Synthesis]]

### 💾 2. DOS & Retro Computing Runtime (`src/kernel/dos/`)
- 🕹️ **FreeDOS & MS-DOS**: Clean-room INT 21h service dispatcher (`int21h.rs`) with AH register function multiplexing.
- ⚡ **DPMI 0.9+ Host**: Dynamic Local Descriptor Table (LDT) segment allocation and real-to-protected mode switching (`dpmi.rs`).
- 📜 **BASIC Language Engine**: AST-based interpreter supporting Dartmouth, GW-BASIC, and QBASIC command syntax (`basic_engine.rs`).
- 🖥️ **NTVDM Subsystem**: NT Virtual DOS Machine 16-to-32 bit thunking tables and console buffer virtualization (`ntvdm.rs`).
- ⚙️ **Virtual 8086 Machine**: In-kernel real-mode execution with 256 IVT vectors and I/O port trapping (`vdm.rs`).
- 📖 Read more: [[DOS-Runtime-Engine]] & [[DOS-and-Legacy-Compatibility]]

### 🔱 3. Extended BSD Distributions (`src/compatibility/`)
- 👻 **GhostBSD**: OpenRC init service management, MATE session integration, and NetworkManager bridge (`ghost_bsd.rs`).
- 🧭 **NomadBSD**: Fast hardware autodetect and layered live USB persistence (`nomad_bsd.rs`).
- 🌙 **MidnightBSD**: `mports` package indexing and GNUstep desktop environment bridge (`midnight_bsd.rs`).
- 🛡️ **HardenedBSD**: SafeStack isolation, forward/backward CFI enforcement, ASLR entropy, and strict W^X PTE validation (`hardened_bsd.rs`).
- 🕊️ **LibertyBSD**: Cryptographic SHA-256 firmware audit rejecting proprietary microcode blobs (`liberty_bsd.rs`).
- 🌊 **FuryBSD**: ZFS Boot Environment (`beadm`/`bectl`) snapshot, clone, and rollback manager (`fury_bsd.rs`).
- 💻 **DesktopBSD**: Polkit user authorization delegation and pkgng GUI installation bridges (`desktop_bsd.rs`).
- 👁️ **KarmaBSD**: DTrace dynamic tracepoint providers and ZFS latency observability (`karma_bsd.rs`).
- ⭐ **StarBSD**: Ultra-lightweight sub-10MB RTOS persona with static memory guard (`star_bsd.rs`).
- 📖 Read more: [[Extended-BSD-Parity]]

### 🐧 4. Extended Linux Distributions (`src/distro/`)
- 🍬 **ElementaryOS**: Pantheon desktop shell, Granite widget bridge, Gala window manager, and AppCenter (`elementary_os.rs`).
- 🚀 **EndeavourOS**: Arch-based minimal base with dynamic desktop selection and welcoming assistant (`endeavour_os.rs`).
- 🐉 **Kali Linux**: Penetration testing tool registry, Undercover Windows camouflage mode, and SDR tools (`kali_linux.rs`).
- 🦜 **Parrot OS**: AnonSurf system-wide Tor router, forensic disk imager, and decoy session mode (`parrot_os.rs`).
- ⬛ **Qubes OS**: Security domain management (Vault, Work, Personal, Untrusted) with disposable VMs and colored borders (`qubes_os.rs`).
- 🔬 **Redox OS**: URL scheme-based microkernel IPC (`file:`, `tcp:`) and Ion shell variable expansion (`redox_os.rs`).
- 📦 **Slackware**: BSD-style rc init runlevels and `.txz` Slackware pkgtool package manager (`slackware.rs`).
- 🦎 **openSUSE**: Zypper SAT dependency solver, Snapper pre/post Btrfs snapshots, and YaST2 configuration (`open_suse.rs`).
- 🟠 **Ubuntu**: Netplan network configuration generation, Snap strictly-confined sandbox, and Ubuntu Pro hooks (`ubuntu.rs`).
- 📖 Read more: [[Linux-Universal-Distro-Parity]]

### 📑 5. Complete Office Productivity Suites (`src/productivity/`)
- ✍️ **Writer**: ODF/DOCX bidirectional import/export, styles catalog, and tracked changes (`libreoffice_parity.rs`).
- 📊 **Calc**: ODS/XLSX formula DAG, cell evaluation, pivot tables, and charting.
- 📽️ **Impress**: ODP/PPTX slide deck manager, hardware slide transitions, and presenter view.
- 📐 **Draw**: Vector graphics canvas, Bézier shape tools, and SVG/ODG export.
- ➗ **Math**: Formula editor rendering LaTeX, MathML, OpenFormula, and TeX mathematical typography.
- 🗃️ **Base**: Database frontend with form designers, query designers, and SQL reporting workbenches.
- 🏛️ **Apache OpenOffice Parity**: Binary and XML parity with legacy StarOffice (`.sxw`, `.sxc`, `.sxi`, `.sxd`) compatibility (`openoffice_parity.rs`).
- 📖 Read more: [[Office-and-Productivity-Suites]]

### 🗄️ 6. Relational Database & Analytics Engines (`src/database/`)
- 🐘 **PostgreSQL**: MVCC ACID transaction engine, B-Tree indices, isolation levels, and SQL parser (`postgres.rs`).
- 🦆 **DuckDB**: Vectorized columnar in-process OLAP engine with predicate filter pushdown and aggregation (`duckdb.rs`).
- 🏹 **Apache Arrow & DataFusion**: Zero-copy in-memory columnar array buffers, IPC streaming, and query planner (`arrow_datafusion.rs`).
- 🦫 **DBeaver**: Universal database workbench connection manager, multi-format exporter, and ER diagrams (`dbeaver.rs`).
- 📊 **Apache Superset**: Enterprise BI visualization slice and dashboard catalog (`superset.rs`).
- ❓ **Metabase**: Self-service question builder and natural-language to SQL translation parser (`metabase.rs`).
- 📖 Read more: [[Database-and-Data-Engineering]]

### 🛠️ 7. Developer & Autonomous AI Tools (`src/tools/` & `src/ai/`)
- 🎨 **Blender Pipeline**: Headless 3D render pipeline with Cycles and Eevee material graph evaluation (`blender_pipeline.rs`).
- 🐳 **Docker OCI Runtime**: Rootless container runtime (`sigma-runc`), layer unpacker, and Compose parser (`docker_oci.rs`).
- 🎥 **VLC Pipeline**: Universal media container demuxer, VA-API hardware acceleration, and Wayland video presentation (`vlc_pipeline.rs`).
- 📝 **NotepadSigma**: Instant startup (<5ms) Scintilla-grade multi-cursor text editor with 20+ syntax lexers (`notepad_sigma.rs`).
- ⚡ **Warp Terminal**: GPU block-based terminal session manager with AI command recommendations (`warp_terminal.rs`).
- 🤖 **Autonomous AI Coding Agents**:
  - **Cline**: Dual-stage planning and execution agent with safety checks and diff previews (`cline_agent.rs`).
  - **CodexCLI**: Natural language to shell command generator with sandboxed execution (`codex_cli.rs`).
  - **CrewAI**: Multi-role autonomous agent team coordinator (Developer, QA, Security, PM) (`crewai_orchestrator.rs`).
  - **DeepSeek Coder**: Local GGUF quantized model runner with Fill-in-the-Middle and code review (`deepseek_coder.rs`).
  - **Hermes Agent**: Autonomous system debugger with kernel panic triage and gated self-healing fixes (`hermes_agent.rs`).
  - **LangChain**: Vector index RAG pipeline, tool registration, and conversational memory buffers (`langchain_pipeline.rs`).
- 📖 Read more: [[Developer-Tools-and-AI-Agents]]
