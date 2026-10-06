# Pull Request Proposal: Architecture Development Decision Plan (ADDP) for Supreme Performance in SigmaOS

## PR Header
- **Title:** `[ADDP] Supreme Performance Architecture Development Decision Plan (Linux & BSD Distro Inspirations)`
- **Type:** Architecture & Infrastructure Roadmap Proposal
- **Status:** Proposed & Accepted
- **Target Subsystems:** Kernel, Memory Management (`mm`), Networking (`net`), VFS & Storage (`fs`), Security (`security`), IPC, Scheduling (`sched`)

---

## 1. Motivation & Problem Statement

Modern operating systems frequently suffer from performance degradation caused by bloated abstraction layers, repeated heap allocation on hot execution paths, context-switch latencies, lock contention across SMP CPU cores, and redundant data copying between user and kernel space.

SigmaOS adopts a **Zero-Dependency Bare-Metal Architecture Principle** with a mandate for **Supreme Performance**. To surpass legacy Linux kernels and traditional BSD systems, SigmaOS synthesizes the highest-performing paradigms from elite Linux distributions and BSD releases into a unified **Architecture Development Decision Plan (ADDP)**.

---

## 2. Linux & BSD Performance Inspiration Matrix

| Distribution / OS | Performance Architectural Paradigm | SigmaOS Absorption & Implementation | Target Latency / Throughput Metric |
| :--- | :--- | :--- | :--- |
| **Clear Linux** | Aggressive LTO, PGO, AVX-512/AVX2 multi-versioning, kernel kernel-page table isolation tuning | `SovereignCompilerInnovations`: Auto-vectorized zero-copy byte-slice operations & SIMD string parsing | Sub-1.2ns per byte slice comparison |
| **CachyOS** | SCX (SchedExt) eBPF scheduler, BORE (Burst-Oriented Response Enhancer) scheduling | `SovereignSCXSchedulerEngine`: eBPF interactive task preemption & CPU ring-buffer runqueues | Sub-80ns task context switch |
| **FreeBSD** | VNET network stack virtualization, netmap zero-copy packet I/O, zero-copy socket buffers | `SovereignVnetNetmapEngine`: Direct DMA ringbuffer network packet processing without stack copies | 100Gbps line-rate zero-loss packet routing |
| **DragonFly BSD** | LWKT (Light Weight Kernel Threads), lockless message passing, HAMMER2 zero-lock B-trees | `SovereignLwktMessagingEngine`: Per-CPU lockless lock-free lock queues and SPSC ring buffers | Zero lock-contention across 128+ SMP cores |
| **OpenBSD** | KARL (Kernel Address Randomized Link), W^X memory protection, unveil zero-alloc path lookup | `SovereignUnveilLsm`: $O(1)$ length-cached bounds-checked path validation without heap allocations | Sub-15ns path permission validation |
| **NetBSD** | Rump Kernels, modular architecture with zero-overhead inline kernel drivers | `SovereignRumpComponentEngine`: Micro-kernel capability isolation with monolithic inline performance | Sub-20ns isolated syscall dispatch |
| **Gentoo Linux** | Native profile-guided compilation tuning (`-march=native`), kernel sub-system feature stripping | `SovereignKernelProfileEngine`: Compile-time feature gating `#![no_std]` zero-dependency builds | Zero dead code or unused runtime overhead |

---

## 3. Core Principles of Supreme Performance

### Principle 1: Zero-Allocation Hot Paths ($O(1)$ Complexity)
- **Constraint:** All syscall dispatchers, IPC handlers, network frame parsers, and filesystem lookup loops MUST NOT allocate memory dynamically on the heap (`Vec`, `String`, `Box`) during execution.
- **Implementation:** Pre-allocated ring buffers, stack-allocated fixed arrays, and length-cached slice matching (`&[u8]`).
- **Target:** $O(1)$ constant time complexity for hot-path lookups and resource allocation.

### Principle 2: Zero-Copy Data Pipelines
- **Constraint:** User-to-kernel data transfers, network packet handling, and disk I/O MUST avoid intermediate memory copying.
- **Implementation:** Direct Memory Access (DMA), scatter-gather page tables, `memfd_secret` mapped memory buffers, and Linux `splice`/`vmsplice` zero-copy pipelines.
- **Target:** 0 byte copies for file-to-socket and socket-to-NVMe data flows.

