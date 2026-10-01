# AGENT.md - SigmaOS Future Development Roadmap: Missing Linux & BSD Components

This document outlines the operational roadmap for AI agents (Bolt, Palette, Sentinel) working on closing remaining architectural, userland, kernel, and subsystem gaps between **SigmaOS** and traditional Linux and BSD distributions.

---

## 🧭 Executive Summary & Core Directives

SigmaOS aims to synthesize the best innovations across Linux (Arch, Debian, Fedora, Alpine, Void, NixOS, Clear Linux, Chimera, Pop!_OS) and BSD (FreeBSD, OpenBSD, NetBSD, DragonFly BSD, GhostBSD) into a unified, safe-Rust operating system.

When implementing or extending missing Linux & BSD components, all agents MUST follow these principles:
1. **Zero External Kernel Dependencies**: Kernel space code must remain pure safe Rust (or strictly encapsulated unsafe Rust for bare-metal hardware registers) with no third-party C library bindings.
2. **Dual POSIX & Sovereign Parity**: Foreign binaries, syscalls, package formats, and configuration files must map cleanly to native SigmaOS primitives.
3. **Comprehensive Unit Testing**: Every added component or bridge must include standalone test coverage executable via `./run_sigma_tests.sh`.

---

## 🔬 Subsystem Parity & Missing Component Roadmap

### 1. Kernel Subsystems & Microarchitecture
- **Linux eBPF / SchedExt (`scx`) Governors**: Extend `SovereignSchedExtEngine` with dynamic AI-driven CPU scheduling policies (Bore v2, EEVDF latency deadline tracking).
- **FreeBSD VNET Jails & Capsicum Framework**: Expand `FreeBsdVnetJailStackEngine` to support hierarchical nested VNET jail routing and Capsicum `cap_rights_limit` file descriptor delegation.
- **OpenBSD Pinned Syscalls & Fine-IBT**: Maintain `OpenBsdPinsyscallGuardEngine` and Indirect Branch Tracking (Fine-IBT) callsite checks for userland-to-kernel entry safety.
- **DragonFly BSD HAMMER2 PFS Replication**: Broaden multi-master PFS transaction log streaming and emergency snapshot scrubbing under high memory pressure.

### 2. Networking, Security & Firewalls
- **OpenBSD PF Firewall & iked IKEv2**: Enhance stateful packet inspection, CARP redundant failover state sync, and iked IKEv2 IPsec security association setup.
- **Linux nftables & XFRM IPsec Policy**: Maintain dual-stack nftables BPF offset rule matching and kernel XFRM IPsec SA/SP transforms.
- **NetBSD NPF Stateful Firewall & BPF**: Expand NPF bytecode JIT compilation and custom N-code packet filter inspection.
- **SLAAC IPv6 Privacy Extensions**: Maintain RFC 4941 dynamic IPv6 temporary address rotation in `OpenBsdIkedSlaacPrivacyEngine`.

### 3. Storage, Filesystems & Memory
- **Linux Bcachefs & ZRAM / Zswap**: Expand multi-device tiered storage extents, compression caching, and background scrubbing.
- **FreeBSD GEOM / CTL SCSI Target Stack**: Support GEOM Gate network block storage, gmirror/gstripe/geli encryption, and CTL SCSI LUN target routing.
- **NetBSD bioctl RAID & devpubd Hotplug**: Maintain `NetBsdBioctlDevpubdEngine` for RAID status monitoring (OK, Degraded, Failed) and devpubd dynamic hotplug event dispatching.
- **OpenBSD Otto-Malloc & FreeBSD UMA Zone Allocator**: Enforce randomized guard pages, junk byte filling, and per-CPU bucket caching in memory allocators.

### 4. Distro Userland, Init & Service Managers
- **Chimera Linux LLVM/FreeBSD Userland & dinit**: Maintain `ChimeraLinuxDinitFreeBsdUserlandEngine` supporting `dinitctl` service graph trees, FreeBSD coreutils compatibility, and LLVM toolchain sanitizers (`-fsanitize=safe-stack`).
- **Slackware pkgtool & SlackBuilds**: Expand `SlackwarePkgtoolSboEngine` for dependency-free `.txz`/`.tgz` package databases and `.SlackBuild` recipe parsing.
- **Void Linux runit & Alpine OpenRC/APK**: Maintain runit 3-stage init lifecycle supervisors and Alpine diskless RAM-boot apkovl persistence.
- **Pop!_OS System76 Power & Auto-Tiling**: Maintain `PopOsSystem76PowerAndAutoTileEngine` for energy performance profiles (Battery Saved, Balanced, High Performance), GPU mode switching (Integrated, Discrete, Hybrid, Compute), and Pop! Shell BSP window tiling layout calculations.

### 5. Packaging & Distribution Interoperability (`sigpkg`)
- **Universal Multi-Format Ingestion**: Ensure `SovereignUniversalPackageManagerInteropOrchestrator` can ingest, translate, sandbox, and install foreign packages (`.deb`, `.rpm`, `.apk`, `.pkg.tar.zst`, `.txz`, `.ebuild`, `.nixpkg`, `.xbps`, `.snap`, `.flatpak`, AppImage).
- **DPLL SAT Dependency Solver**: Maintain `SovereignUniversalManifestNormalizerSatSolver` to normalize foreign library requirements to canonical SigmaOS capabilities (e.g. `sovereign-libc`, `sovereign-openssl`).
- **Scriptlet Sandboxing**: Enforce strict OpenBSD pledge/unveil, FreeBSD Capsicum, and Linux Landlock sandbox policies when running pre/post install scriptlets.

---

## 🛠️ Verification & Test Execution Protocol

Before committing changes to any missing Linux or BSD component, run:

```bash
# 1. Compile and test specific module standalone
rustc --test src/distro/sovereign_linux_bsd_ecosystem_pinnacle_suite.rs --edition=2021 -o build/test_pinnacle
./build/test_pinnacle

# 2. Run full SigmaOS test suite
./run_sigma_tests.sh
```

Ensure **100% test pass rate** with 0 compilation errors or test regressions.

---

*End of AGENT.md*
