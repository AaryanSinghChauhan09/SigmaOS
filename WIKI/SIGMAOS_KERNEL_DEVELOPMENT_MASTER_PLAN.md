# SigmaOS Kernel Development Master Plan: Linux & BSD Inspired Architecture

## Executive Summary

The **SigmaOS Kernel** is engineered as a hybrid microkernel operating system bridging high-performance Linux kernel constructs (`mm/`, `sched/`, `io_uring/`, `net/ebpf/`) with FreeBSD reliability (`UMA`, `GEOM`, `intr_event`), OpenBSD security mitigation (`pledge`/`unveil`, `KARL`, `pinsyscall`), NetBSD driver modularity (`rump`), and seL4 microkernel formal verification invariants (`src/kernel/ipc.rs`).

This document defines the master development plan for the SigmaOS kernel across architectural pillars, subsystem specifications, a 4-phase chronological development roadmap, and verification benchmark metrics.

---

## 1. Architectural Philosophy & Cross-Distro Inspirations

```
                          ┌──────────────────────────────────────────────────────────┐
                          │               SigmaOS Hybrid Microkernel                 │
                          └────────────────────────────┬─────────────────────────────┘
                                                       │
      ┌────────────────────────────────────────────────┼────────────────────────────────────────────────┐
      ▼                                                ▼                                                ▼
┌───────────────────────────┐            ┌───────────────────────────┐            ┌───────────────────────────┐
│     Linux Kernel 6.8+     │            │       FreeBSD 14.0        │            │       OpenBSD 7.4         │
│ • EEVDF / BORE Scheduler  │            │ • UMA Zone Allocator      │            │ • Pledge & Unveil Sandbox │
│ • io_uring Ring Buffers   │            │ • GEOM Storage Framework  │            │ • KARL Kernel Relinking   │
│ • eBPF / XDP JIT Runtime  │            │ • VNET Jail Sandboxing    │            │ • PinSyscall Hardening    │
└───────────────────────────┘            └───────────────────────────┘            └───────────────────────────┘
      │                                                │                                                │
      ▼                                                ▼                                                ▼
┌───────────────────────────┐            ┌───────────────────────────┐            ┌───────────────────────────┐
│      seL4 Microkernel     │            │       NetBSD 10.0         │            │   Illumos / Solaris 11    │
│ • Formal Capability Proofs│            │ • Rump Anykernel Drivers  │            │ • Crossbow VNIC Virtualization│
│ • Zero-Copy IPC Channels  │            │ • Veriexec Integrity      │            │ • DTrace Dynamic Tracing  │
└───────────────────────────┘            └───────────────────────────┘            └───────────────────────────┘
```

---

## 2. Six Core Kernel Development Pillars

### Pillar 1: Scheduler, Interrupts & Process Control (`src/kernel/sched/`, `src/hardware/`)
* **EEVDF / BORE Real-Time Scheduler (`src/kernel/sched/deadline.rs`)**:
  - Implement Earliest Eligible Virtual Deadline First (EEVDF) with Burst-Oriented Response Enhancer (BORE) latency boosts for interactive Zenith compositor threads.
  - Implement `SCHED_DEADLINE` Earliest Deadline First (EDF) and Constant Bandwidth Server (CBS) guarantees for audio/video tasks.
* **Dynamic IRQ Load Balancing (`src/hardware/compatibility.rs`, `src/drivers/msix_engine.rs`)**:
  - Dynamically rebalance IO-APIC redirection table entries across CPU cores based on active throughput counters (`irqbalance` model).
  - Multi-queue MSI-X vector steering allocating 1:1 NVMe and 100GbE interrupt vectors across CPU APIC IDs.

### Pillar 2: Memory Management & Demand Paging (`src/memory/`, `src/kernel/vmm_paging.rs`)
* **Physical & Virtual Memory Architecture**:
  - 4-level PML4 page table translation with Ring 0 (Kernel) vs. Ring 3 (User) boundary isolation, SMEP/SMAP CPU guards, and ASLR entropy.
  - Transparent Huge Pages (THP) 2MB/1GB page table entry promotion and background `khugepaged` page collapsing.
* **Per-CPU UMA / SLUB Allocator (`src/memory/zone.rs`)**:
  - CPU-local lockless object caches (`kmem_cache_cpu`) providing zero-contention kernel object allocation on 64+ core systems.
* **Demand Paging & Swap Compression (`src/memory/cma.rs`, `src/memory/cgroups.rs`)**:
  - Zero-fill anonymous allocation on initial fault, Copy-On-Write page duplication, and ZRAM/Zswap LZ4 page compression pools.
  - Linux cgroups v2 4-tier limits (`memory.min`, `memory.low`, `memory.high`, `memory.max`) and Pressure Stall Information (PSI).

### Pillar 3: Modular Driver Architecture & Peripheral Bus (`src/kernel/drivers/`, `src/hal/`)
* **Native Bare-Metal Drivers**:
  - PCIe Gen 5 NVMe driver (`src/kernel/drivers/nvme.rs`) with admin and I/O submission/completion queues.
  - USB 3.x xHCI host controller driver (`src/kernel/drivers/xhci.rs`) with transfer rings and event endpoints.
* **NetBSD Rump Anykernel Driver Isolation (`src/kernel/universal_modular_system.rs`)**:
  - Run third-party device drivers in sandboxed userland micro-domains, preventing fault propagation to the core kernel.
