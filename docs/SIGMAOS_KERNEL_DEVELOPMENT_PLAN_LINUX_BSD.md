# SigmaOS Linux & BSD-Inspired Kernel Development Plan

## Executive Overview

The **SigmaOS Kernel Architecture** (`src/kernel/universal_modular_system.rs`, `src/kernel/boot_foundations.rs`, `src/kernel/vmm_paging.rs`, `src/kernel/sigma_io_uring.rs`) is engineered as a modern, memory-safe hybrid microkernel operating system. It bridges the hardware isolation and performance paradigms of leading **Linux** distributions (Linux v6.8+ LTS kernel, `io_uring` async I/O, eBPF/XDP fast-path networking, cgroups v2 resource limits, EEVDF scheduling) with the security, simplicity, and storage innovations of **BSD** operating systems (FreeBSD 14 `kldload` dynamic kernel modules, OpenBSD 7.4 PF stateful firewall & `pledge`/`unveil` sandboxing, NetBSD 10 Rump Kernel driver isolation, and Illumos/Solaris ZFS self-healing storage).

This document establishes the master development plan for the SigmaOS kernel across architectural pillars, subsystem technical specifications, a 4-phase chronological development roadmap, and benchmark metrics.

---

## 1. Architectural Philosophy & Cross-Distro Inspirations

```
                               ┌─────────────────────────────────────────┐
                               │       SigmaOS Hybrid Microkernel        │
                               └────────────────────┬────────────────────┘
                                                    │
      ┌─────────────────────────────────────────────┼─────────────────────────────────────────────┐
      ▼                                             ▼                                             ▼
┌───────────────────────────┐             ┌───────────────────────────┐             ┌───────────────────────────┐
│     Linux Kernel Core     │             │    FreeBSD & OpenBSD      │             │  NetBSD & Illumos/seL4    │
│ • eBPF / XDP Fast-Path    │             │ • FreeBSD `kldload` LKMs  │             │ • NetBSD Rump Drivers     │
│ • Async `io_uring` I/O    │             │ • OpenBSD PF Firewall     │             │ • Illumos ZFS Self-Heal   │
│ • cgroups v2 & EEVDF      │             │ • Capsicum / Pledge Rights│             │ • seL4 Capability IPC     │
└───────────────────────────┘             └───────────────────────────┘             └───────────────────────────┘
```

---

## 2. Seven Core Kernel Development Pillars

### Pillar 1: Hybrid Microkernel Core & Zero-Copy Capability IPC (`src/kernel/boot_foundations.rs`)
* **UEFI Boot, SMP Topology & Syscall ABI Table**:
  - Direct 64-bit UEFI boot protocol initialization, Secure Boot PQC hash validation, SMP AP core startup, LAPIC/IO-APIC timer ticks, and x2APIC vector steering (`src/drivers/msix_engine.rs`).
  - Ring 0 -> Ring 3 PML4 virtual memory isolation with SMEP/SMAP guards (`src/kernel/vmm_paging.rs`).
  - Fast capability-based zero-copy IPC messaging primitives inspired by seL4 and Fuchsia Zircon.

### Pillar 2: Dynamic LKM & FreeBSD `kldload` Kernel Module Subsystem (`src/kernel/universal_modular_system.rs`)
* **`SovereignModularKernelEngine`**:
  - Dynamic in-kernel module loading (`insmod` / FreeBSD `kldload`) and unloading (`rmmod` / `kldunload`).
  - Ed25519 & Dilithium PQC digital signature enforcement on kernel binaries prior to symbol linking.
  - Automated symbol table resolution (`/proc/kallsyms` / `ksyms`) and runtime module parameter tuning (`sysctl`).

### Pillar 3: udev/devd Hotplug Peripheral & Driver Subsystem (`src/hal/advanced_hal.rs`)
* **`SovereignDriverManager`**:
  - Hotplug device discovery across PCI Express, USB xHCI, NVMe, VirtIO, ACPI, and Bluetooth buses.
  - 3-tier driver architecture:
    - **Tier 1 (Core Bundled)**: Embedded x86_64 framebuffer, VirtIO block/net/gpu, NVMe, USB HID.
    - **Tier 2 (On-Demand)**: DRM/KMS graphics drivers (Intel Xe, AMDGPU, NVIDIA open-kernel).
    - **Tier 3 (Virtualization Shim)**: Fallback emulation shims for legacy or proprietary hardware.

