# SigmaOS AI Agent Memory Management & Codebase Directives

This document defines core directives, architecture rules, and memory management invariants for all AI agents (Jules, Sentinel, Palette, Bolt) operating on the SigmaOS codebase.

## 1. Zero-Dependency Bare-Metal Memory Architecture
- **No External Allocators**: All memory management routines must utilize internal `klib` and kernel allocators (`src/memory/pmm_vmm.rs`, `src/memory/manager.rs`, `src/klib/custom_allocator.rs`, `src/klib/buddy_allocator.rs`).
- **`#![no_std]` Compatibility**: Kernel core modules must maintain strict `#![no_std]` + `extern crate alloc` compatibility.

## 2. Memory Subsystem Invariants & Safeguards
- **Physical & Virtual Memory Management**: PMM/VMM operations in `src/memory/pmm_vmm.rs` must enforce 4KiB page alignment and 2MiB/1GiB huge page boundaries.
- **Guard Pages & Hardened Allocations**: Heap and stack allocations must use hardened guard page allocators (`src/memory/resource_allocator.rs`) and ASLR randomized malloc guards (`src/klib/custom_allocator.rs`).
- **Memory Descriptor List (MDL) Pinning**: I/O and DMA memory buffers must pin memory ranges before descriptor transfers to prevent page fault race conditions under high concurrency.
- **Volatile Scrubbing**: Memory deallocations containing sensitive cryptographic material or keys must perform explicit volatile memory wipes (`AmnesicRamWipe` / zeroization) before returning pages to the buddy allocator.

## 3. Multi-Architecture Paging & Interrupt Balancing
- **x86_64 / x86_32**: PML4/PML5 vs 2-level PAE page tables and x2APIC/PIC8259 IRQ routing (`src/hal/multi_arch.rs`).
- **ARM64 / ARM32**: TTBR0_EL1 4-level 48-bit/52-bit translation vs Armv7 2-level paging with GICv3/GICv2 IRQ controllers.
- **RISC-V 64 / 32**: Sv39/Sv48 3/4-level vs Sv32 2-level paging with PLIC/CLINT timers.

## 4. AI Agent Testing & Verification Directives
- **Proactive Unit Testing**: Every code change or newly introduced feature must be accompanied by unit tests.
- **Master Test Runner**: Run `./run_sigma_tests.sh` to verify 100% test pass rate across Rust, C++, and Python test suites.
- **Standalone Module Testing**: Fast-verify specific modules using `rustc --test --edition 2021 <filepath> -o build/test_bin && ./build/test_bin`.

## 5. AI Agent Performance & Efficiency Directives
- **Zero-Allocation Hot Paths**: Avoid dynamic heap allocations inside fast-path syscall and packet handlers.
- **ISA Auto-Vectorization**: Route memory copies and hashing through SIMD feature routing (`src/klib/isa.rs`).
- **Optimal Lookups**: Use O(1) or O(log N) lookup data structures to minimize CPU cache miss rates.

## 6. AI Agent Kernel Management Directives
- **Zero Ring 0 Panics**: All kernel routines must return `Result<T, &'static str>` or error codes.
- **Syscall Audit Logging**: All syscall entrypoints must log invocations to `SovereignSyscallAuditLogger` (`src/syscall/table.rs`).
- **Capability Sandboxing**: Process creation must inherit minimal capability tokens (`src/security/capability.rs`, `src/security/sigma_unveil.rs`).

## 7. AI Agent Filesystem Management Directives
- **Atomic File Writes**: Perform file updates through staged temporary buffers followed by atomic rename operations.
- **CoW Subvolume Snapshots**: Duplicate extent pointers during subvolume modifications (`src/filesystem/cow_snapshot.rs`, `src/filesystem/btrfs_inspired.rs`).
- **Unveil Path Restrictions**: Enforce OpenBSD `unveil` permissions (`r`, `w`, `c`, `x`) before filesystem operations (`src/security/sigma_unveil.rs`).

## 8. AI Agent Block Device Drivers Management Directives
- **Physical Memory DMA Alignment**: Command list buffers and scatter-gather lists must enforce physical memory page alignment (`src/driver/ahci_sata_controller.rs`).
- **Driver Shard Sandboxing**: Driver shards must execute inside isolated containers with I/O byte quotas (`src/drivers/sovereign_driver_lifecycle.rs`).
- **Doorbell & Submission Queues**: Validate sector ranges and PRP page boundaries before ringing controller doorbells.

