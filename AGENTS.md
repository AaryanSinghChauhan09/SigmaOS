# SigmaOS AI Agent Security Management Directive (`AGENTS.md`)

This document defines operational guidelines, security policies, and verification instructions for autonomous AI engineering agents working on the SigmaOS codebase.

---

## 1. Core Principles for AI Agents

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
10. **Concurrent Thread Lifecycle & Stack Management:**
   * For SystemThread TCBs, hybrid 1:1 / M:N fiber models, context switching, stack guard pages, and work-stealing thread pools, follow [docs/AGENTS_CONCURRENT_THREAD_MANAGEMENT.md](docs/AGENTS_CONCURRENT_THREAD_MANAGEMENT.md).
11. **Consumable Resource & Depletion Management:**
   * For entropy pools, energy/power budgets, ephemeral ports, backpressure flow control, and consumable IPC signals, follow [docs/AGENTS_CONSUMABLE_RESOURCE_MANAGEMENT.md](docs/AGENTS_CONSUMABLE_RESOURCE_MANAGEMENT.md).
12. **Binary Section Operation & W^X Enforcement:**
   * For ELF/PE section layouts (`.text`, `.rodata`, `.data`, `.bss`, `.got`), page table permission flags, $4\text{ KB}$ alignment, and RELRO lockdown, follow [docs/AGENTS_SECTION_OPERATION_MANAGEMENT.md](docs/AGENTS_SECTION_OPERATION_MANAGEMENT.md).
13. **Disk Operation & Block I/O Schedulers:**
   * For Kyber/BFQ/mq-deadline I/O schedulers, NVMe SQ/CQ multi-queue ring buffers, AHCI SATA PRDT tables, and TRIM/Deallocate discard commands, follow [docs/AGENTS_DISK_OPERATION_MANAGEMENT.md](docs/AGENTS_DISK_OPERATION_MANAGEMENT.md).
14. **Zombie Process Harvesting & Orphan Adoption:**
   * For POSIX `waitpid()` collection (`WNOHANG`), Init PID 1 orphan reaper loops, two-stage resource reclamation, and PCB table leak prevention, follow [docs/AGENTS_ZOMBIE_SYSTEMS_OPERATION_MANAGEMENT.md](docs/AGENTS_ZOMBIE_SYSTEMS_OPERATION_MANAGEMENT.md).
15. **Reducing C++ Dependency & Rust-First Migration:**
   * For migrating legacy C++ sources (`.cpp`/`.hpp`) to zero-dependency `#![no_std]` Rust modules and C-ABI FFI shims (`extern "C"`), follow [docs/AGENTS_REDUCING_CPP_DEPENDENCY.md](docs/AGENTS_REDUCING_CPP_DEPENDENCY.md).
16. **Multi-Architecture Hardware HAL Support:**
   * For multi-target HAL traits (`PlatformHAL`), interrupt domain routing (APIC/GIC/PLIC/ExtIOI), and multi-arch CPU dispatches (x86_64, x86_32, AArch64, AArch32, RISC-V 64/32, LoongArch64, PPC64LE), follow [docs/AGENTS_MULTI_ARCHITECTURE_SUPPORT.md](docs/AGENTS_MULTI_ARCHITECTURE_SUPPORT.md).
17. **Linux & BSD Distribution Parity Guidelines:**
   * For FreeBSD Capsicum/jails/netmap, OpenBSD pledge/unveil/signify, NetBSD Rump kernels, DragonFly HAMMER2, NixOS declarative flakes, and universal package transpilation, follow [docs/AGENTS_DISTRO_PARITY_GUIDELINES.md](docs/AGENTS_DISTRO_PARITY_GUIDELINES.md).

---

## 🏎️ Subsystem Management Protocols for AI Agents

