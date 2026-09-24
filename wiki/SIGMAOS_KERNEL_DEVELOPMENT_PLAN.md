# Strategic Development Plan for the SigmaOS Kernel

## Executive Summary
This document establishes the strategic 5-phase development roadmap for the SigmaOS microkernel (`src/kernel/`). Inspired by Linux 6.12+ LTS, FreeBSD 14.1+, OpenBSD 7.6+, and Omarchy Linux desktop paradigms, the plan outlines key architectural advancements across Virtual Memory Management (VMM), asynchronous `io_uring` I/O engines, Multi-Arch HAL with ACPI NUMA topology, Post-Quantum Cryptography (PQC) kernel module verification, and eBPF dynamic tracing with self-healing capabilities.

---

## 1. Kernel Architecture Inspiration & Benchmark Matrix

| Operating System | Kernel Feature | Absorbed Kernel Technology | SigmaOS Integration Layer |
| :--- | :--- | :--- | :--- |
| **Linux 6.12+ LTS** | 5-Level PML5 Paging | 57-bit Virtual Address Space expansion (`CR4.LA57`) | `src/kernel/vmm_paging.rs` (`Pml5PageTable`) |
| **Linux 6.12+ LTS** | `io_uring` SQPOLL | Kernel-thread submission polling for zero-syscall I/O | `src/kernel/io_uring_sqpoll_sovereign.rs` |
| **Linux 6.12+ LTS** | Landlock LSM v5 | Path-based and network-scoped process sandboxing | `src/kernel/sovereign_nextgen_distro_leap.rs` |
| **FreeBSD 14.1+** | Capsicum Capabilities | Unforgeable descriptor rights delegation | `src/kernel/bsd_kernel_parity.rs` |
| **FreeBSD 14.1+** | VNET Network Jails | Virtualized network stack instances per jail/container | `src/kernel/linux_bsd_innovations.rs` (`FreeBsdVnetManager`) |
| **OpenBSD 7.6+** | Pledge & Unveil | System call promise restrictions & restricted path views | `src/kernel/linux_bsd_innovations.rs` (`OpenBsdPledge`) |
| **OpenBSD 7.6+** | `pinsyscall` | System call instruction pointer validation | `src/kernel/sovereign_2026_distro_leap_engine.rs` |
| **CachyOS** | BORE Scheduler | Burst-Oriented Response Enhancer for interactive desktop | `src/kernel/bore.rs` (`InteractiveHybridScheduler`) |

---

## 2. Strategic 5-Phase Kernel Development Roadmap

```
┌───────────────────────────────────────────────────────────────────────────┐
│                 SIGMAOS KERNEL DEVELOPMENT ROADMAP                        │
└───────────────────────────────────────────────────────────────────────────┘
   Phase 1: VMM & Proactive Memory Compaction
   ├── 57-bit PML5 paging (`Pml5PageTable` in `vmm_paging.rs`)
   ├── Background proactive compaction daemon (`src/kernel/memory.rs`)
   └── Adaptive KSM (Kernel Samepage Merging) zero-page hash scanner

   Phase 2: Asynchronous io_uring SQPOLL & Zero-Copy VFS
   ├── Kernel-thread SQPOLL worker daemon (`io_uring_sqpoll_sovereign.rs`)
   ├── Fixed I/O buffer registration (`IORING_REGISTER_BUFFERS`)
   └── Async zero-copy `splice`/`sendfile` pipe and socket pipeline

   Phase 3: Multi-Arch HAL & ACPI NUMA / IOMMU DMA Isolation
   ├── ACPI 6.5+ SRAT/SLIT table parsing in `src/kernel/hal.rs`
   ├── Per-device IOMMU DMA page table domain mapping (`src/kernel/iommu.rs`)
   └── CXL 3.0 cache-coherent memory pool abstractions

   Phase 4: Post-Quantum Security & Control-Flow Integrity (CFI)
   ├── Dilithium-5 PQC digital signature verification for LKM module loading
   ├── Intel CET / ARM GCS hardware shadow stacks against ROP/JOP exploits
   └── Landlock LSM v5 rule hierarchies (`SovereignLandlockV5Guard`)

   Phase 5: eBPF Dynamic Tracing & Autonomous Self-Healing Kernel
   ├── XDP (eXpress Data Path) kernel packet filter engine (`xdp_engine_sovereign.rs`)
   ├── eBPF JIT bytecode compiler (`src/kernel/ebpf_vm.rs`)
   └── Automated kernel panic isolation and live-patching (`breakthroughs.rs`)
```

