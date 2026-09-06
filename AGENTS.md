# AI Agent Directives & Memory Management Guide for SigmaOS

Welcome, AI Engineer / Agent! This document specifies core operational guidelines and memory management procedures for working with the SigmaOS codebase.

---

## 🧠 Memory Management Principles in SigmaOS

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

---

## 🎨 UI Management & Accessibility Principles

1. **Native WASM / Rust UI Engine First**
   - Implement UI event handlers, keyboard focus, and ARIA attributes in native Rust/WASM (`NativeWasmDesktopEngine` in `src/desktop/web_wasm_bridge.rs` & `zenith_desktop/src/lib.rs`). Reduce or eliminate JavaScript runtime dependencies.

2. **Accessibility Standards (Palette Persona)**
   - Icon-only buttons **must** include an `aria-label`.
   - Support keyboard navigation (`Enter` and `Space` key activation) and focus states (`:focus-visible` / `.keyboard-focus`).
   - Use `set_secure_text_content` (`textContent`) to prevent innerHTML XSS vulnerabilities when rendering dynamic titles or strings.

3. **CSS Design Tokens**
   - Leverage theme custom properties defined in `zenith_desktop.css` (`--accent-gold`, `--accent-blue`, `--accent-cyan`).

---

## 🌐 Network Management & Security Validation Principles

1. **IPv4 & IPv6 Address Parsing Security (`src/security/input_validation.rs`)**
   - In `validate_ipv4`: Reject leading zeros in multi-digit octets (e.g. `010.0.0.1`) to prevent octal parser differential SSRF vulnerabilities.
   - In `validate_ipv6`: Track explicit block count alongside `double_colon` compressed blocks. Reject addresses with 8 or more explicit blocks (`blocks >= 8`) when compressed notation is used.

2. **Cross-Platform Firewall Translation (`src/network/`)**
   - Provide interoperable rule translation across OpenBSD `pf` (`pf_firewall.rs`), Linux `nftables` (`nftables.rs`), and NetBSD `npf` (`npf_firewall.rs`).
   - Enforce FreeBSD VNET per-jail network stack isolation (`distro_net.rs`).

---

## 🔄 System State Management Principles

1. **Declarative State Graph (`src/system/state.rs`)**
   - Use NixOS-inspired `DeclarativeStateGraph` to manage system configuration nodes (`StateNode`).
   - Call `validate()` before committing state transitions to verify dependency IDs exist.

2. **Atomic Generation Rollback**
   - Trigger `create_generation` before performing major configuration updates, enabling instant atomic rollback (`rollback()`) on failure.

3. **MVI Reactive Store (`src/klib/store.rs`)**
   - For UI and userland reactivity, dispatch immutable actions through `StateStore<S, A>` and pure `Reducer<S, A>` functions.

---

## 🔒 Spinlock System Synchronization Principles

1. **Ticket Spinlock Fairness (`TicketSpinlock` in `src/kernel/classic_os.rs`)**
   - Use atomic ticket/now_serving counters with exponential backoff (`core::hint::spin_loop()`) to ensure SMP lock fairness.

2. **Fine-Grained Contention Tracking (`FineGrainedSpinlock` in `src/kernel/core/sovereign_scheduler.rs`)**
   - Track `acquire_count` and `contention_count` for latency diagnostics (FreeBSD `mtx` & Linux `spinlock_t` parity).

3. **Deadlock & Interrupt Safety**
   - Never perform dynamic memory allocation or blocking operations while holding a spinlock.
   - Acquire multiple spinlocks in strict ascending hierarchical order.

---

## 💾 Block-Oriented Device Management Principles

1. **Unified Device Abstraction (`src/storage/block.rs`)**
   - Implement `BlockOrientedDevice` for block drivers (`SsdBlockDevice`, `NvmeBlockDevice`).
   - Check `dev.is_write_blocked()` before all destructive operations (`Write`, `DiscardTrim`, `SecureErase`).

2. **Buffer Alignment & Bound Checks**
   - Ensure read/write buffers match `dev.block_size()` and verify `block_num < dev.total_blocks()` to prevent out-of-bounds access (`BlockError::OutOfBounds`).

3. **Cache & Partition Synchronization**
   - Invalidate matching blocks in `SimpleBlockCache` when performing discard or secure erase ops.

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

## 8. CONCURRENT PROCESS MANAGEMENT (`docs/AGENTS_CONCURRENT_PROCESS_MANAGEMENT.md`)

- **Atomic PCB State Machine**: PCB state transitions in `src/kernel/process.rs` and `src/kernel/sched/task.rs` must update atomically without lock contention races across CPU cores.
- **Zombie Child Reaping & Signal Safety**: Child processes entering `ProcessState::Zombie` must support `waitpid()` exit status reclamation; forceful signal cancellation (`SIGKILL`) must automatically release held spinlocks and file locks to prevent deadlocks.

---

## 9. CACHE OPERATION MANAGEMENT (`docs/AGENTS_CACHE_OPERATION_MANAGEMENT.md`)

- **DMA Cache Line Flushing**: Memory buffers used for hardware DMA transfers on non-coherent buses must execute explicit `clflushopt` or `clwb` cache line flushes (`src/kernel/mm/cpu_cache.rs`).
- **TLB Invalidation & Shootdowns**: Unmapping or modifying page table entries in `src/memory/tlb_associative.rs` must execute `invlpg` and broadcast multicore TLB shootdown interrupts.

---

## 10. STANDALONE TESTING & VERIFICATION PROTOCOL

1. **Stream-Oriented Line Disciplines (`src/kernel/tty.rs`)**
   - Support canonical mode editing (`ICANON`), signal interjection (`ISIG`), and software flow control (`IXON`/`IXOFF`).

2. **Termios Signal & Echo Flushing**
   - Flush input buffers (`flush_input()`) upon processing signal bytes (`VINTR`, `VQUIT`, `VSUSP`) when `ISIG` is active.

3. **Lock-Free Hardware FIFO Operations (`src/kernel/drivers/legacy/uart_8250.rs`)**
   - Maintain zero heap allocations during high-frequency character transfer interrupt routines.

---

## 🔌 Device Classes & Operation Management Principles

1. **Unified Device Traits (`src/driver/device.rs`)**
   - Implement `Device` and `UnifiedPeripheral` traits for driver classes (`Block`, `Character`, `Network`, `Graphics`, `Input`, `Audio`).

2. **MMIO Volatile Register Safety**
   - Access memory-mapped channel registers (`PortAddress::MemoryMapped`) exclusively via volatile primitives (`read_volatile`/`write_volatile`).

3. **Driver Object & Extension Lifecycles (`IoManager`)**
   - Track device reference counts (`increment_ref`/`decrement_ref`) and release all `DeviceExtension` context buffers upon driver unload (`io_unload_driver`).

---

## ⚙️ Testing & Verification Procedures

- **Kernel Primitives (`klib`):**
  `cargo test --lib -- klib::json` or `./run_sigma_tests.sh`
- **Linux/BSD System Gap Engines:**
  `rustc --test src/distro/linux_bsd_distro_gaps.rs --edition=2021 -o build/distro_gaps_test && ./build/distro_gaps_test`
- **Python Integration Suite:**
  `pytest tests/test_unit_core.py tests/test_integration_system.py tests/test_stress_fuzz_bench.py`