- **Driver Management**: Refer to `docs/AI_AGENT_DRIVER_MANAGEMENT.md` for driver lifecycle directives.
- **Cache Operation Management**: Refer to `docs/AGENTS_CACHE_OPERATION_MANAGEMENT.md` for explicit CPU cache line flushing (`clflush`, `clflushopt`, `clwb`), TLB invalidation/shootdown, Page Cache Radix-Tree operations, SLUB object cache recycling, `#[repr(align(64))]` CPU cache alignment, and JIT instruction cache synchronization rules.
- **Zones Operation Management**: Refer to `docs/AGENTS_ZONES_OPERATION_MANAGEMENT.md` for physical memory zones (`ZONE_DMA`, `ZONE_DMA32`, `ZONE_NORMAL`, `ZONE_HIGHMEM`), FreeBSD UMA (Universal Memory Allocator) zone management (`uma_zcreate`, `uma_zalloc`, `uma_zfree`, `uma_zdrain`), watermark-driven page reclamation, and double-free protection invariants.
- **4-Bit Operation Management**: Refer to `docs/AGENTS_FOUR_BIT_OPERATION_MANAGEMENT.md` for 4-bit nibble packing/unpacking bitwise standards, INT4/NF4 AI model weight quantization (`src/ai/quantization.rs`, `src/ai/voice.rs`), Binary Coded Decimal (BCD) RTC decoding, and 4-bit control register bitfield masking.
- **12-Shard Sovereign Architecture**: Refer to `docs/AGENTS_12_SHARD_SOVEREIGN_ARCHITECTURE.md` for the 12 Safe-Rust microkernel shards taxonomy (Media, Networking, Storage, AI, Compositor, Drivers, Security, Virtualization, System, Package, IPC, Hardware), browser shell integration, Unix primitives for PWAs (`pipe`, `spawn`, `mmap`, `/dev`), and firmware-free driver isolation directives.
- **Component Subsystem Guidelines**: Refer to `docs/AGENTS_COMPONENT_GUIDELINES.md` for Genode-style hierarchical component tree ownership (`src/kernel/component.rs`), parent-child resource delegation, FreeBSD Capsicum FD rights sandboxing, OpenBSD Pledge/Unveil path scoping, Fedora AppStream/modulemd profiles, and component lifecycle supervision.
- **Task Management Guidelines**: Refer to `docs/AGENTS_TASK_GUIDELINES.md` for task lifecycle state machines (`TaskState::Pending`, `Running`, `Blocked`, `Completed`, `Evicted`, `Failed`), EEVDF/BORE task scheduling priorities, BSD `kqueue(2)` event loop waits, fine-grained `pledge`/`unveil` sandboxing, cgroups v2 resource envelopes, and atomic task state rollback protocols.
- **Python Dependency Reduction**: Refer to `docs/AGENTS_REDUCING_PYTHON_DEPENDENCY.md` for guidelines on replacing external Python scripts (`scripts/*.py`, `tests/*.py`) with zero-dependency Rust executables (`src/tools/`), POSIX shell scripts (`scripts/*.sh`), or WebAssembly (Wasm) micro-runtimes.
- **Public Launch & Governance**: Refer to `docs/LAUNCH_ANNOUNCEMENT.md` (Launch Manifesto), `docs/WHITEPAPER.md` (Technical Whitepaper), `docs/PRESS_KIT.md` (Press Assets & Media FAQ), and `docs/GOVERNANCE_CHARTER.md` (Contributor Charter).

---

## 🚗 Driver Management Protocols for AI Agents

### A. Access Control & Sandboxing
- **Landlock v5 + Pledge + Unveil + Capsicum:**
  - File path access must be scoped using `SovereignLandlockV5Guard`.
  - System call promises must be constrained using OpenBSD pledge/unveil enforcers.
  - File descriptors must delegate fine-grained rights via FreeBSD Capsicum.

### B. Cryptographic Integrity & Livepatching
- Post-Quantum Cryptography (Dilithium-5 / Kyber-1024) and Ed25519 signature verification must be enforced for package manifests and livepatching trampolines (`KernelPatchVerificationEngine`).
- Differential rollback snapshots (`SigmaDeltaStateSnapshotEngine`, `SovereignPackageRollbackEngine`) must allow sub-1ms state restoration.

### C. Vulnerability & Audit Auditing
- Maintain vulnerability classification (`Vulnerable`, `Fixed`, `Unaffected`) in `SecurityAdvisoryTracker`.
- Perform QA signoff quorum checks (`PackageSignoff`) requiring `qa_tested`, `build_reproducible`, and `security_audited` flags.