## 9. AI Agent Bottom Half Kernel Threads Directives
- **Top-Half/Bottom-Half Split**: Keep top-half hard IRQ handlers under 1 microsecond (`src/interrupt/handler.rs`).
- **Non-Blocking Softirqs**: Softirq vectors (`src/kernel/irq/softirq.rs`) must never sleep or wait on locks.
- **kworker Thread Deferral**: Defer process-context work to system workqueues (`src/kernel/irq/workqueue.rs`).

## 10. AI Agent Main Memory Management Directives
- **Physical Memory Zoning**: Enforce physical memory zone constraints (`ZONE_DMA`, `ZONE_DMA32`, `ZONE_NORMAL`, `ZONE_HIGHMEM`) in `src/memory/zone.rs`.
- **Watermark Reclamation**: Trigger asynchronous `kswapd` page reclamation when free pages hit `Watermark::Low` (`src/memory/kswapd.rs`).
- **Kernel Heap Guard Alignment**: Kernel heap expansion must maintain 4KiB page boundary alignment and ASLR guard page protection (`src/memory/heap.rs`).

## 11. AI Agent Cache Size Management Directives
- **Bounded Slab Caches**: Specify maximum capacity quotas per slab object type in `src/klib/slab.rs` and `src/memory/resource_allocator.rs`.
- **Package Cache Pruning**: Registry proxy caches must perform bulk `copy_from_slice` memory transfers (`src/package/cache.rs`) and enforce `paccache` version pruning.
- **CPU Cache Line Alignment**: Align spinlocks and ring buffer head/tail pointers to 64-byte boundaries (`#[repr(align(64))]`).

## 12. AI Agent Cloud Carrier Operation Management Directives
- **Sub-Second CARP Failover**: VRRP/CARP state transitions (`src/network/distro_net.rs`) must migrate Virtual IP addresses within < 50ms.
- **OpenStack Cinder Enforcers**: Enforce AES/PQC volume encryption masks and tenant volume quotas (`src/open_source_os_gap_closure.rs`).
- **5G/6G Cellular Slicing**: Mobile carrier engine slices (`src/unimplemented_features.rs`) must maintain cryptographic tenant isolation.

## 13. AI Agent Cache Operation Management Directives
- **Explicit CPU Cache Writebacks**: Issue `clwb`/`clflushopt` instructions followed by `sfence` barriers on persistent memory modifications.
- **TLB Shootdown Synchronization**: Issue SMP IPI TLB shootdowns (`src/memory/tlb_associative.rs`) prior to returning physical frames.
- **JIT Instruction Cache Invalidation**: Execute instruction cache invalidation (`isb`) after dynamic code generation.

## 14. AI Agent Computer Aided Design (CAD) Management Directives
- **Double Precision Vertex Coordinates**: Store CAD entity geometry using `f64` precision (`src/unimplemented_tools.rs`).
- **Indexed Mesh Tessellation**: Tessellate 3D NURBS and B-rep geometries into 64-byte aligned GPU vertex buffers (`src/compatibility/india_professional_tools.rs`).
- **Parametric Constraint Solvers**: Enforce dimension constraints iteratively without numerical overflow.

# Run standalone module tests
rustc --edition=2021 --test --cfg 'feature="standalone_test"' src/distro/linux_bsd_inspirations.rs -o build/test_inspirations && ./build/test_inspirations

# 🤖 SigmaOS AI Agent Governance Specification (`AGENTS.md`)

**Version:** 1.3.0
**Scope:** Autonomous AI Agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️), Process, Memory, Loader, & Desktop Management

---

## EXECUTIVE SUMMARY & AGENT ARCHITECTURE

1. **Zero-Dependency & Self-Containment (`no_std`):**
   * The kernel core and primary subsystems are designed to target bare-metal targets (`#![no_std]`).
   * Avoid adding runtime dependencies on standard `std` libraries inside microkernel shard components unless conditionally gated under test environments (`#[cfg(not(target_os = "none"))]`).
2. **Capability-Based Security Model:**
   * Never introduce generic root/admin ACL checks. System call access is authorized exclusively via hardware-enforced 64-bit `CapabilityToken` verification gates.