### Pillar 4: Process Control, cgroups v2 & Capsicum Sandboxing (`src/memory/cgroups.rs`)
* **`SovereignProcessControlManager`**:
  - Resource quota management via cgroups v2 memory controllers (4-tier limits: `memory_min`, `memory_low`, `memory_high`, `memory_max`, and swap accounting).
  - FreeBSD RCTL action policies (`Log`, `Deny`, `SigKill`) and Pressure Stall Information (PSI) tracking.
  - Futex synchronization primitives (`futex_wait` / `futex_wake`) and FreeBSD Capsicum descriptor rights checks.

### Pillar 5: Universal VFS & Async `io_uring` Storage Subsystem (`src/kernel/sigma_io_uring.rs`)
* **`SovereignVfsStorageManager`**:
  - Multi-filesystem VFS abstraction layer supporting Ext4, ZFS, Btrfs, Soft Updates FFS, DevFS, ProcFS, and SysFS (`src/filesystem/mount_namespace.rs`).
  - Lock-free kernel `io_uring` submission (SQ) and completion (CQ) rings with SQPOLL background worker threads for 1M+ IOPS storage throughput.
  - Sub-second Copy-on-Write (COW) snapshotting and A/B root partition rollback (`src/filesystem/cow_snapshot.rs`).

### Pillar 6: OpenBSD PF & eBPF/XDP Fast-Path Network Stack (`src/net/tcpip_stack.rs`)
* **`SovereignNetworkStackManager`**:
  - OpenBSD PF stateful firewall rule evaluation engine with NAT, port forwarding, and connection tracking tables.
  - Kernel eBPF/XDP (eXpress Data Path) ring buffer attachment for 10Gbps+ packet filtering at the NIC driver layer.
  - VNET network namespace container isolation inspired by FreeBSD Jails and Linux network namespaces.

### Pillar 7: Default-Deny Access Control & Capability Policy Engine (`src/security/exec_guard.rs`)
* **Zorin Exec Guard & Pledge/Unveil**:
  - Default-Deny execution model enforcing capability attenuation on binaries (`FileRead`, `FileWrite`, `ProcessExec`, `NetworkTcp`).
  - OpenBSD `pledge()` system call filtering and `unveil()` filesystem path restriction guards (`src/security/pledge_impl.rs`).
  - Linux Landlock ABI v4 unprivileged filesystem sandboxing (`src/security/landlock.rs`).

---

## 3. Four-Phase Chronological Development Roadmap

```
  Phase 1: Boot Foundations & Hybrid Microkernel Core (Months 1–3)
  ├── UEFI Boot Protocol, PML4 Page Tables, & SMP AP Core Startup
  ├── POSIX Core Syscall Table & Ring 0 -> Ring 3 User Boundary
  └── Dynamic LKM Loading (`kldload`) & PQC Module Signature Checks

  Phase 2: Storage, VFS & Lock-Free `io_uring` Async I/O (Months 3–6)
  ├── VFS Mount Manager & Ext4 / ZFS / Btrfs Filesystem Drivers
  ├── Lock-Free Kernel `io_uring` SQ/CQ Ring Buffers & SQPOLL Threads
  └── Dual-Root A/B Copy-On-Write Partition Swapping & Auto-Rollback

  Phase 3: cgroups v2, Capsicum & Driver Hotplug Engine (Months 6–9)
  ├── cgroups v2 Memory Controller (4-Tier Limits, PSI & RCTL)
  ├── udev/devd Hotplug Peripheral Management & MSI-X IRQ Steering
  └── FreeBSD Capsicum Descriptor Rights & Futex Synchronization

  Phase 4: Fast-Path Networking, Default-Deny Security & Benchmarking (Months 9–12)
  ├── OpenBSD PF Stateful Firewall & eBPF/XDP Zero-Copy Network Ring
  ├── Zorin Exec Guard Default-Deny Policy Engine & Landlock ABI v4
  └── Real Hardware Smoke Testing, QEMU Validation & Performance Benchmarks
```

---

## 4. Benchmark & Acceptance Metrics

| Subsystem Target | Benchmark Framework | Target Performance Metric |
| :--- | :--- | :--- |
| **System Call Overhead** | `lmbench` / `sysbench` | < 120 nanoseconds per null system call |
| **Storage Async Throughput** | `fio` `io_uring` SQPOLL test | > 1,000,000 IOPS on PCIe Gen4 NVMe storage |
| **Network Packet Processing** | eBPF / XDP `pktgen` benchmark | > 14,000,000 packets/sec (10Gbps line rate) |
| **A/B Partition Swapping** | COW Root Snapshot Test | Sub-50ms partition metadata swap time |
| **Context Switch Latency** | `perf bench sched pipe` | < 1.2 microseconds process context switch time |