---

## 3. Detailed Phase Specifications

### Phase 1: VMM & Proactive Memory Compaction
- **PML5 Paging**: Implements 5-level paging structures to expand x86_64 virtual memory addressing from 48-bit (256 TB) to 57-bit (128 PB), enabling high-density enterprise memory configurations.
- **Proactive Compaction**: Adds a background memory compaction thread that defragments physical page frames to maintain pools of contiguous 2MB and 1GB transparent huge pages (THP).
- **Adaptive KSM**: Scans userland memory regions for identical page payloads, merging duplicate pages into single copy-on-write (COW) physical frames.

### Phase 2: Asynchronous `io_uring` SQPOLL & Zero-Copy VFS
- **SQPOLL Kernel Daemon**: Executes an asynchronous kernel worker thread (`IoUringSqpollDaemon`) that continuously polls the `io_uring` submission queue, eliminating user-to-kernel context switch overhead.
- **Fixed Buffers**: Pre-registers application memory buffers with `IORING_REGISTER_BUFFERS`, skipping page pinning and unpinning overhead on every I/O call.
- **Zero-Copy VFS**: Provides kernel-level `splice` pipelines transferring data directly between file descriptors, network sockets, and ring buffers without user-space staging.

### Phase 3: Multi-Arch HAL & ACPI NUMA / IOMMU DMA Isolation
- **ACPI SRAT/SLIT Parsing**: Dynamically parses System Resource Affinity Tables (SRAT) and System Locality Information Tables (SLIT) at boot to map CPU core affinity directly to NUMA memory nodes.
- **IOMMU DMA Isolation**: Configures dedicated IOMMU page table domains for every PCIe endpoint device, blocking unauthorized DMA access to kernel memory space.
- **CXL 3.0 Integration**: Supports CXL 3.0 cache-coherent memory expansion devices as secondary NUMA nodes.

### Phase 4: Post-Quantum Security & Control-Flow Integrity
- **PQC Module Signature Verification**: Enforces Kyber-1024 / Dilithium-5 post-quantum signature validation prior to loading external kernel modules (`src/kernel/module_loader.rs`).
- **Hardware Shadow Stacks**: Integrates Intel CET and ARM GCS shadow stack pointer validation on function call returns, blocking Return-Oriented Programming (ROP) attacks.
- **Landlock LSM v5**: Enforces file path and network port access rules at the kernel boundary (`SovereignLandlockV5Guard`).

### Phase 5: eBPF Dynamic Tracing & Autonomous Self-Healing Kernel
- **eBPF JIT Compiler**: Translates eBPF bytecode directly into native x86_64 / AArch64 machine instructions for high-performance tracing (`src/kernel/ebpf_vm.rs`).
- **XDP Fastpath**: Filters network frames at the NIC driver layer before allocating `sk_buff` buffers (`xdp_engine_sovereign.rs`).
- **Autonomous Self-Healing**: Monitors kernel subsystem health; on soft fault or memory corruption detection, `SelfHealingKernel` isolates the faulty module, applies live patches, and restores valid state without system reboot.

---

## 4. Verification & Testing Strategy
All kernel features are continuously verified using native test suites:
```bash
# Run kernel subsystem tests
cargo test --package sigmaos --lib kernel

# Run native test runner
bash run_sigma_tests.sh
```
