# SigmaOS AI Agent Instructions (`AGENTS.md`)

Welcome, AI Agent! This file contains repository-specific directives, architectural rules, coding standards, and testing procedures for working on **SigmaOS**.

---

## 1. Primary Directives & Code Conventions

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

---

## 2. Universal Package Management Guidelines for AI Agents (`SigmaPkg`)

When modifying or interacting with the package management subsystem (`src/package/`, `src/sigpkg/`):

1. **Universal Package Formats:**
   SigmaOS natively supports Linux, BSD, and Unix package formats:
   - Linux: `.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.moss`, `.hpkg`, `.tcz`, `.ipk`, `.opkg`, `.xbps`, `.eopkg`
   - BSD/UNIX: `.txz` (FreeBSD), `.openbsd.tgz` (OpenBSD), `.pkgsrc` (NetBSD), `.p5p` / `.ips` (Solaris), `.nar` (Guix/Nix)
   - App Bundles: `.flatpak`, `.appimage`, `.snap`

2. **Core Structs & APIs:**
   - `UniversalPackageManager` (`src/sigpkg/universal_oop_system.rs`): Handles package database state, installation, and removal.
   - `UniversalPackageAdapter` (`src/sigpkg/universal_adapter.rs`): Maps format-specific metadata to `StandardPackage`.
   - `UniversalDependencyMapper` (`src/sigpkg/universal_adapter.rs`): Maps cross-distro package dependency names (e.g. `python3-dev` -> `python`).
   - `UniversalDryRunSimulator` (`src/sigpkg/universal_adapter.rs`): Simulates installs to verify filesystem conflicts and missing dependencies before committing changes.

3. **Safety & Sandboxing Rules:**
   - Scriptlets (`pre-install`, `post-install`, `triggers`) MUST execute inside `pledge`/`unveil` or `Landlock`/`AppArmor` sandboxes (`src/package/sandbox.rs`).
   - All package operations MUST support atomic CoW snapshot rollbacks (`src/package/updater.rs`).

---

## 3. Interface Management Guidelines for AI Agents

When interacting with Network, Display/UI, and Hardware Driver Interfaces:

1. **Interface Subsystems:**
   - **Network Interfaces (`src/net/`, `src/network/`):** `SimpleNetworkStack`, `NetworkInterface`, `MacAddress`, `IPv4Address`, eBPF filters, and WireGuard VPN tunnels.
   - **Display & Graphical UI Interfaces (`src/graphics/`, `src/desktop/`, `src/ui/`):** Zenith Wayland layer-shell compositor, HiDPI scaling, GTK3/4 `GtkHeaderBar`/`GtkBox` UI toolkit, and Control Center subpanels.
   - **Hardware Driver & Bus Interfaces (`src/hal/`, `src/driver/`, `src/device/`):** `SovereignDriver` lifecycle, PCI ECAM bus, USB XHCI controllers, and hot-swappable driver shards.

2. **CLI Commands for AI Agents:**
   - Network: `sigma-net link list --json`, `sigma-net ip addr add`, `sigma-net wifi connect`
   - UI/Desktop: `zenith-ctl display info --json`, `zenith-ctl theme set`
   - Hardware Drivers: `sigma-driver list --json`, `sigma-driver reload <driver>`

3. **Safety & PolicyKit Constraints:**
   - Network interface changes require PolicyKit authorization `org.sigmaos.network.configure`.
   - Driver shard loading requires PolicyKit authorization `org.sigmaos.driver.load`.
   - Refer to `docs/ai_agents_interface_management.md` for full developer documentation.

---

## 4. Spinlock Management Guidelines for AI Agents

When interacting with kernel concurrency, atomic state, and scheduler synchronization:

1. **Spinlock Types:**
   - `FineGrainedSpinlock` (`src/kernel/core/sovereign_scheduler.rs`): Atomic CAS locking with fine-grained contention tracking (`lock_count`, `spin_cycles`).
   - `SpinMutex` (`src/system/state.rs`): Used for global kernel configuration access where sleeping is prohibited.
   - Atomic RingBuffer Spinlock (`src/klib/ringbuf.rs`): Atomic `0/1` lock for MPMC ringbuffers.
   - WDK Driver Spinlock (`src/kernel/wdk_core.rs`): IRQL DISPATCH_LEVEL execution bounded spinlocks.

2. **Core Directives & Safety:**
   - Never sleep or block while holding a spinlock.
   - Busy-wait loops MUST execute `core::hint::spin_loop()` for CPU pipeline efficiency.
   - Always enforce strict lock ordering to prevent deadlocks across SMP cores.
   - Refer to `docs/ai_agents_spinlocks_management.md` for full developer documentation.

---

## 5. Block Device Driver Guidelines for AI Agents

