# AI Agent Roadmap: Linux & BSD Kernel, Storage & Security Innovations (PR Format)

## Overview
This roadmap establishes guidelines and specifications for AI agents developing low-level kernel, storage, and security components inspired by Linux, FreeBSD, OpenBSD, NetBSD, and DragonFly BSD in SigmaOS.

## Priority Architectural Pillars

### 1. Kernel Scheduling & MicroVM Hypervisor Isolation
- **Linux eBPF SchedExt & BORE**: Pluggable schedulers for low-latency gaming and real-time processing.
- **FreeBSD VNET Jails & MicroVMs**: Per-jail network stack virtualization and Bhyve/Firecracker microVM isolation.

### 2. Immutable Storage & CoW Snapshot Management
- **openSUSE MicroOS Transactional Rootfs**: Read-only Btrfs/ZFS root filesystems with transactional update snapshots and atomic A/B reboot application.
- **DragonFly BSD HAMMER2 & FreeBSD bectl**: Multi-master PFS transaction replication and ZFS Boot Environment management.

### 3. Capability Security & Mandatory Access Control
- **OpenBSD Pledge & Unveil**: Granular process capability restriction and path isolation tables.
- **FreeBSD Capsicum & Casper**: Descriptor-based capability rights and delegated daemon services.

### 4. Package Ecosystem Transpilation
- **Void Linux XBPS & Alpine APK v3**: Dependency graph solving, SHA-256 binary validation, and file trigger dispatchers.
- **Gentoo Portage eLinux / Embedded Targets**: Cross-compilation sysroots and granular USE flag evaluation.

## Pull Request Guidelines for AI Agents
1. Every new component must be implemented in Rust under `src/`.
2. Re-export all public structs in `src/distro/mod.rs` and `src/lib.rs`.
3. Add a standalone test runner block to `./run_sigma_tests.sh`.
4. Ensure 100% test pass rate across all 65+ test binaries.
