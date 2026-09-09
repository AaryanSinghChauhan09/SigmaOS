# AI Agent Guidelines: Linux & BSD Distro Parity & Component Improvements

## 📌 1. Executive Summary & Philosophy

In **SigmaOS**, system components and security models are directly inspired by the most resilient architectural innovations across **Linux and BSD distributions**.

Rather than re-inventing lower-level primitives, SigmaOS absorbs, refines, and unifies the finest ideas from FreeBSD, OpenBSD, NetBSD, DragonFly BSD, NixOS, Arch Linux, Alpine, Void, Fedora, and Chimera into a zero-dependency, capability-gated microkernel.

As an AI agent, your directive when enhancing microkernel components is to **obey distribution parity invariants, enforce least-privilege sandboxing, and ensure $100\%$ zero-dependency Rust implementation purity**.

---

## 🏛️ 2. BSD Distribution Innovations & Component Integration

```
+-----------------------------------------------------------------------------------+
|                        BSD DISTRO INNOVATIONS IN SIGMAOS                          |
+-----------------------------------------------------------------------------------+
|  🔴 FreeBSD     | Capsicum Capability Mode, Jails Isolation, netmap Fast Packet    |
|  🟡 OpenBSD     | pledge(2) Syscalls, unveil(2) VFS, signify(1) Signatures, CARP   |
|  🟠 NetBSD      | Rump Kernels (Decoupled Drivers), NPF BPF JIT Firewall           |
|  🐉 DragonFly   | HAMMER2 PFS Filesystem, CoW Snapshots, BLAKE3 Deduplication       |
+-----------------------------------------------------------------------------------+
```

### 2.1 FreeBSD Parity Modules
* **Capsicum Capability Sandboxing:** `src/security/capability.rs`, `src/distro/bsd_linux_innovations.rs`
  * Implements `cap_rights_limit()` descriptor capability degradation.
* **FreeBSD Jails:** `src/kernel/linux_bsd_innovations.rs` (`FreeBsdJail`)
  * Hierarchical, capability-isolated process environments with nested jail support.
* **netmap Zero-Copy Networking:** `src/network/distro_net.rs` (`KernelFastPacketEngine`)
  * Lock-free ring buffer packet I/O bypassing standard VFS overhead.

### 2.2 OpenBSD Parity Modules
* **`pledge(2)` Privilege Reduction:** `src/security/pledge.rs`, `src/shell/sigma_sh.rs`
  * Restricts permitted system calls (`PLEDGE_STDIO`, `PLEDGE_RPATH`, `PLEDGE_INET`, `PLEDGE_EXEC`).
* **`unveil(2)` Path Sandboxing:** `src/security/sigma_unveil.rs`
  * Hides and restricts filesystem paths per-process (`UnveilPermissions::READ`, `WRITE`, `EXEC`).
* **`signify(1)` Cryptographic Signatures:** `src/sigpkg/declarative_build.rs` (`OpenBsdSignifyPackageReproducer`)
  * Small, tamper-evident manifest signatures verifying package deposit integrity.

### 2.3 NetBSD & DragonFly BSD Parity Modules
* **NetBSD Rump Kernels:** `src/kernel/linux_bsd_innovations.rs` (`NetBsdRumpKernel`)
  * Runs driver components inside isolated userspace hypercalls without kernel panic risks.
* **DragonFly HAMMER2 PFS:** `src/kernel/linux_bsd_innovations.rs` (`Hammer2PfsSnapshot`)
  * Multi-volume CoW filesystem with instant snapshotting and BLAKE3 block-level deduplication.

---

## 🐧 3. Linux Distribution Innovations & Component Integration

```
+-----------------------------------------------------------------------------------+
|                       LINUX DISTRO INNOVATIONS IN SIGMAOS                         |
+-----------------------------------------------------------------------------------+
|  ❄️ NixOS / Guix| Flakes Declarative State, Immutable Generational Rollback        |
|  🏹 Arch Linux  | Pacman Hooks, PKGBUILD Recipe Compiler, Clean Chroot Builds       |
|  🏔️ Alpine      | apk v3 Index Parser, musl-Hardened Memory, Dinit Supervisor      |
|  🌀 Void Linux  | xbps-src Clean-Room Builder, runit Fast Service Supervisor       |
|  🛡️ Fedora      | ostree Atomic Deployments, Keylime TPM 2.0 Measured Attestation    |
+-----------------------------------------------------------------------------------+
```

### 3.1 NixOS & Guix Declarative Configuration Engine
* **Module Location:** `src/system/state.rs`, `src/distro/linux_bsd_parity.rs` (`NixOSFlakeEngine`)
* **Behavior:** Replaces chaotic `/etc/*` text file sprawl with a single, version-controlled declarative state graph (`DeclarativeStateGraph`).
* **Generational Rollbacks:** $O(1)$ sub-millisecond atomic state rollbacks (`rollback(generation_id)`).

### 3.2 Universal Package Manager (`SigmaPkg`) & Transpilation
* **Module Location:** `src/package/universal.rs`, `src/sigpkg/universal_adapter.rs`
* **Transpilation Rule:** Auto-detects, unpacks, and transpiles foreign DEB, RPM, Pacman, APK, PKGNG, XBPS, and Nix packages into native `.sigmapkg` format with complete dependency translation (`UniversalPackageFormatBridge`).

---

## 🛡️ 4. AI Agent Mandatory Implementation Rules

1. **Least-Privilege Default:**
   * Every new shell command, userland tool, or service MUST invoke `pledge()` and `unveil()` upon initialization to enforce strict least-privilege sandboxing.
2. **Zero POSIX Bloat:**
   * Do not import heavy POSIX C libraries or external binary utilities. All distro-inspired innovations MUST be written natively in `#![no_std]` Rust.
3. **Atomic Installs & Rollbacks:**
   * All filesystem modifications and package installs MUST write to temporary staging areas before performing an atomic CoW commit (`SnapperTransactionGuard`).

---

## 🧪 5. Standalone Testing Commands

AI agents can verify Linux and BSD distro-inspired parity modules via standalone unit compilation:

```bash
# Test FreeBSD Capsicum, OpenBSD pledge/unveil, and Linux cgroups
rustc --test --edition=2021 src/security/capability.rs -o build/cap_tests && ./build/cap_tests && rm build/cap_tests

# Test NixOS Flakes, Arch Pacman, and Void runit innovations
rustc --test --edition=2021 src/distro/linux_bsd_parity.rs -o build/distro_tests && ./build/distro_tests && rm build/distro_tests

# Test Universal Package Format Bridge (DEB, RPM, Pacman, APK, Nix)
rustc --test --edition=2021 src/package/universal.rs -o build/pkg_bridge_tests && ./build/pkg_bridge_tests && rm build/pkg_bridge_tests
```
