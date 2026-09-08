# AGENTS.md — AI Agent Guidelines, Versioning, Thread Synchronization, HTML Dependency Reduction, Compile-Time Defenses, Clock, Circular Buffers, Cache Memory, Process, Hardware Fitting, Backend, Loading, Processor & Network Management for SigmaOS

This document provides instructions, rules, and procedures for AI agents working in the SigmaOS repository, specifically regarding **Version Handling**, **Release Channels**, **Thread Synchronization & Atomic Operations**, **HTML Dependency Reduction & Text-First Architecture**, **Compile-Time Defenses & Build Hardening**, **Clock & Timer Management**, **Circular Buffer & Ring Buffer Management**, **Cache Memory Management**, **Process Lifecycle & Signal ABI Management**, **Hardware Fitting & Driver Auto-Binding**, **Network Stack & eBPF Management**, **Processor Subsystem Management**, **Backend Management**, **Bootloader & Driver Loading**, **Multi-Distro Packaging Parity**, and **Core Subsystem Changes**.

---

## 1. Core Principles & Philosophy

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
- **Task Management Guidelines**: Refer to `docs/AGENTS_TASK_GUIDELINES.md` for task lifecycle state machines (`TaskState::Pending`, `Running`, `Blocked`, `Completed`, `Evicted`, `Failed`), EEVDF/BORE task scheduling priorities, BSD `kqueue(2)` event loop waits, fine-grained `pledge`/`unveil` sandboxing, cgroups v2 resource envelopes, and atomic task state rollback protocols.
- **Python Dependency Reduction**: Refer to `docs/AGENTS_REDUCING_PYTHON_DEPENDENCY.md` for guidelines on replacing external Python scripts (`scripts/*.py`, `tests/*.py`) with zero-dependency Rust executables (`src/tools/`), POSIX shell scripts (`scripts/*.sh`), or WebAssembly (Wasm) micro-runtimes.
- **Public Launch & Governance**: Refer to `docs/LAUNCH_ANNOUNCEMENT.md` (Launch Manifesto), `docs/WHITEPAPER.md` (Technical Whitepaper), `docs/PRESS_KIT.md` (Press Assets & Media FAQ), and `docs/GOVERNANCE_CHARTER.md` (Contributor Charter).

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

When modifying, releasing, or updating versions in SigmaOS:

### 2.1 Core Repository & Cargo Version
* Core package version is declared in `Cargo.toml` (`version = "0.1.0"`).
* **MAJOR (x.0.0):** Incompatible API/ABI or kernel architecture changes (e.g., breaking KABI stability).
* **MINOR (0.x.0):** New backward-compatible kernel subsystems, drivers, or distro parity features.
* **PATCH (0.0.x):** Backward-compatible bug fixes, performance optimizations, or security patches.

---

## 3. Thread Synchronization & Lock-Free Atomic Rules for AI Agents

When implementing or modifying concurrent thread synchronization:

1. **Explicit Memory Orderings:**
   Always use `Ordering::Release` when publishing shared state updates and `Ordering::Acquire` when reading published state. Do NOT use `Ordering::Relaxed` for synchronized pointer states.
2. **Futex Fast-Path Operations:**
   Userspace lock primitives MUST check atomic lock variables before invoking the kernel futex wait syscall.
3. **Spinlock Backoff:**
   Spinlock loops MUST execute `core::hint::spin_loop()` CPU pause hints to reduce interconnect bus contention.

---

## 4. HTML Dependency Reduction & Text-Based Interface Rules for AI Agents

When creating or modifying documentation, dashboards, or user interfaces:

1. **Text-First & Terminal Preference:**
   Prioritize Markdown (`DocFormat::Markdown`), AsciiDoc (`DocFormat::AsciiDoc`), or ANSI terminal output over HTML web rendering.
2. **HTML Entity Escaping (`escape_html`):**
   If HTML string output is necessary, ALL dynamic string parameters MUST be sanitized via `escape_html` in `src/docs/mod.rs` to neutralize XSS vectors (`<`, `>`, `&`, `"`, `'`).

---

## 5. Compile-Time Defenses & Build Hardening Rules for AI Agents

When modifying build settings, profile options, or feature flags:

1. **`#![no_std]` Zero-Dependency Invariant:**
   Maintain 100% self-sufficient core Rust implementations. Do NOT add external dependencies to `Cargo.toml`.
2. **`panic = "abort"` Unwind Protection:**
   Both `dev` and `release` profiles MUST use `panic = "abort"` to prevent stack unwinding exploit primitives.

---

## 6. Clock Algorithm & Timer Management Rules for AI Agents

When modifying clock page replacement or timekeeping subsystems:

