# AGENT.md - SigmaOS Future Development Roadmap for Missing Linux & BSD Components

This document serves as the operational guide and technical specification for AI engineering agents working on closing all remaining component and subsystem gaps between **SigmaOS** and upstream **Linux** and **BSD** operating system distributions.

---

## 🎯 Master Objective

Achieve 100% feature, ABI, and operational parity with modern Linux (6.12+ LTS) and BSD (FreeBSD 14.1, OpenBSD 7.6, NetBSD 10.0, DragonFly BSD 6.4) ecosystems in **zero-dependency, safe Rust**.

---

## 🚀 Priority Roadmap for Missing Components

### Phase 1: Linux Kernel Subsystems & Hardening

1. **`sched_ext` (eBPF Extensible Scheduler)**
   - *Target*: Parity with Linux 6.12+ `scx` scheduler frameworks (`scx_bpfland`, `scx_rusty`, `scx_lavd`).
   - *Requirement*: Dynamic userland/eBPF CPU task scheduling policies for real-time and gaming workloads.

2. **Landlock LSM v5 & BPF LSM**
   - *Target*: Linux unprivileged filesystem sandboxing & eBPF LSM hook gates.
   - *Requirement*: Path-based read/write/exec restriction rules enforced per thread.

3. **Bcachefs Advanced Tiered CoW Storage**
   - *Target*: Parity with Bcachefs multode tiering, encryption, and inline compression.
   - *Requirement*: Extent-based copy-on-write allocation with automatic SSD/NVMe caching tiers.

4. **io_uring Asynchronous Ring Buffer Engine**
   - *Target*: High-throughput zero-copy asynchronous I/O completion queues.
   - *Requirement*: Fast submission (`SQ`) and completion (`CQ`) ring buffers for network and storage syscalls.

5. **systemd 256+ Parity & Varlink IPC**
   - *Target*: Modern `systemd-sysext`, `systemd-confext`, `homed`, and Varlink binary IPC transport.
   - *Requirement*: Immutable system extension overlay mounts and PQC encrypted home directories.

---

### Phase 2: BSD Subsystem Innovations

1. **FreeBSD 14.1 VNET, Jails, Capsicum & GEOM**
   - *Target*: Complete VNET virtual network stack per jail and Capsicum capability mode sandboxing.
   - *Requirement*: Fine-grained file descriptor rights enforcement (`CAP_READ`, `CAP_WRITE`, `CAP_SEEK`).

2. **OpenBSD 7.6+ Pledge, Unveil, KARL & pf**
   - *Target*: Process call promise restrictions (`pledge`), path visibility locks (`unveil`), Kernel Address Randomized Link (`KARL`), and stateful Packet Filter (`pf`).
   - *Requirement*: Mandatory pledge/unveil sandboxing for all userland scriptlets and processes.

3. **NetBSD Rump Kernels & Anyware Drivers**
   - *Target*: Modular hypercall-based userland kernel drivers for VFS, TCP/IP, and USB.
   - *Requirement*: Microkernel-style isolated driver execution without host kernel panics.

4. **DragonFly BSD HAMMER2 Multi-PFS & Block Deduplication**
   - *Target*: Pseudo Filesystems (PFS), cluster replication, and Merkle tree block deduplication.
   - *Requirement*: Zero-overhead snapshotting and multi-node cluster state synchronization.

---

### Phase 3: Universal Package Management & Distro Absorption

1. **SigmaPkg Universal Ingestion Engine (`src/package/`)**
   - *Target*: Absorption of 29+ foreign package formats into native `SigmaPkg`.
   - *Formats*: `.pkg.tar.zst` (Arch), `.deb` (Debian/Ubuntu), `.rpm` (Fedora/RHEL), `.apk` (Alpine), `.xbps` (Void), `.ebuild` (Gentoo), `.nix` (NixOS), `.pkg` (FreeBSD), `.flatpak`, `.snap`, `.appimage`, `.ipk` (OpenWrt), `.eopkg` (Solus), `.pet`/`.pup` (Puppy), `.txz` (Slackware), `.p5p` (Illumos).

2. **DPLL SAT Dependency Resolver & Mirror Manager**
   - *Target*: Exact multi-distro dependency graph resolution and `/etc/pacman.d/mirrorlist` speed benchmarking.
   - *Requirement*: Direct translation of foreign package capability dependencies into canonical SigmaOS capabilities (`sovereign-libc`, `sovereign-openssl`, `sovereign-graphics`).

---

## 🛠️ Verification & Testing Mandate

Agents working on these components must ensure:
1. All changes compile cleanly under `cargo check --lib`.
2. Unit tests covering new data structures and methods are placed in the respective file under `#[cfg(test)]`.
3. The master test suite `./run_sigma_tests.sh` executes with a **100% pass rate**.

---
