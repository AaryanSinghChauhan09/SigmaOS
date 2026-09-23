# AI Agent SigmaOS Component Development Master Guide & Directives

**The Sovereign Operating System AI Engineering Specification**

---

## Executive Summary & Master Purpose

This document serves as the canonical **Standard Operating Procedure (SOP) and Engineering Specification** for Autonomous AI Development Agents (including Jules, Bolt ⚡, Palette 🎨, Sentinel 🛡️, and subagent specialists) building, refactoring, testing, and maintaining components of **SigmaOS**.

SigmaOS is a secure, fast, bare-metal, zero-dependency operating system written in safe Rust (`#![no_std]`). To systematically absorb the pinnacle features of Linux (Arch, Debian, Fedora, Gentoo, Alpine, Void, NixOS, Omarchy, VanillaOS) and BSD (FreeBSD, OpenBSD, NetBSD, DragonFly BSD) distributions while eliminating fragmentation and legacy bloat, AI Agents must strictly obey the directives below.

---

## 🏛️ Section 1: Core AI Agent Engineering Directives

### 1.1 Zero External Crate Dependencies (`#![no_std]` Native Pure Rust)
- All kernel modules, security enforcers, drivers, filesystems, and userland utilities MUST be implemented natively using `#![no_std]` Rust primitives (`alloc::vec::Vec`, `alloc::string::String`, `alloc::collections::BTreeMap`, `core::ptr`).
- Do NOT introduce external third-party crates into `Cargo.toml`.
- All algorithms, parsers, cryptographic routines (Kyber-1024 / Dilithium-5), data structures, and IPC mechanisms must remain self-contained within `src/klib/` or native system shards.

### 1.2 Bare-Metal Object-Oriented Programming (OOP) Principles
Every SigmaOS component developed by AI Agents must implement clean object-oriented architecture tailored for bare-metal systems programming:
- **Encapsulation**: Hardware registers, MMIO ranges, and memory page locks must be protected inside isolated struct definitions with validated public methods.
- **Inheritance & Device Family Hierarchies**: Establish base device traits (e.g., `StorageDriver`, `NetworkDriver`, `InputDriver`) with specialized implementations (`NvmeController`, `E1000Nic`, `XhciUsbHost`).
- **Polymorphism**: Dynamic dispatch (`Box<dyn Driver>`) or static generics (`impl Trait`) to unify hardware variations under a single OS interface.
- **OS Design Patterns**:
  - *Singleton Pattern*: Global OS managers (`SovereignProcessManager`, `GlobalMemoryGovernor`).
  - *Factory Pattern*: Dynamic driver and package adapter instantiation based on PCI/USB hardware IDs (`PackageFactory`, `DriverFactory`).
  - *Observer Pattern*: Thread-safe asynchronous event notifications for device hotplug and kernel state changes (`PackageTriggerRegistry`).
  - *Adapter Pattern*: Wrapping legacy Linux/BSD interfaces into native `UnifiedPackage` and `SovereignSyscall` representations.

---

## 🐧 Section 2: Linux & BSD Distro Absorption Frameworks

AI Agents must incorporate proven architectural patterns from major OS ecosystems:

| Ecosystem / Distro | Key Innovation Absorbed | SigmaOS Native Replacement Engine |
|---|---|---|
| **Arch Linux** | PKGBUILD recipes, ALPM dependency resolution, AUR audit gating | `SovereignPackagePullRequestEngine` & `SigPkg` |
| **Debian / Ubuntu** | `dpkg-divert`, `dpkg-statoverride`, `apt-fast` mirror selection, preseed | `DebianDpkgDivertEngine`, `DebianAptFastMirrorSelectorEngine` |
| **Fedora / RHEL** | Ignition declarative provisioning, Greenboot health checks, Crypto-Policies | `FedoraGreenbootHealthCheckEngine`, `FedoraCryptoPolicyProfile` |
| **Gentoo Linux** | Portage ebuild slots, subslots, USE flag feature toggles | `GentooEclassSlotEngine`, `PortageUseFlagGovernor` |
| **FreeBSD** | ZFS Boot Environments (`bectl`), Capsicum capabilities, VuXML audit | `FreeBsdZfsBootenvEngine`, `FreeBsdCapsicumRightsEngine` |
| **OpenBSD** | Syscall restriction (`pledge`), path scoping (`unveil`), signify signatures | `OpenBsdPledgeUnveilSentinelEngine`, `OpenBsdSignifyBaseEngine` |
| **Void Linux** | Runit 3-stage init process supervision, XBPS transaction journal | `VoidRunitServiceSupervisorEngine`, `VoidXbpsTransactionJournalEngine` |
| **Omarchy Linux** | Minimal Starship prompt, Quickshell top bar, screen capture/OCR/sharing | `OmarchyMinimalStarshipPromptEngine`, `OmarchyCaptureAndSharingEngine` |
| **VanillaOS** | APX subsystem containerized package exports | `LinuxVanillaOsApxEngine` |
| **OpenWrt** | UCI unified configuration interface & IPK package management | `LinuxOpenWrtUciIpkEngine` |

---

## 🤖 Section 3: Composite AI Specialist Roles

When developing components, AI Agents adopt specific specialist mindsets:

1. **Bolt ⚡ (Performance & Efficiency Specialist)**:
   - Eliminates heap allocations in loop hot-paths ($O(N^2) \to O(N \log N)$ or O(1) ring buffers).
   - Key intermediate lookup maps on borrowed string slices (`&str`).
   - Maintains performance learnings in `.jules/bolt.md`.

2. **Palette 🎨 (UX & Accessibility Specialist)**:
   - Ensures WCAG 2.1 AA keyboard accessibility, high-contrast visual focus rings, and ARIA live regions across Zenith DE widgets and installer wizards.
   - Maintains UX learnings in `.jules/palette.md`.

3. **Sentinel 🛡️ (Zero-Trust Security Specialist)**:
   - Enforces post-quantum Dilithium-5 signature verification, bounds checking, path traversal mitigations, and `pledge`/`unveil` process sandboxing.
   - Maintains security learnings in `.jules/sentinel.md`.

---

## 🛠️ Section 4: Component Development Lifecycle & Verification

AI Agents must execute the following 5-step verification lifecycle for every change:

```
[1. Requirement & Interface Analysis] ──> [2. Safe Rust Implementation in src/]
                                                     │
[5. Documentation & Wiki Sync] <── [4. Master Test Runner] <── [3. Unit Test Verification]
```

1. **Requirement & Interface Analysis**: Review `ARCHITECTURE.md`, `ROADMAP.md`, and `WIKI/FUTURE-DEVELOPMENT-ROADMAP.md` before altering interfaces.
2. **Implementation**: Write clean, zero-dependency `#![no_std]` Rust code under `src/`. Re-export new public types in parent `mod.rs` and `src/lib.rs`.
3. **Unit Test Verification**: Include inline `#[cfg(test)] mod tests` blocks for every new struct/engine.
4. **Master Test Runner Execution**: Run `./run_sigma_tests.sh` and ensure 100% pass rate across all unit and integration test suites.
5. **Documentation & Wiki Mirror Sync**: Update `WIKI/FUTURE-DEVELOPMENT-ROADMAP.md` and execute `./scripts/sync_wiki.sh` to synchronize all documentation mirrors (`WIKI/`, `wiki/`, `wiki_repo/`).

---

*AI Agent SigmaOS Component Development Master Guide — Version 1.0 (2026)*