1. **Clock Page Replacement Hand-Pointer Traversal:**
   Page frame eviction MUST traverse physical memory frames in a circular queue. Clear reference bits from `1` to `0` for second-chance evaluation before evicting unreferenced pages.

---

## 7. Circular Buffer & Lock-Free Ring Buffer Rules for AI Agents

When implementing or modifying ring buffers in `src/klib/ring_buffer.rs`, `src/klib/ringbuf.rs`, or `src/media/sovereign_video_player.rs`:

1. **Power-of-Two Capacity Rule:**
   Ring buffer capacities MUST be powers of two ($2^k$) to perform $O(1)$ index wrapping via bitwise AND `idx & (capacity - 1)`.

---

## 8. Cache Memory Architecture, LRU Eviction & Package Cache Rules for AI Agents

When modifying cache memory engines, key-value stores, or package cache trimmers:

1. **Key Invalidation Invariant:**
   `SovereignCacheEngine::set` MUST purge pre-existing entries with matching keys via `self.entries.retain(|e| e.key != key)` before inserting new values.

---

## 9. Process Lifecycle, Signal ABI Translation & Supervision Rules for AI Agents

When modifying process management, signal handlers, or pseudo-terminals:

1. **State Machine Transitions (`SovereignProcessLifecycleController`):**
   Ensure process state changes (`Created`, `Ready`, `Running`, `Blocked`, `Stopped`, `Zombie`, `Terminated`) execute under thread-safe synchronization.

---

## 10. Hardware Fitting, Driver Auto-Binding & Device Adaptation Rules for AI Agents

When writing, probing, or modifying hardware device drivers (`src/drivers/`):

1. **Bus Signature Probing:**
   Driver probe routines MUST evaluate Vendor ID (VID), Product ID (PID), and interface class codes before claiming attachment.

---

## 11. Network Stack, eBPF/XDP & PQC Security Rules for AI Agents

When modifying networking drivers, eBPF filters, or VPN subsystems:

1. **Kernel Bypass eBPF/XDP Processing:**
   Use zero-copy DMA ring buffers (`process_xdp_zero_copy_packet`). Ensure XDP actions explicitly return `XDP_PASS`, `XDP_DROP`, `XDP_TX`, or `XDP_REDIRECT`.

---

## 12. Processor Topology, CPU Scheduling & Multi-Core Rules for AI Agents

When modifying CPU scheduling, task management, or ISA optimization:

1. **ISA Level Auto-Detection (`src/klib/isa.rs`):**
   Support x86-64 microarchitecture levels (`v1`..`v4`). Route vectorized operations via `vectorized_memcpy` dynamically based on detected features.

---

## 13. Kernel, Bootloader & Driver Loading Rules for AI Agents

When modifying boot sequences, driver registration, or scheduler loading:

1. **Multi-Stage Boot Pipeline:**
   Respect the 4-phase boot sequence: Bootloader -> Kernel Initialization -> Dynamic Driver Loading -> Userland Supervisor.

---

## 14. Backend Subsystem & Server Engine Rules for AI Agents

When modifying backend services in `src/open_source_obsoletion.rs`, `src/open_source_os_gap_closure.rs`, or `src/automation/system_level.rs`:

1. **Zero-Dependency Native Backend Engines:**
   Maintain native parity for embedded DBs (`SovereignEmbeddedDb`), web servers (`SovereignWebServer`), in-memory caches (`SovereignCacheEngine`), message brokers (`SovereignMessageBroker`), secret vaults (`SovereignSecretVault`), object stores (`SovereignDistributedStorage`), and orchestrators (`SovereignK8sOrchestratorEngine`).

---

## 15. Explicit Rules for Human Contributors & AI Agents

### 15.1 Universal Rules for Human Contributors
1. **Zero External Dependencies Policy (`no_std`):**
   * Core microkernel shards and kernel subsystems MUST be `#![no_std]` and MUST NOT add external dependencies under `[dependencies]` in `Cargo.toml`.
   * Use native `klib` abstractions (`klib::Vec`, `klib::HashMap`, `klib::BTreeMap`, `klib::String`).
2. **Memory Safety & Unsafe Code Guidelines:**
   * Prefer Safe-Rust. Every `unsafe` block MUST be preceded by a `// SAFETY:` comment documenting safety invariants, pointer alignment, and memory bounds.
3. **Post-Quantum Cryptographic (PQC) Security Standard:**
   * All driver signatures, package manifests, and kernel module attestations MUST use Dilithium-5 signatures or Kyber-1024 KEM.
4. **Sandboxing & Least Privilege:**
   * Userland binaries MUST declare OpenBSD `pledge`/`unveil` rights or FreeBSD Capsicum capabilities before handling untrusted data.
5. **Testing Invariants & Quality Verification:**
   * Every new feature or bugfix MUST include unit tests. Run `cargo check --lib` and `./run_sigma_tests.sh` before submitting pull requests.

