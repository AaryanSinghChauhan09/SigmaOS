# AI Agent Component Development Guide: SigmaOS Subsystems

This document provides architectural standards, engineering principles, and operational guidelines for AI agents developing and maintaining operating system components in SigmaOS.

---

## 1. Executive Summary & Linux/BSD Distro Inspiration Matrix

SigmaOS absorbs proven innovations across leading Linux distributions and BSD variants into a self-sufficient, zero-dependency, safe Rust operating system architecture. When an AI agent develops or refactors any component in SigmaOS, it must adhere to clean-room implementations inspired by these open-source operating systems:

| Linux / BSD Distribution | Inspired Subsystem & Architectural Concept | Target SigmaOS Module Path |
| :--- | :--- | :--- |
| **Arch Linux** | Pacman ALPM database integrity, `makepkg` recipes, AUR helper, PKGBUILD parsers | `src/distro/arch.rs`, `src/sigpkg/arch_pacman_engine.rs` |
| **Debian / Ubuntu** | `dpkg` triggers, `debconf` preseed, APT pin rules, AppArmor security profiles | `src/package/universal.rs`, `src/security/selinux.rs` |
| **Fedora / RHEL** | DNF5 advisory solver, Koji build server, Greenwave CI, Ignition provisioning | `src/compatibility/fedora.rs`, `src/package/universal.rs` |
| **Gentoo Linux** | Portage USE flags, subslot dependencies, ebuild CFLAGS tuner, sandbox environment | `src/distro/gentoo_inspirations.rs`, `src/package/` |
| **Alpine / Void** | Musl APK world state, Void runit stage supervisor, XBPS soname orphan resolver | `src/distro/void_runit.rs`, `src/package/universal.rs` |
| **NixOS / Guix** | Content-addressed `/sigma/store`, declarative generation rollbacks, hermetic builds | `src/filesystem/fhs_engine.rs`, `src/package/universal.rs` |
| **openSUSE** | YaST2 control center, Snapper Btrfs CoW pre/post transaction recovery | `src/distro/missing_linux_bsd_components.rs` |
| **FreeBSD** | VNET virtualized network stack isolation, Jails containerization, Capsicum rights, GEOM | `src/net/linux_bsd_network_innovations.rs`, `src/storage/geom.rs` |
| **OpenBSD** | `pledge` & `unveil` syscall sandboxing, KARL kernel randomization, Signify PKG verification | `src/security/syscall_filter.rs`, `src/security/pledge.rs` |
| **NetBSD** | Rump kernel driver server isolation, rump call dispatching, component servers | `src/distro/missing_linux_bsd_components.rs` |
| **DragonFly BSD** | HAMMER2 crash-consistent CoW filesystem, deduplication, VARSYM symbol resolution | `src/distro/missing_linux_bsd_components.rs` |
| **Illumos / Solaris** | DTrace USDT dynamic probes, ZFS ARC adaptive replacement cache, Crossbow VNICs | `src/distro/missing_linux_bsd_components.rs` |

---

## 2. Core Engineering Principles for AI Agents

1. **Zero External Dependencies (`#![no_std]` Purity):**
   - Core kernel, driver, and system components must not depend on external crates in `Cargo.toml`.
   - Use native alloc primitives (`alloc::vec::Vec`, `alloc::string::String`, `alloc::collections::BTreeMap`) or stack-allocated buffers (`[u8; N]`) for bare-metal contexts.
2. **Clean-Room Safe Rust Implementations:**
   - Never copy C/C++ or GPL source code directly. All implementations must be clean-room safe Rust code designed from public API specifications, POSIX standards, or open documentation.
3. **Deterministic Error Handling:**
   - Return explicit `Result<T, &'static str>` or domain-specific error enums rather than panicking or invoking `unwrap()` on untrusted input data.
4. **Memory Alignment & Bounded Allocations:**
   - Validate slice bounds and buffer capacities prior to indexing or memory copies. Use wrapping distance arithmetic or bitwise index masking for ring buffers to prevent integer overflows.

---

## 3. Component Subsystem Architectural Mapping

When adding or updating features, AI agents must route code into the appropriate top-level module:

```
src/
├── kernel/        # Core scheduler, memory paging, io_uring, process isolation, cgroups
├── fs/            # VFS layer, SigmaFS, bcachefs, ZFS ARC, OverlayFS, FHS translator
├── net/           # TCP BBR/CUBIC, eBPF/XDP zero-copy, FreeBSD VNET, WireGuard PQC
├── security/      # Seccomp-BPF, OpenBSD pledge/unveil, Capsicum rights, sudo/doas
├── desktop/       # Zenith Wayland compositor, XFCE engine, Omarchy desktop, speaker tuning
├── package/       # Universal package manager, multi-format adapters (Deb, Arch, RPM, APK, PKG)
├── drivers/       # Universal driver lifecycle, udev/devd hardware hotplug event bridge
├── ai/            # On-device text dictation, compute scheduler, local LLM quantization
└── distro/        # Distro-specific gap closures, inspirations, and compatibility synthesis
```

---

## 4. Verification Protocol & Standalone Test Runner

All AI agent changes must be verified using both standalone rustc test compilation and the native test runner harness:

1. **Individual Module Standalone Test:**
   ```bash
   rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/<subsystem>/<file>.rs -o build/test_<file> && ./build/test_<file>
   ```
2. **Comprehensive Native Test Suite:**
   ```bash
   ./run_sigma_tests.sh
   ```

---

## 5. Maintenance Checklist for AI Agents

- [ ] **No Hardcoded Secrets or Credentials:** Ensure tokens, passwords, or secrets use dynamic token generation or mock identifiers (`SIGMA_NO_HARDCODED_SECRET`).
- [ ] **Unsafe Rust Isolation:** Restrict `unsafe` blocks strictly to low-level hardware or FFI interfaces with explicit `// SAFETY:` invariant comments (`SIGMA_NO_UNSAFE`).
- [ ] **Standalone Test Coverage:** Accompany every new struct or engine method with unit tests under `#[cfg(test)]`.
- [ ] **Regression Verification:** Execute `./run_sigma_tests.sh` to confirm zero test failures, zero panic regressions, and clean compilation.

---

*Document Version:* 1.0.0
*Maintained By:* SigmaOS AI Agent Core Engineering & Architecture Committee
