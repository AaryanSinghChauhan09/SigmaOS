# SigmaOS Wiki

**SigmaOS** is a sovereign, secure, next-generation operating system written in Rust — designed for zero-dependency bare-metal execution with full Linux/BSD distro compatibility.

> ✅ **Build Status:** 0 compilation errors as of September 2026. All 14 open PRs merged.

---

## 🚀 Getting Started

- **Sovereign Microkernel Core:** Memory isolation, CachyOS BORE / EEVDF scheduler, capability bounding sets, and zero-copy IPC channels.
- **Systemd Betsy Init Supervisor:** Full unit parsing, Cgroup v2 slice memory quotas, watchdog health monitoring, and alternative init bridging.
- **GTK & Libadwaita Sovereign UI Toolkit:** `GtkHeaderBar` CSD, `AdwPreferencesPage`, `AdwActionRow`, `GtkCssProvider`, `GtkSignalDispatcher`, status bar panel, dock bar, and workspace overview.
- **Sovereign Network Discovery Engine:** ZeroConf mDNS / DNS-SD, UPnP / SSDP M-SEARCH, LLMNR / NBNS host resolution, and ICMPv6 NDP neighbor table tracking.
- **Interactive `sigma-sh` REPL:** Zsh/Fish syntax-highlighted line editor (`ReplLineEditor`), Fish auto-suggestions (`AutoSuggestTabPopup`), job control (`jobs`/`fg`/`bg`), and OpenBSD pledge/unveil capability sandboxing.
- **Multi-Distro Compatibility & Parity:** Dependency installers and translation adapters for Arch Linux (ALPM/Pacman), Debian/Ubuntu (APT/dpkg), Gentoo (Portage USE flags), Fedora (RPM/SELinux), Linux Mint (Cinnamon, mintupgrade, mintstick, mintmenu), and FreeBSD (Jails/Capsicum/GEOM).
- **Post-Quantum Cryptography:** Native Dilithium-5 and Kyber-1024 cryptographic verification for driver and package attestation.
- **Zero-Trust Access Control & MAC:** Discretionary (DAC), Mandatory Access Control (MAC LSM Inode/Ptrace/Socket hooks), and Role-Based (RBAC) security enforcers.
- **Zenith Desktop & Sovereign Media Suite:** Built-in zero-dependency multimedia tools, video editor (SigmaCut), audio DSP, and responsive UI components.
- **AI Agents Master Guide:** [[AI_AGENTS_GUIDE]] - Authoritative reference for autonomous coding agents and subagents.
- **AI Agents UX Management Guide:** [[AI_AGENTS_UX_MANAGEMENT_GUIDE]] - Interface, visual layout, and UX guidelines for autonomous AI agents.
- **AI Agents Time Management Guide:** [[AI_AGENTS_TIME_MANAGEMENT_GUIDE]] - Timekeeping primitives, clock sync, and temporal architecture for autonomous AI agents.
- **AI Agents Security Management Guide:** [[AI_AGENTS_SECURITY_MANAGEMENT_GUIDE]] - Capability sandboxing, PQC attestation, MAC, and digital forensics for autonomous AI agents.
- **AI Agents Procedure Call Management Guide:** [[AI_AGENTS_PROCEDURE_CALL_MANAGEMENT_GUIDE]] - Syscall dispatchers, FFI bindings, zero-copy IPC ring channels, and RPC for autonomous AI agents.
- **AI Agents Ballooning Management Guide:** [[AI_AGENTS_BALLOONING_MANAGEMENT_GUIDE]] - VirtIO memory ballooning, RAM inflation/deflation, and hypervisor overcommit management for AI agents.
- **AI Agents Boot Management Guide:** [[AI_AGENTS_BOOT_MANAGEMENT_GUIDE]] - UEFI/BIOS handoff, Multiboot2, Secure Boot verification, boot optimization, and init handoff for AI agents.

---

## 🏗️ Architecture & Design

