# AI Agent Management: Hybrid Distro Interoperability & Subsystem Bridge

This document specifies guidelines for AI agents developing and maintaining the universal cross-distro subsystem bridge (`SovereignUniversalDistroBridge` in `src/distro/linux_bsd_inspirations.rs`).

---

## 1. Scope & Core Objectives

SigmaOS features a unified translation and interop bridge (`dispatch_cross_subsystem_operation`) allowing binaries, services, package formats, and security rules from 21+ Linux and BSD distribution modes to run natively across all 24+ core SigmaOS subsystems.

---

## 2. Core Subsystems & Cross-Distro Translation Matrix

| Subsystem Target | Supported Distro Modes | Dispatch Action & Translation Protocol |
| :--- | :--- | :--- |
| **`init` / Supervision** | Systemd, OpenRC, Runit, Shepherd, Dinit, SysVInit | Map unit files, runlevels, and runit service DAGs to `SystemdInitManager`. |
| **`package` Management** | ALPM/Pacman, DNF/RPM, APT/Deb, Portage, APK, XBPS | Translate manifest specs (`.deb`, `.rpm`, `.pkg.tar.zst`, `.ebuild`) to `UnifiedPackage`. |
| **`security` Sandboxing** | Landlock v5, FreeBSD Capsicum, OpenBSD Pledge/Unveil | Translate declarative security profiles into `SovereignLandlockV5Guard` capability masks. |
| **`storage` & VFS** | ZFS, Btrfs, HAMMER2, ext4, UFS2 | Map VFS paths (`/etc/os-release`, `/dev`, `/proc`, `/sys`, `/run/user/UID`) dynamically. |
| **`ipc` & Messaging** | Linux Pipe2/Splice, D-Bus, Fedora Messaging | Translate AMQP/ZeroMQ topics and POSIX `PIPE_BUF` ring frames into lockless DMA pipes. |

---

## 3. Mandatory AI Agent Maintenance Directives

1. **Subsystem Coverage Invariant:** Whenever a new subsystem target is added to SigmaOS, `dispatch_cross_subsystem_operation` MUST be updated to support all 21+ distro modes.
2. **Matrix Verification:** AI agents MUST ensure `verify_all_subsystems_compatibility_matrix()` passes cleanly in `src/distro/linux_bsd_inspirations.rs`.
3. **Zero Runtime Overhead:** Cross-distro dispatch must compile to direct match arms or jump tables without dynamic heap allocations during high-frequency syscall loops.