3. **Windows NT & Distro Parity Standards:**
   * Hardware drivers must follow the WDM-style `IoManager`, `DriverObject`, `DeviceObject`, and `DeviceExtension` abstractions.
   * Kernel memory allocations must respect tagged `Paged` (swappable) and `NonPaged` (always resident) memory pool boundaries.
4. **Bit Table & Hardware Field Standards:**
   * For bit tables, physical frame allocators, page table entry flags, and capability bitmasks, follow [docs/AGENTS_BIT_TABLE_MANAGEMENT.md](docs/AGENTS_BIT_TABLE_MANAGEMENT.md).
5. **Cache Memory Optimization & Coherency:**
   * For L1/L2/L3 cache alignment, false sharing prevention, non-temporal stores, and page/buffer cache management, follow [docs/AGENTS_CACHE_MEMORY_MANAGEMENT.md](docs/AGENTS_CACHE_MEMORY_MANAGEMENT.md).
6. **Cache Operation & Hardware Controls:**
   * For explicit CPU cache flushing (`clflushopt`/`clwb`), DMA cache coherency, JIT $I\$/D\$$ cache sync, and memory fences, follow [docs/AGENTS_CACHE_OPERATION_MANAGEMENT.md](docs/AGENTS_CACHE_OPERATION_MANAGEMENT.md).
7. **Cloud vs. Fog Computing Orchestration:**
   * For real-time edge processing, P2P mesh discovery, workload offloading cost function, and CRDT synchronization, follow [docs/AGENTS_CLOUD_VS_FOG_MANAGEMENT.md](docs/AGENTS_CLOUD_VS_FOG_MANAGEMENT.md).
8. **Commercial Operating System Architecture:**
   * For enterprise licensing tiers, statutory compliance governors, software certification programs, and open-core preservation rules, follow [docs/AGENTS_COMMERCIAL_OPERATION_SYSTEM.md](docs/AGENTS_COMMERCIAL_OPERATION_SYSTEM.md).