| Page | Description |
|------|-------------|
| [ARCHITECTURE](ARCHITECTURE) | System architecture overview |
| [kernel](kernel) | Kernel design and internals |
| [memory-management](memory-management) | Memory management subsystem |
| [process-management](process-management) | Process and task management |
| [filesystem](filesystem) | Virtual filesystem (VFS) |
| [networking](networking) | Networking stack |
| [drivers](drivers) | Driver model and hardware abstraction |
| [bootloader](bootloader) | Bootloader and early init |
| [shell](shell) | SigmaShell (sigma_sh) |

---

## 📦 Package Management

| Page | Description |
|------|-------------|
| [PACKAGE_MANAGEMENT](PACKAGE_MANAGEMENT) | Complete sigpkg reference |
| [package-manager](package-manager) | Package manager architecture |
| [LINUX_BSD_DISTRO_COMPATIBILITY_GUIDE](LINUX_BSD_DISTRO_COMPATIBILITY_GUIDE) | Linux/BSD distro compatibility |
| [FEDORA_PARITY_FEATURES](FEDORA_PARITY_FEATURES) | Fedora-parity features |

---

## 🔐 Security

| Page | Description |
|------|-------------|
| [SECURITY](SECURITY) | Security model and policies |
| [security](security) | Security subsystem internals |
| [api-reference](api-reference) | Public API reference |

---

## 📋 API & Namespace

| Page | Description |
|------|-------------|
| [API_DOCUMENTATION_v0.9](API_DOCUMENTATION_v0.9) | Full API documentation v0.9 |
| [NAMESPACE_IMPLEMENTATION](NAMESPACE_IMPLEMENTATION) | Namespace implementation details |
| [NAMESPACE_SYSCALLS_API_REFERENCE](NAMESPACE_SYSCALLS_API_REFERENCE) | Namespace syscall API |

---

## 🗺️ Roadmap & Status

| Page | Description |
|------|-------------|
| [ROADMAP](ROADMAP) | Development roadmap |
| [FUTURE-DEVELOPMENT-ROADMAP](FUTURE-DEVELOPMENT-ROADMAP) | Long-term vision |
| [CHANGELOG](CHANGELOG) | Version changelog |
| [RELEASE_NOTES_v0.9](RELEASE_NOTES_v0.9) | v0.9 release notes |
| [WHAT_IS_WORKING_AND_NOT_WORKING](WHAT_IS_WORKING_AND_NOT_WORKING) | Current feature status |
| [NEXT_STEPS_GUIDELINES](NEXT_STEPS_GUIDELINES) | Contributor guidelines for next steps |
| [TIER1_FEATURES](TIER1_FEATURES) | Tier-1 feature tracking |

---

## 🌐 Linux Distro Innovations

| Page | Description |
|------|-------------|
| [SigmaOS-vs-Linux-Distros-Comparative-Dashboard](SigmaOS-vs-Linux-Distros-Comparative-Dashboard) | Feature parity dashboard vs major distros |
| [SigmaOS_Gap_Closing_Roadmap](SigmaOS_Gap_Closing_Roadmap) | Gap-closing roadmap vs Linux |
| [LINUX_BSD_INNOVATIONS_IMPLEMENTED](LINUX_BSD_INNOVATIONS_IMPLEMENTED) | Implemented Linux/BSD innovations |
| [Operations-and-Continuous-Improvement-Guide](Operations-and-Continuous-Improvement-Guide) | Ops and CI guide |
| [SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V19](SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V19) | Sovereign OS Encyclopedia V19 |

---

## 💡 Ideas & Planning

| Page | Description |
|------|-------------|
| [100-Improvement-Ideas](100-Improvement-Ideas) | 100 improvement ideas |
| [ImprovementPlan](ImprovementPlan) | Improvement plan |
| [DETAILED_IMPROVEMENT_PLAN](DETAILED_IMPROVEMENT_PLAN) | Detailed improvement plan |
| [SIGMAOS_500_REPOS_TRI_AGENT_ABSORPTION_AND_IMPLEMENTATION_PLAN](SIGMAOS_500_REPOS_TRI_AGENT_ABSORPTION_AND_IMPLEMENTATION_PLAN) | 500-repo absorption plan |

---

*Last updated: September 2026 — All PRs merged, 0 compilation errors.*
