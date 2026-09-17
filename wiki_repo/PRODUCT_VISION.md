# SigmaOS Product Vision & Engineering Contract

## 1. Executive Product Statement
**SigmaOS is a secure, fast, opinionated Rust desktop operating system with atomic updates, capability-based applications, and a curated Zenith workflow.**

Inspired by **Omarchy Linux**, SigmaOS prioritizes shipping a cohesive, daily-driver desktop distribution rather than attempting to simultaneously be a universal microkernel, cloud orchestrator, bare-metal hypervisor, and full POSIX replacement for every existing operating system.

---

## 2. The Engineering Contract

### 2.1 Supported Architecture & Runtime Model
- **Primary Target for Desktop Edition (M0–M5)**: `x86_64` (UEFI / VirtIO in QEMU, progressing to Intel/AMD integrated graphics).
- **Core Standard Library Decision**:
  - SigmaOS Desktop Edition uses a **hybrid practical `std` runtime** for userspace applications, compositor, package management (`sigpkg`), and desktop shells.
  - Low-level kernel primitives (`src/kernel/`, `src/klib/`) maintain `#![no_std]` compatibility to preserve the freestanding microkernel evolutionary path.
  - No undocumented switching: any module requiring `std` explicitly imports standard collections (`std::collections::BTreeMap`), while freestanding boot blocks explicitly use `#![no_std]`.

### 2.2 Strict Zero-External-Dependency Rule
- **Definition**: The root `Cargo.toml` maintains **0 third-party external crates** (`[dependencies]` is empty).
- All cryptographic primitives, data structures, parsers, protocols, Wayland bridges, and packaging solvers are natively implemented and audited in-tree.

### 2.3 Single Demonstrable Product Path
All engineering efforts converge on one verifiable end-to-end loop:
```text
Download Verified Image
  └──> Boot via UEFI in QEMU / Target Hardware
        └──> Minimal Install / User Selection
              └──> Zenith Wayland Desktop Session
                    └──> Terminal & Application Launcher
                          └──> Install Signed Package (sigpkg)
                                └──> Atomic Update with Generation Snapshot
                                      └──> Instant Safe Rollback upon Failure
```

---

## 3. Product Deliverables by Stage

| Subsystem | Scope for Desktop Release | Long-Term Research (Deferred) |
|---|---|---|
| **Compositor** | Zenith Wayland compositor (`src/compositor/zenith_core.rs`), layer-shell widgets, Tokyo-Night / Catppuccin theme live-switching | Full X11 nested rootless multi-seat server |
| **Packaging** | Native signed SigmaPkg with Merkle store, Adler-32 delta updates, and atomic rollback | Full AUR community hosting, 50 native distro package builders |
| **Security** | OpenBSD-style `pledge` & `unveil`, Capsicum fd rights, Landlock sandbox prompts | Distributed TPM cluster remote attestation |
| **Init & Boot** | Sub-second parallel boot sequencer (<250ms target) | Distributed cloud init orchestrators |
| **Hardware** | x86_64 UEFI, VirtIO GPU/net/block, NVMe, USB HID | Obsolete legacy architectures (Alpha, SH4, SPARC) |
