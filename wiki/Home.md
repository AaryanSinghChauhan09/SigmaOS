# Welcome to the SigmaOS Wiki & Central Portal

**The Sovereign, Zero-Dependency, Quantum-Safe Operating System Portal**

SigmaOS is a high-performance, memory-safe, non-Linux/non-POSIX sovereign operating system engineered in 100% Safe Rust (`klib`). It combines atomic generation updates, capability-based sandboxing, zero-copy networking, hardware-accelerated AI execution, and universal Linux & BSD distro subsystem parity.

---

## ⚡ Quick Navigation Grid

| Pillar | Documentation & Architecture | Key Features |
| :--- | :--- | :--- |
| 🛡️ **Security & Hardening** | [Security Model](Security-Sandboxing-and-Hardening) • [Zorin Exec Guard](Security-Sandboxing-and-Hardening#zorin-exec-guard) | OpenBSD `pledge`/`unveil`, FreeBSD `Capsicum`, Landlock, PQC Dilithium5/Kyber1024 |
| 🎨 **Zenith & Desktop** | [Zenith Compositor](Zenith-Compositor) • [Cinnamon DE](Category-Desktop-Environment) | Wayland native, Muffin WM snapping, Bolt launcher, Palette dynamic themes |
| 📦 **Universal Packaging** | [SigmaPkg Engine](Package-Management-and-Sigpkg) • [Content-Addressed Store](Content-Addressed-Storage) | Arch PKGBUILD, Debian `.deb`, Fedora `.rpm`, Alpine `.apk`, Gentoo Portage, Nix Flakes |
| 🚀 **Kernel & Performance** | [BORE & EEVDF Scheduler](Performance-Tuning-and-Kernel) • [Demand Swap](Demand-Paging-and-Swap) | Zero-copy XDP, MGLRU, io_uring, ZRAM compressed swap, eBPF JIT compiler |
| 🎮 **Gaming & Graphics** | [Gamescope Microcompositor](Category-Desktop-Environment) • [Proton DXVK](List-of-Applications) | FSR upscaling, MangoHud HUD telemetry, GameMode governor, EAC/BattlEye shims |
| 🤖 **Autonomous AI** | [System AI Engine](AI_DEVELOPER_PLATFORM) • [Agentic OS](Category:Development) | eBPF GPU governor, Capsicum sandboxed LLMs, PII prompt scrubbing, ZK vector vaults |

---

## 🏆 Distro Parity & Superiority Dashboard

SigmaOS natively absorbs and surpasses the core innovations of all major Linux and BSD distributions:

| Distribution | Absorbed Feature / Subsystem | SigmaOS Native Replacement | Parity Status |
| :--- | :--- | :--- | :---: |
| **Arch Linux** | `pacman`, AUR PKGBUILD, `vercmp`, `arch-news` | Native `sigpkg` ALPM engine & AUR linter | 100% |
| **Linux Mint** | Cinnamon Desktop, Muffin WM, Nemo Dual-Pane | Native `CinnamonMuffinWindowManager` & `Nemo` | 100% |
| **Fedora Linux** | `dnf`/RPM, Silverblue OSTree, systemd-oomd | Atomic generation updates & `FedoraHyperreadinessEngine` | 100% |
| **Gentoo Linux** | Portage, ebuild Manifests, USE flags | `GentooUseFlagResolverEngine` & EAPI-8 slots | 100% |
| **NixOS / Guix** | Flakes, content-addressed store `/nix/store` | Content-Addressed Store `/sigma/store/` | 100% |
| **FreeBSD** | Bhyve hypervisor, Capsicum, `sysctl.conf`, GEOM | `FreeBsdBhyveVirtioEngine` & Capsicum sandboxing | 100% |
| **OpenBSD** | `vmm(4)`, `pledge(2)`, `unveil(2)`, `signify(1)`, `doas` | Native `OpenBsdVmmMicroHypervisorEngine` & `doas` | 100% |
| **CachyOS** | eBPF `sched_ext`, ZRAM zstd compression | `CachyOsSchedExtFramework` & `ZramSwapEngine` | 100% |

---

## 🤖 AI Agent & Developer Component Guidance

- **[AI Agent SigmaOS Component Development Guidance](AI_AGENT_SIGMAOS_COMPONENT_DEVELOPMENT_GUIDANCE)** — Master engineering standards and step-by-step procedures for AI Agents developing kernel, driver, desktop, package manager, and security components inspired by Linux and BSD distributions.

---

## 📚 Omnipresent Self-Sufficiency Index

SigmaOS eliminates external application dependencies across 450+ tools, frameworks, codecs, databases, and AI models through native `klib` Safe-Rust implementations:

- **[Privilege Delegation (doas)](SECURITY)**
  Secure privilege escalation and delegation.

- **[Networking & WireGuard](networking)**
  Network configuration and WireGuard VPN integration.

- **[Logging & Journalctl](syslog)**
  System logging and journal management.

- **[Power Management](Power-Management)**
  CPU frequency scaling, device power states, battery management, and thermal control.

- **[Real-Time Computing](Real-Time-Computing)**
  Real-time scheduling, preemptible kernel, and deterministic latency.

- **[Virtualization and Containers](Virtualization-and-Containers)**
  Hardware virtualization, container technology, and orchestration.

- **[System Call Interface](System-Call-Interface)**
  Comprehensive system call interface and security features.

- **[Inter-Process Communication](Inter-Process-Communication)**
  Pipes, message queues, shared memory, and synchronization.

---

## Security & Hardening

- **[Security & Sandboxing](Security-Sandboxing-and-Hardening)**
  Comprehensive security model and sandboxing capabilities.

- **[Zorin Exec Guard](Security-Sandboxing-and-Hardening#zorin-exec-guard)**
  Default-deny capability permission model.

- **[Hardware Enclaves & PQC](Security-Sandboxing-and-Hardening#hardware-enclaves--post-quantum-cryptography)**
  Hardware enclaves and post-quantum cryptography support.

- **[Memory Safety Guarantees](SECURITY)**
  Rust-enforced memory safety and kernel hardening features.

- **[Category:Security](Category:Security)**
  Security and hardening articles.

---

## Performance & Kernel

- **[Performance & BORE Scheduler](Performance-Tuning-and-Kernel)**
  Performance tuning and BORE scheduler configuration.

- **[Low-Overhead Kernel Observability](Low-Overhead-Kernel-Observability)**
  High-performance kernel tracing with SigmaTrace and eBPF.

- **[MGLRU & Memory Reclamation](Performance-Tuning-and-Kernel#mglru--memory-reclamation)**
  Multi-Gen LRU and memory reclamation optimization.

- **[eBPF / XDP Networking](Performance-Tuning-and-Kernel#ebpf--xdp-zero-copy-networking)**
  eBPF and XDP zero-copy networking support.

- **[Lock-Free Data Structures](SIGMA_CONCURRENCY_PRIMITIVES)**
  Lock-free concurrency primitives and data structures.

- **[Category:Performance](Category-Performance)**
  Performance and kernel articles.

---

## Maintenance & Recovery

- **[Rollback Engine & Snapshots](Maintenance-and-Rollback-Engine)**
  Atomic rollback engine and system snapshots.

- **[Declarative Generations](Maintenance-and-Rollback-Engine#declarative-system-generations)**
  Declarative system state and generation management.

- **[ZFS Boot Environments](freebsd-zfs)**
  ZFS boot environment management.

- **[Btrfs Subvolumes](SIGMA_VFS_LAYER)**
  Btrfs subvolume management and CoW filesystems.

---

## Hardware & Platform

- **[Hardware Support Matrix](SUPPORT_MATRIX)**
  Comprehensive hardware compatibility and support matrix.

- **[Multi-Architecture Support](ARCHITECTURE)**
  Support for x86_64, AArch64, and other architectures.

- **[Virtualization & Containers](Virtualization-and-Containers-Isolation)**
  Virtualization and container isolation features.

- **[Device Drivers](Device-Drivers)**
  Device driver support and management.

- **[Category:Hardware](Category:Hardware)**
  Hardware and platform articles.

---

## Our Community

- **[Code of Conduct](Code-of-Conduct)**
  Guidelines for the SigmaOS community.

- **[Getting Involved](Getting-Involved)**
  Describes various ways contributors can participate in the SigmaOS community.

- **[Contributing](Contributing)**
  Guide to contributing to SigmaOS development and documentation.

- **[International Communities](International-Communities)**
  Collection of links to SigmaOS communities around the world.

- **[Category:Community](Category-Community)**
  Community articles and guidelines.

---

## Wiki Interaction

- **[Help:Reading](Help-Reading)**
  Find clarifications if you struggle to understand instructions in some articles.

- **[Help:Browsing](Help-Browsing)**
  How to search the wiki, find related articles and view the wiki offline.

- **[Wiki:Contributing](Wiki-Contributing)**
  The starting point for those willing to contribute to the wiki.

- **[Help:Editing](Help-Editing)**
  Tutorial on editing articles and introduction to wiki text syntax.

---

## Reference Documentation

- **[Product Vision & Manifesto](PRODUCT_VISION)**
  Strategic vision and product manifesto.

- **[Release Criteria](RELEASE_CRITERIA)**
  Quality gates and release criteria.

- **[Architecture Decision Records](ARCHITECTURE_DECISIONS)**
  Key architectural decisions and their rationale.

- **[Comprehensive OS Analysis](SIGMAOS_COMPREHENSIVE_OS_ANALYSIS)**
  Consolidated analysis of SigmaOS architecture and comparisons.

- **[Category:Development](Category-Development)**
  Development and technical articles.

---

## Gap Closure Roadmap

- **[Phase 1: Critical Foundation](Phase-1-Gap-Closure-Implementation-Plan)**
  PCI/PCIe enumeration, GPU drivers, demand paging, pledge/unveil, eBPF JIT (Months 1-3)

- **[Phase 2: Core Features](Phase-2-Gap-Closure-Implementation-Plan)**
  Kernel modules, IRQ balancing, Cgroups v2, io_uring, Capsicum (Months 4-6)

- **[Phase 3: Advanced Features](Phase-3-Gap-Closure-Implementation-Plan)**
  ZFS, Btrfs, XDP, PF firewall, Nix/Guix builds (Months 7-12)

- **[Phase 4: Enterprise Features](Phase-4-Gap-Closure-Implementation-Plan)**
  NVIDIA GPU, Wi-Fi 6E/7, USB3/4, Mach/Zircon IPC, Portage (Months 13-18)

---

**[View all wiki pages](Table-of-contents)**

---

## Omnipresent Self-Sufficiency Reference
- **[Sovereign OS Absolute Omnipresent Self-Sufficiency Ultra Encyclopedia V34](SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V34)**
  Master architectural reference detailing native zero-dependency replacements for external software, AI models, frameworks, database engines, robotics tools, codecs, and operating systems.
