# Sovereign Kernel Architecture & Subsystem Specification

## Overview
SigmaOS features a bare-metal 64-bit multi-core kernel architecture inspired by Linux, FreeBSD, OpenBSD, and NetBSD.

## Subsystems
1. **64-bit SMP Bootstrap**: PML4 identity page tables, GDT initialization, Multiboot1/2 handoff, early UART serial output, and 16-bit to 64-bit AP real-mode trampoline.
2. **Page Compaction & Memory Reclaim**: High-order allocation failure compaction passes, kswapd LRU page reclaim, ZRAM swap compression, and transparent huge pages (THP).
3. **Real-Time Hybrid Scheduler**: Sub-5 microsecond RTLane deadline scheduling, POSIX RT preemption-threshold gating, FreeBSD ULE interactivity scoring, and NUMA/DVFS power governor tuning.
4. **I/O & Network Engine**: `io_uring` v2 async rings, zero-copy `splice` page pipelines, XDP express data path shaping, stateful nftables / pf packet filtering, and proxy firewalls.
5. **Security & Privilege Escalation**: Threat-safe privilege escalation validation with constant-time token comparison, OpenBSD `pledge`/`unveil`, FreeBSD Capsicum, and Linux Landlock LSM sandboxing.