### 15.2 Explicit Rules for AI Agents (Autonomous Development)
1. **Planning & Review Protocol:**
   * AI agents MUST call `request_plan_review` with a structured plan before using `set_plan` for the first time.
   * Plans MUST include a dedicated step for verifying test execution and a pre-commit step using the exact phrasing:
     `Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.`
2. **Always Verify Modifications:**
   * After modifying any file, the AI agent MUST confirm the change using a read-only tool (`read_file` or `list_files`).
3. **Code Review & Feedback Implementation:**
   * Before finalizing any PR or submission, the AI agent MUST invoke `request_code_review` and address all blocking feedback.
4. **Memory Recording Directive:**
   * Upon completing code review and verification, the AI agent MUST call `initiate_memory_recording` to document key architectural patterns and learnings.
5. **Secret Scanner False Positive Mitigation:**
   * Test functions or mock credentials MUST use variable names prefixed with `mock_` or `test_` (e.g. `mock_client_secret`) to prevent automated scanner triggers.

---

## 16. Checklist for AI Agents & Contributors

1. **Update Manifests & Documentation** when bumping versions, adding drivers, or modifying synchronization logic.
2. **Run Standalone Subsystem Tests:**
   ```bash
   rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/open_source_os_gap_closure.rs
   ```
3. **Execute Full Pipeline:** Run `./run_sigma_tests.sh` and ensure all test steps pass.
4. **Follow Conventional Commits:**
   `docs(agents): add thread synchronization guide` or `fix(sync): enforce acquire-release memory ordering`.

---

## 16. Detailed Documentation References

For technical specifications, see:
* [`docs/AGENTS_VERSION_HANDLING.md`](docs/AGENTS_VERSION_HANDLING.md)
* [`docs/AGENTS_BACKEND_MANAGEMENT.md`](docs/AGENTS_BACKEND_MANAGEMENT.md)
* [`docs/AGENTS_LOADING_MANAGEMENT.md`](docs/AGENTS_LOADING_MANAGEMENT.md)
* [`docs/AGENTS_PROCESSOR_MANAGEMENT.md`](docs/AGENTS_PROCESSOR_MANAGEMENT.md)
* [`docs/AGENTS_NETWORK_MANAGEMENT.md`](docs/AGENTS_NETWORK_MANAGEMENT.md)
* [`docs/AGENTS_FITTING_MANAGEMENT.md`](docs/AGENTS_FITTING_MANAGEMENT.md)
* [`docs/AGENTS_PROCESS_MANAGEMENT.md`](docs/AGENTS_PROCESS_MANAGEMENT.md)
* [`docs/AGENTS_CACHE_MEMORY_MANAGEMENT.md`](docs/AGENTS_CACHE_MEMORY_MANAGEMENT.md)
* [`docs/AGENTS_CIRCULAR_BUFFER_MANAGEMENT.md`](docs/AGENTS_CIRCULAR_BUFFER_MANAGEMENT.md)
* [`docs/AGENTS_CLOCK_ALGORITHM_MANAGEMENT.md`](docs/AGENTS_CLOCK_ALGORITHM_MANAGEMENT.md)
* [`docs/AGENTS_COMPILE_TIME_DEFENSES_MANAGEMENT.md`](docs/AGENTS_COMPILE_TIME_DEFENSES_MANAGEMENT.md)
* [`docs/AGENTS_REDUCING_HTML_DEPENDENCY.md`](docs/AGENTS_REDUCING_HTML_DEPENDENCY.md)
* [`docs/AGENTS_THREAD_SYNC_MANAGEMENT.md`](docs/AGENTS_THREAD_SYNC_MANAGEMENT.md)
* [`docs/RELEASE_CADENCE.md`](docs/RELEASE_CADENCE.md)
* [`docs/package-manager.md`](docs/package-manager.md)

---

## 17. Linux & BSD Distro Parity Directives

When integrating or refining Linux & BSD distro capability engines in `src/distro/`:
1. **Zero-Dependency Subsystem Parity:** Implement clean-room, `#![no_std]` Rust modules that absorb and emulate key distro innovations (e.g. Void Linux runit service supervision in `VoidRunitServiceSupervisorEngine`, Alpine Linux tmpfs apk volatile overlays in `AlpineApkVolatileOverlayEngine`, openSUSE YaST2/Snapper, NetBSD rump kernels, Ubuntu netplan/cloud-init, GNU Guix Shepherd/store derivations).
2. **Re-export Invariants:** Always re-export newly implemented distro engines in `src/distro/mod.rs` and `src/lib.rs`.
3. **Verification:** Validate all distro parity engines using standalone unit tests (`rustc --test --edition 2021`) and `./run_sigma_tests.sh`.