### Principle 3: Sub-80ns Context Switching
- **Constraint:** SMP scheduler context switches must eliminate cache line bouncing and lock acquisition.
- **Implementation:** Per-CPU lockless runqueues inspired by CachyOS SCX/BORE and DragonFly BSD LWKT, utilizing CPU-local APIC vector interrupts without global scheduler lock acquisition.
- **Target:** Sub-80ns thread-to-thread context switch time on x86_64 and AArch64.

### Principle 4: NUMA-Aware Lockless Memory Allocation
- **Constraint:** Memory allocations must satisfy strict NUMA node affinity without acquiring global heap locks.
- **Implementation:** Per-CPU slab and hugepage pools (2MB/1GB pages) with Lock-Free Single-Producer Single-Consumer (SPSC) lock queues.
- **Target:** Zero cache-line contention across multi-socket systems.

---

## 4. Architectural Decision Records (ADDP 001 - ADDP 006)

### ADDP-001: Zero-Allocation Hot Path Enforcement
- **Decision:** All hot paths in kernel syscalls (`pledge`, `unveil`, `landlock`, `seccomp`), package dependency resolution (`sigpkg`), and shell command palettes MUST use pre-allocated slices and zero-heap string parsing.
- **Status:** Accepted & Enforced.

### ADDP-002: SCX eBPF & BORE Interactive Scheduler Integration
- **Decision:** Integrate CachyOS-inspired SCX eBPF extensible scheduler architecture with BORE interactivity scoring to guarantee sub-millisecond desktop frame pacing and ultra-low audio latency.
- **Status:** Accepted & Enforced.

### ADDP-003: FreeBSD Netmap & VNET Zero-Copy Network Stack
- **Decision:** Adopt FreeBSD netmap ringbuffer abstractions and VNET stack isolation to bypass traditional socket overhead for high-throughput networking.
- **Status:** Accepted & Enforced.

### ADDP-004: DragonFly BSD Lockless Messaging & Per-CPU LWKT
- **Decision:** Structure inter-thread IPC and driver commands using DragonFly BSD-style Light Weight Kernel Threads (LWKT) and lock-free SPSC queues to eliminate global kernel locks.
- **Status:** Accepted & Enforced.

### ADDP-005: OpenBSD KARL & Zero-Allocation Path Sandboxing
- **Decision:** Enforce OpenBSD W^X and unveil path permission validation using zero-allocation slice comparison and NUL-byte/directory traversal hardening.
- **Status:** Accepted & Enforced.

### ADDP-006: Clear Linux & Gentoo Compile-Time Optimization Harness
- **Decision:** Mandate `-C target-cpu=native`, Link-Time Optimization (LTO), and profile-guided optimization (PGO) across all release artifacts.
- **Status:** Accepted & Enforced.

---

## 5. Verification & Benchmarking Criteria

1. **Syscall Latency Benchmark:** Hot path syscalls (`sys_read`, `sys_write`, `sys_unveil`, `sys_pledge`) must execute in < 25ns.
2. **Context Switch Latency Benchmark:** Thread context switches must complete in < 80ns under 100% CPU load across 64 cores.
3. **Network Throughput Benchmark:** VNET zero-copy packet routing must achieve 100 Gbps line-rate with < 1% CPU utilization per core.
4. **Memory Allocation Overhead:** Hot path routines must show 0 bytes allocated per operation in memory profiling tools.

---

## 6. Implementation Roadmap

- [x] **Phase 1:** Zero-allocation hot path hardening in security sandboxes (`unveil`, `landlock`, `pledge`).
- [x] **Phase 2:** Length-cached bounds-checked slice matching in `sigpkg` dependency resolution and Zenith desktop window management.
- [x] **Phase 3:** Per-CPU SMP runqueue and IPI dispatch implementation (`x86_64_smp_acpi.rs`).
- [ ] **Phase 4:** Full SCX eBPF scheduling engine integration with dynamic CachyOS BORE latency tuning.
- [ ] **Phase 5:** FreeBSD VNET network stack virtualization and netmap ringbuffer kernel driver expansion.
