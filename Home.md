# ⚡ SigmaOS v15.0 Zenith — Wiki Home

> **The Sovereign Lattice Operating System. 600 shards. PQC-hardened. Industrial-grade.**

[![GitHub](https://img.shields.io/badge/GitHub-SigmaOS-black?logo=github)](https://github.com/AaryanSinghChauhan09/SigmaOS)
[![Latest Release](https://img.shields.io/badge/latest-v15.0.0--Stable-brightgreen)](https://github.com/AaryanSinghChauhan09/SigmaOS/releases)
[![License](https://img.shields.io/badge/license-Sovereign--MIT-purple)](https://github.com/AaryanSinghChauhan09/SigmaOS)
[![Architecture](https://img.shields.io/badge/arch-x86__64%20%7C%20ARM64%20%7C%20RISC--V-green)](https://github.com/AaryanSinghChauhan09/SigmaOS)

---

## 🏗️ Unified Strategy (v15.0+)
SigmaOS v15.0 follows a **Unified Development Strategy** to ensure functional, algorithmic, and design parity across all 7 formats.
*   [**Unified Development Strategy**](Architecture#5-unified-development-strategy-v150) — Core architecture overview.
*   [**Core Toolset Manifest**](file:///c:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS/docs/architecture/CORE_TOOLSET.md) — Baseline tools in every edition.
*   [**Unified Task Manifest**](file:///c:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS/docs/UNIFIED_TASK_MANIFEST.md) — Future and incomplete work tracker.

---

SigmaOS v15.0 ships in **7 distinct editions**, each purpose-built for a specific deployment scenario. Select the edition that matches your hardware and use case:

---

### 🖥️ [Zenith Standalone](Home-Zenith-Standalone)

**The flagship bare-metal edition.** Full sovereign ownership of your hardware. No hypervisor, no Windows, no Linux substrate.

- ✅ Complete Zenith Desktop (Z-DESKTOP)
- ✅ All 600 shards
- ✅ PQC-hardened (Dilithium-5 + Kyber-1024)
- ✅ <2ms boot with SSB fast-boot mode
- **Best for**: Power users, professionals, dedicated workstations

**Release**: [v15.0-zenith-standalone](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-standalone)

---

### ⚡ [Zenith Dual-Boot](Home-Zenith-Dualboot)

**Coexistence with Windows or Linux.** Install alongside your existing OS — safely, reversibly, and without touching foreign partitions.

- ✅ GRUB2 sovereign boot manager
- ✅ GPT-safe installer
- ✅ Windows 10/11 & Linux compatible
- ✅ Cross-OS file sharing
- **Best for**: Users transitioning to SigmaOS, professionals who need both worlds

**Release**: [v15.0-zenith-dualboot](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-dualboot)

---

### 🔬 [Zenith Core](Home-Zenith-Core)

**Headless microkernel — no GUI.** Pure kernel power for servers, CI/CD nodes, embedded systems, and kernel developers.

- ✅ 2 MB kernel image
- ✅ <100ms headless boot
- ✅ Full kernel ABI + 48 custom syscalls
- ✅ PXE, container, and VM deployment
- **Best for**: Servers, embedded systems, kernel researchers, CI/CD infrastructure

**Release**: [v15.0-zenith-core](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-core)

---

### 🌐 [Zenith Browser](Home-Zenith-Browser)

**Privacy-first, web-optimized OS.** Purpose-built for sovereign browsing with zero tracking, built-in PQC VPN, and 3-second boot to browser.

- ✅ SovereignBrowser (PQC-hardened TLS)
- ✅ Built-in S-VPN + SovereignBlock ad blocker
- ✅ Zero telemetry — ever
- ✅ <3 second boot to browser
- **Best for**: Privacy advocates, kiosk stations, Chromebook replacements, secure browsing

**Release**: [v15.0-zenith-browser](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-browser)

---

### 📱 [Zenith App](Home-Zenith-App)

**Universal application platform.** Run Linux, Windows (S-Wine), Android (S-ARC), and WebAssembly apps alongside native sovereign shards.

- ✅ Linux ELF + Windows S-Wine + Android S-ARC
- ✅ Sovereign App Nexus (50,000+ apps)
- ✅ Per-app S-ARMOR sandboxing
- ✅ Built-in developer tools + SovereignIDE
- **Best for**: Developers, creative professionals, enterprise workstations

**Release**: [v15.0-zenith-app](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0-zenith-app)

---

### 🏔️ [Stable (LTS)](Home-Stable)

**Production-hardened Long-Term Support.** 3 years of security backports, formal verification, enterprise directory integration, and fleet management.

- ✅ 3-year LTS security backports
- ✅ LDAP/Active Directory integration
- ✅ FIPS 140-3 aligned cryptography
- ✅ Atomic updates with rollback
- **Best for**: Enterprises, governments, critical infrastructure, production deployments

**Release**: [v15.0.0-Stable](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0.0-Stable)

---

### 🌅 [Horizon (Research Preview)](Home-Horizon)

**Bleeding-edge research edition.** Neural scheduler, quantum HAL, holographic UI, AI-native IPC, and Rust kernel drivers — the future of SigmaOS.

- 🔬 Neural AI Scheduler (ML-driven)
- 🔮 Quantum HAL (QPU integration)
- 🌌 Holographic UI (XR support)
- 🦀 Rust Kernel Driver Framework
- **Best for**: Kernel researchers, AI engineers, quantum computing pioneers, early adopters

> ⚠️ Not for production — research preview

**Release**: [v15.0.0-Horizon](https://github.com/AaryanSinghChauhan09/SigmaOS/releases/tag/v15.0.0-Horizon)

---

## 📊 Edition Comparison

| Feature | Standalone | Dual-Boot | Core | Browser | App | Stable | Horizon |
|---|---|---|---|---|---|---|---|
| Desktop | ✅ Full | ✅ Full | ❌ CLI | ✅ Minimal | ✅ Full | ✅ Full | ✅ Holo |
| PQC Security | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Boot Time | <2s | <5s | <100ms | <3s | <5s | <8s | <10s |
| RAM (idle) | ~512MB | ~512MB | ~128MB | ~256MB | ~800MB | ~512MB | ~2GB |
| Windows Apps | ❌ | ❌ | ❌ | ❌ | ✅ S-Wine | ❌ | ✅ |
| Android Apps | ❌ | ❌ | ❌ | ❌ | ✅ S-ARC | ❌ | ✅ |
| Neural Sched | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Quantum HAL | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| LTS Support | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ 3yr | ❌ |
| Production | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ⚠️ Beta |

---

## 🏛️ Core Documentation

| Document | Description |
|---|---|
| [Architecture](Architecture) | Sovereign Lattice Microkernel architecture overview |
| [Kernel-Developer-Handbook](Kernel-Developer-Handbook) | Deep-dive kernel development reference |
| [SYSCALLS](SYSCALLS) | Full sovereign syscall table reference |
| [Installation](Installation) | General installation guide |
| [Security-Safety](Security-Safety) | PQC security architecture |
| [Performance-Benchmarks](Performance-Benchmarks) | Benchmarks vs Linux/Windows |
| [Driver-Framework](Driver-Framework) | Sovereign Driver Framework (SDF) |
| [Sovereign-Lattice-Filesystem](Sovereign-Lattice-Filesystem) | SLF distributed filesystem |
| [Sovereign-Industrial-Scheduler](Sovereign-Industrial-Scheduler) | S-CFS scheduler internals |
| [Sovereign-Memory-Management](Sovereign-Memory-Management) | Memory management deep-dive |
| [API-Reference](API-Reference) | Sovereign API reference |
| [CONTRIBUTING](CONTRIBUTING) | Contribution guide |
| [SECURITY](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/SECURITY.md) | Security policy & CVE reporting |

---

## 🚀 Quick Start

**Not sure which edition to choose?**

```
Are you replacing Windows/Linux entirely?     → Standalone
Running alongside Windows or Linux?           → Dual-Boot
Building a server or embedded system?         → Core
Need a privacy-first browser experience?      → Browser
Need to run apps from multiple ecosystems?    → App
Deploying in enterprise / production?         → Stable
Researching AI/quantum/kernel internals?      → Horizon
```

---

## 🧬 What is SigmaOS?

SigmaOS is a **ground-up sovereign operating system** built on the Sovereign Lattice Microkernel. It is:

- **Not a Linux fork** — sovereign kernel written in C++20/C11, not derived from Linux
- **Not a Windows clone** — entirely independent architecture and ABI
- **Post-quantum secured** — Dilithium-5 signatures and Kyber-1024 key exchange throughout
- **Modular** — 600 professional shards loadable/unloadable at runtime
- **Industrial-grade** — Designed for bare-metal performance, not VM emulation

### Architecture Snapshot

```
┌─────────────────────────────────────────────────────────┐
│                     USERLAND LAYER                       │
│  Z-DESKTOP  │  OmniShell  │  sigma-pkg  │  App Store    │
├─────────────────────────────────────────────────────────┤
│                  SHARD LATTICE (600 shards)              │
│  S-NET  │  S-VFS  │  S-SCHED  │  S-SEC  │  S-GPU  ...  │
├─────────────────────────────────────────────────────────┤
│              SOVEREIGN LATTICE MICROKERNEL               │
│    Syscall Table │ S-ARMOR │ IMA │ PQC Daemon           │
├─────────────────────────────────────────────────────────┤
│                      HAL LAYER                           │
│    CPU │ Memory │ Serial │ PCI │ USB │ NVMe │ GPU        │
├─────────────────────────────────────────────────────────┤
│                   BARE METAL / SILICON                   │
└─────────────────────────────────────────────────────────┘
```

---

## 🔗 Links

- **GitHub Repository**: [AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
- **All Releases**: [Releases Page](https://github.com/AaryanSinghChauhan09/SigmaOS/releases)
- **Issue Tracker**: [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- **Discussions**: [GitHub Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
- **Security Advisories**: [Security](https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories)

---

*SigmaOS v15.0 Zenith — Sovereign computing, redefined from silicon up.*