* **3-Tier Driver Bundling (`src/hal/advanced_hal.rs`)**:
  - `Tier1CoreBundled` (built-in storage, display, HID), `Tier2OnDemandFetch` (wireless, GPU firmware), and `Tier3VirtualizationShim` (QEMU/KVM fallback).

### Pillar 4: Filesystem Architecture & Async I/O (`src/filesystem/`, `src/kernel/sigma_io_uring.rs`)
* **Multi-Filesystem VFS Engine (`src/filesystem/vfs.rs`)**:
  - Native Ext4 with POSIX.1e extended ACLs, Btrfs with Copy-on-Write subvolumes, and ZFS zpool boot environment slots.
  - Dual-Root A/B Copy-On-Write atomic system updates with sub-50ms metadata cloning and 60-second hardware watchdog rollback.
* **High-Performance `io_uring` Subsystem (`src/kernel/sigma_io_uring.rs`)**:
  - Lock-free Submission Queue (SQ) and Completion Queue (CQ) ring buffers with kernel `SQPOLL` thread offload.
  - Fixed registered file descriptor tables (`IORING_REGISTER_FILES`) and provided buffer rings.

### Pillar 5: Network Stack & eBPF/XDP JIT Runtime (`src/network/`, `src/kernel/sigma_ebpf_runtime.rs`)
* **OpenBSD PF Stateful Firewall & FreeBSD VNET Jails**:
  - Stateful packet filtering with NAT table state, TCP sequence randomization, and FreeBSD VNET network namespace isolation.
* **Native x86_64 eBPF JIT Compiler (`src/kernel/sigma_ebpf_runtime.rs`)**:
  - Translate eBPF bytecode instructions into native x86_64 machine code for XDP fast-path packet filtering at wire speed.

### Pillar 6: Mandatory Access Control & Kernel Hardening (`src/security/`)
* **Multi-Layer MAC Engine (`src/security/mandatory_access_control.rs`, `src/security/landlock.rs`)**:
  - SELinux security contexts (`user:role:type:level`), Landlock ABI v4 filesystem and TCP socket restriction rulesets.
  - OpenBSD `pledge`/`unveil` privilege reduction and Zorin Exec Guard default-deny execution policies (`src/security/exec_guard.rs`).
* **Confidential Computing & Integrity Attestation (`src/security/pqc_measurement.rs`)**:
  - AMD SEV-SNP and Intel TDX encrypted memory page flag enforcement (`ENCRYPTED_SEV_SNP`, `INTEL_TDX_SHARED`).
  - Dilithium-5 PQC kernel signature verification and TPM 2.0 PCR attestation.

---

## 3. Four-Phase Chronological Kernel Roadmap

```
  Phase 1: Bare-Metal Foundations (Months 1–3)
  ├── UEFI Boot Protocol, Secure Boot & Initramfs Parser
  ├── SMP AP Core Startup & IO-APIC / HPET Timer Dispatches
  ├── x86_64 PML4 Page Tables, SMEP/SMAP Guards & Ring 0 -> 3 Boundary
  └── NVMe PCIe & USB 3.x xHCI Controller Drivers

  Phase 2: Memory, Scheduler & Storage Engines (Months 3–6)
  ├── EEVDF / BORE Scheduler & SCHED_DEADLINE Real-Time EDF/CBS
  ├── Per-CPU SLUB/UMA Caches & Transparent Huge Pages (THP)
  ├── Demand Paging Fault Handler & Copy-On-Write Engine
  └── io_uring Lock-Free SQ/CQ Rings & Dual-Root A/B Atomic Updates

  Phase 3: Networking, eBPF & Confidential Computing (Months 6–9)
  ├── OpenBSD PF Stateful Firewall & FreeBSD VNET Jails
  ├── Native x86_64 eBPF JIT Compiler & XDP Fast-Path Engine
  ├── AMD SEV-SNP & Intel TDX Confidential Memory Pages
  └── Live Kernel Patching (`SovereignLivePatchEngine`)

  Phase 4: Qualification & Release Engineering (Months 9–12)
  ├── `will-it-scale` & `stress-ng` Multi-Core Concurrence Validation
  ├── Physical Hardware Support Qualification Matrix
  ├── Full PQC Attestation & Systemd Target Parity Verification
  └── SigmaOS v1.0 Long-Term Support (LTS) Release
```

---

## 4. Verification and Benchmark Metrics

| Subsystem Target | Benchmark Framework | Target Performance Metric |
| :--- | :--- | :--- |
| **Scheduler Latency** | `hackbench` / `cyclictest` | Sub-50 microsecond worst-case dispatch latency |
| **SLUB Multi-Core Scaling** | `will-it-scale` (malloc/page_fault) | > 90% linear scaling up to 128 CPU cores with zero lock contention |
| **Async Storage Throughput** | `fio` (io_uring Engine, 4K Random Read) | > 1,200,000 IOPS on PCIe Gen 5 NVMe SSDs |
| **XDP Packet Filtering** | `pktgen` 64-Byte Wire Speed Test | > 14.8 Million Packets Per Second (Mpps) per 10GbE port |
| **A/B System Rollback** | Cold Reboot Watchdog Test | < 50ms Copy-on-Write metadata clone & instant watchdog rollback |
