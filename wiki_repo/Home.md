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

- **[Sovereign OS Absolute Omnipresent Self-Sufficiency Ultra Encyclopedia V31](SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V31)**
  Comprehensive architectural reference detailing zero-dependency replacements for VLC, LibreOffice, GIMP, Audacity, Firefox, Blender, PyTorch, Llama, MySQL, PostgreSQL, Wireshark, Docker, and VirtualBox.

---

## 🏁 Getting Started & Installation

- **[About SigmaOS](About-SigmaOS)** — Overview of SigmaOS system architecture and non-Linux sovereign kernel design.
- **[Installation Guide](Installation-Guide)** — Step-by-step guide to installing SigmaOS via Rust Hash-Sum verification.
- **[General Recommendations](General-Recommendations)** — Post-installation configuration, systemd/openrc service supervision, and tuning.
- **[System Architecture](ARCHITECTURE)** — 12 System Shards, multi-architecture HAL, and zero-dependency driver model.

---

**[Browse All Wiki Articles](Table-of-contents)**