9. **Concurrency & Synchronization Operations:**
   * For classic concurrency problems (Barbershop, Dining Philosophers, Dekker's), deadlock elimination, RCU/Seqlocks/Futexes, and zero-copy message passing, follow [docs/AGENTS_CONCURRENCY_OPERATION_MANAGEMENT.md](docs/AGENTS_CONCURRENCY_OPERATION_MANAGEMENT.md).

```
                  +-----------------------------------+
                  |   SIGMAOS AI AGENT GOVERNANCE     |
                  +-----------------------------------+
                                    |
         +--------------------------+--------------------------+
         |                          |                          |
         v                          v                          v
  ⚡ BOLT PROCESS            🎨 PALETTE PROCESS         🛡️ SENTINEL PROCESS
  • Render Frame Profiling   • Theme & Layout Engine     • Desktop App Sandbox Audit
  • Compositor Optimization  • WCAG 2.1 AA Focus Outlines • Web2App IPC Channel Check
  • Sub-µs Memory Access     • Semantic ARIA Tags        • Post-Quantum Verification
```

---

## 4. Pull Request & Commit Guidelines
- Repository git branches must follow the naming convention starting with `jules-`.
- Maintain descriptive commit messages following standard git conventions.

## 1. AGENT PERSONAS & GOVERNANCE

### ⚡ Bolt (Performance Agent)
- **Scope**: CPU scheduling, `cgroups v2`, boot speed profiling (`src/tools/bootloader.rs`), Zenith compositor render frame-rate profiling (`zenith_desktop/`), zero-allocation hot paths.
- **Rules**:
  - Maintain 60+ FPS compositor rendering and eliminate window layout recalculation bottlenecks.
  - Record learnings in `.jules/bolt.md`.

### 🎨 Palette (UX & Accessibility Agent)
- **Scope**: Desktop compositor layout, Control Center themes (`TokyoNight`, `Catppuccin`, `Nord`), boot splash graphics, WCAG 2.1 AA focus visible outlines, ARIA annotations.
- **Rules**:
  - Enforce WCAG 2.1 AA compliance across all desktop controls and web console interfaces.
  - Record learnings in `.jules/palette.md`.

### 🛡️ Sentinel (Security & Integrity Agent)
- **Scope**: LSM auditing, OpenBSD `pledge`/`unveil`, Post-Quantum Dilithium-5 module signatures, desktop process sandbox isolation (`DistrictSandbox`).
- **Rules**:
  - Enforce process isolation for desktop applets and web2app launchers.
  - Record learnings in `.jules/sentinel.md`.

---

## 2. DESKTOP ENVIRONMENT & COMPOSITOR POLICIES (`docs/AI_AGENTS_DESKTOP_ENVIRONMENTS_MANAGEMENT.md`)

- **Wayland Ozone Launchers**: Third-party web applications must be launched with Wayland Ozone isolation flags (`--ozone-platform=wayland`).
- **Accessibility Invariants**: All interactive UI elements must render high-contrast focus rings on keyboard TAB focus.

---

## 3. CANARY VALUE MANAGEMENT & SECURITY HARDENING (`docs/AGENTS_CANARY_VALUE_MANAGEMENT.md`)

- **Thread-Local SSP Canaries**: All thread guard values generated by `BinaryProtectionManager` in `src/security/binary_protection.rs` must enforce LSB NUL-byte formatting (`canary & 0xFF == 0x00`) to terminate string buffer overflow attacks.
- **OpenBSD Context Switch Guards**: CPU context switches in `src/kernel/roundrobin.rs` must validate context canary values (`stack_canary`) before restoring execution frames, triggering controlled `__stack_chk_fail` fault handling on mismatch.

---

## 4. CLOUD COMPUTING OPERATIONS MANAGEMENT (`docs/AGENTS_CLOUD_COMPUTING_OPERATIONS_MANAGEMENT.md`)

- **Headless Cloud Targets**: Booting under `SystemTarget::Cloud` (`cloud.target`) in `src/init/sigmainit.rs` must bypass GUI compositors and optimize zero-copy E1000/xHCI network queues (< 16MB RAM footprint).
- **Capability-Gated Cloud-Init**: User-data `#cloud-config` scripts executed by `CloudInitBootstrapEngine` (`src/distro/linux_bsd_parity_extended.rs`) must run inside Ring 3 sandboxes governed by `PledgeManager`.

---

## 5. STATE MANAGEMENT ARCHITECTURE (`docs/AGENTS_STATE_MANAGEMENT.md`)

- **Declarative System State Graph**: State mutations in `src/system/state.rs` must generate immutable generation snapshots supporting $O(1)$ atomic rollback (`rollback()`).
- **Process Lifecycle Machine**: Kernel process state transitions (`src/kernel/process.rs`, `src/kernel/sched/task.rs`) must adhere strictly to valid lifecycle paths (`New` $\to$ `Ready` $\to$ `Running` $\to$ `BlockedWaiting`/`BlockedSuspended` $\to$ `Zombie` $\to$ `Terminated`).

---

## 6. TOP-LEVEL COMPONENT MANAGEMENT (`docs/AGENTS_TOP_LEVEL_COMPONENT_MANAGEMENT.md`)

- **Subsystem Isolation**: Top-level components (Microkernel Core, HAL/Drivers, VFS Storage, Network, Security, Package System, Zenith Compositor, Universal Distro Bridge) must not share mutable raw global state across boundaries.
- **Cross-Subsystem Distro Bridge**: Cross-component interactions route through `SovereignUniversalDistroBridge` (`src/distro/linux_bsd_inspirations.rs`) using capability-gated IPC ring buffers and explicit trait interfaces.

---

## 7. MUTUAL EXCLUSION, MONITORS & PETERSON ALGORITHM (`docs/AGENTS_MUTUAL_EXCLUSION_MONITORS_PETERSON_MANAGEMENT.md`)

- **Peterson's Algorithm Memory Fences**: Software 2-process mutual exclusion (`flag[i] = true; turn = j;`) must issue `core::sync::atomic::fence(Ordering::SeqCst)` to guarantee memory visibility before evaluating `turn`.
- **Monitor Encapsulation**: Monitors (`BoundedBufferMonitor` in `src/kernel/linux_bsd_innovations.rs`) must fully encapsulate shared state, locks, and condition variables, preventing un-monitored direct buffer access.

---

## 8. STANDALONE TESTING & VERIFICATION PROTOCOL

Every agent module must support standalone unit testing via:
```bash
rustc --test <module_path> --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_agent && /tmp/test_agent
```