When interacting with block storage hardware, NVMe/SATA drivers, VirtIO disks, and SCSI controllers:

1. **Core Interfaces & Traits:**
   - `BlockDevice` (`src/driver/device.rs`): Fundamental block read/write interface (`read_block`, `write_block`, `block_size`, `total_blocks`).
   - `BlockDeviceDriver` (`src/driver/framework.rs`): Multi-block batch I/O driver trait.
   - `VirtioBlockShim` (`src/driver/shims.rs`): VirtIO paravirtualized block queue shim.
   - `StorportDriver` (`src/driver/windows_compat.rs`): SCSI CDB miniport adapter.

2. **Core Directives & Safety:**
   - Always validate sector alignment and check bounds (`block < total_blocks`).
   - Honor forensic write-blocking flags (`UsbStorageFilterDriver`).
   - Refer to `docs/ai_agents_block_device_drivers.md` for full developer documentation.

---

## 6. Cache Levels & Memory Hierarchy Guidelines for AI Agents

When interacting with CPU hardware cache, virtual memory page cache, VFS dcache, or package caches:

1. **Cache Hierarchy Tiers:**
   - **CPU Hardware Cache** (`src/kernel/mm/cpu_cache.rs`): L1I/L1D, L2, L3 caches with MESI coherence and PLRU/LRU eviction.
   - **Kernel Page Cache** (`src/kernel/mm/page_cache.rs`): Caches block storage sectors in RAM pages (`CachedPage`) with dirty page syncing.
   - **Kernel Object Slab Cache** (`src/kernel/mm/slab_allocator.rs` & `src/klib/slab.rs`): Reuses pre-allocated kernel object slots (`dentry`, `inode`).
   - **Package Cache** (`src/package/paccache.rs`): Manages downloaded package tarball retention and version trimming (`PaccacheEngine`).

2. **Core Directives & Safety:**
   - Always flush dirty pages in `PageCache` before unmounting filesystems.
   - Align critical concurrency structures to 64-byte cache line boundaries (`#[repr(align(64))]`).
   - Refer to `docs/ai_agents_cache_levels.md` for full developer documentation.

---

## 7. Cache Block Size & Line Management Guidelines for AI Agents

When interacting with CPU cache block transfers, cache line alignment, and vectorized memory copies:

1. **Cache Block Parameters:**
   - Standard Cache Line/Block Payload: 64 bytes (`data: [u8; 64]` in `src/kernel/mm/cpu_cache.rs`).
   - Bitwise Address Shift: `addr >> 6` block address decomposition.
   - Cache Line Detection API: `detect_cache_line_size()` in `src/arch/cpu_features.rs`.

2. **Core Directives & Safety:**
   - Align high-frequency atomic spinlocks and per-CPU counters to 64-byte boundaries to prevent false sharing.
   - Structure high-throughput zero-copy DMA and memory loops in 64-byte chunks.
   - Refer to `docs/ai_agents_cache_block_size.md` for full developer documentation.

---

## 8. Child Process Management Guidelines for AI Agents

When spawning, managing, or reaping child processes:

1. **Child Process Lifecycle:**
   - **Forking:** Process duplication via `Process::fork()` in `src/kernel/process.rs` inherits PPID, PGID, UID/GID, open file descriptors, signal masks/actions, and working directory.
   - **State Transitions:** Process states (`New`, `Ready`, `BlockedWaiting`, `BlockedSuspended`, `Zombie`).
   - **Harvesting & Reparenting:** Exit codes harvested via `waitpid()`. Terminated process orphans automatically reparented to `init` (`PID 1`).

2. **Core Directives & Safety:**
   - Always reap child process exit codes to prevent process table accumulation.
   - Ensure child processes receive isolated physical page tables (`new_pt_phys`).
   - Refer to `docs/ai_agents_child_process_management.md` for full developer documentation.

---

## 9. GitHub Actions CI Directives

1. **Action References Pinning:**
   - All GitHub Action references in `.github/workflows/` MUST be pinned to 40-character commit SHAs or valid tags.
   - `actions/download-artifact` MUST be pinned to `cc203385981b70ca67e3a982f6e5f6e62f59a86e` to remediate GHSA security vulnerabilities.
2. **Rust Toolchain Step Syntax:**
   - When using `uses: dtolnay/rust-toolchain@v1` (or `@stable`), always supply a `with:` block containing `toolchain: stable` (or `nightly`) with correct 8-space step key indentation under `- uses:`.

---

## 10. Documentation & Wiki Synchronization

When updating documentation or roadmap files in `docs/` or `WIKI/`, always run `./scripts/sync_wiki.sh` to keep `wiki/` and `wiki_repo/` documentation mirrors in sync.

---
*Maintained by the SigmaOS Core Architecture Team.*