### D. Buffer Overflow & Buffer Overrun Management
- Follow technical directives in `AGENTS_BUFFER_OVERFLOW.md`, `AGENTS_BUFFER_OVERRUN.md`, `docs/AGENTS_BUFFER_OVERFLOW.md`, and `docs/AGENTS_BUFFER_OVERRUN.md`.
- Enforce guard page allocations (`alloc_with_guard_page`), stack clash protection (`has_guard_page`), bounds-checked FFI c-string helpers (`cstrlen`), ring buffers, and W^X / DEP policies.

### E. Bitmap Operations & Resource Allocation
- Follow technical directives in `AGENTS_BITMAP_OPERATIONS.md` and `docs/AGENTS_BITMAP_OPERATIONS.md`.
- Utilize lock-free `AtomicBitmap` for page frames, PIDs, and IRQ vector allocations with atomic memory ordering.

### F. Boot Block & Bootloader Management
- Follow technical directives in `AGENTS_BOOT_BLOCK.md` and `docs/AGENTS_BOOT_BLOCK.md`.
- Ensure `SigmaBootloaderEngine` systemd-boot loader entries and GRUB configs enforce measured boot TPM PCR measurements (`TPM_PCR_4`) and path validation.

### G. Circular Buffer Management & Lock-Free IPC
- Follow technical directives in `AGENTS_CIRCULAR_BUFFER.md` and `docs/AGENTS_CIRCULAR_BUFFER.md`.
- Enforce power-of-two capacity alignment, atomic head/tail pointer ordering (`Acquire`/`Release`), and lock-free bounds checking on `RingBuf` and `RingBuffer`.

### H. Clock Interrupt & Timer Management
- Follow technical directives in `AGENTS_CLOCK_INTERRUPT.md` and `docs/AGENTS_CLOCK_INTERRUPT.md`.
- Ensure clock interrupt handlers avoid blocking locks or allocations, and manage `TimerDescriptor` state transitions atomically.

### I. Coarse Parallelism & Threading Management
- Follow technical directives in `AGENTS_THREADING_PARALLELISM.md` and `docs/AGENTS_THREADING_PARALLELISM.md`.
- Ensure multi-threaded tasks respect RCU synchronization epochs (`rcu_epoch`), adaptive thread quanta (`adaptive_thread_quantum_multiplier`), and stack guard isolation (`has_guard_page`).

### J. Microprocessor Operation Management
- Follow technical directives in `AGENTS_MICROPROCESSOR_OPERATIONS.md` and `docs/AGENTS_MICROPROCESSOR_OPERATIONS.md`.
- Enforce multi-architecture context switching (`CpuContextState`), microarchitecture ISA auto-detection (`x86-64-v1`..`v4`), IRQL execution level guards (`DispatchLevel`), and thermal power governance.

### K. Constrained Application Protocol (CoAP) Management
- Follow technical directives in `AGENTS_COAP_MANAGEMENT.md` and `docs/AGENTS_COAP_MANAGEMENT.md`.
- Ensure IoT CoAP resource endpoints (`CoAPResource`), request methods (`CoAPMethod`), and error codes (`CoAPError`) maintain `#![no_std]` compliance and payload bounds safety.

### L. Control Mode Operation Management
- Follow technical directives in `AGENTS_CONTROL_MODE.md` and `docs/AGENTS_CONTROL_MODE.md`.
- Validate terminal control mode notification parsers (`tmux`), enforce `AccessControlMatrix` rights, and manage remote controller session transitions safely.

### M. Comprehensive Access Operations Management
- Follow technical directives in `AGENTS_ACCESS_MANAGEMENT.md` and `docs/AGENTS_ACCESS_MANAGEMENT.md`.
- Manage the complete access lifecycle across LDAP directory services (`LdapAccessClient`), anonymous/authenticated client tiers, direct/relative path canonicalization, memory access protection (`W^X`), read/write permission enforcers (`FileAttributeAccessControl`), RAT remote files, and wireless access points.

---

## 3. Pre-Commit Verification Checklist for AI Agents

Before submitting changes, AI agents must execute:
1. `./run_sigma_tests.sh` to run 220+ atomic Rust unit tests and Python integration tests.
2. `cargo fmt` to verify code formatting.
3. Validate standalone builds for modified modules (`rustc --edition=2021 --test <modified_file.rs>`).
