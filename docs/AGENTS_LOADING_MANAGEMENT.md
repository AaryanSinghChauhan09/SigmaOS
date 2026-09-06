# SigmaOS Bootloader, Module & Workload Loading Management Guide for AI Agents

This guide provides technical specifications, boot sequences, dynamic binary loading procedures, driver dependency resolution, and workload load balancing guidelines for AI agents managing loading subsystems in SigmaOS.

---

## 1. Zero-Dependency Boot & Kernel Loading Sequence

SigmaOS features a multi-stage `#![no_std]` boot and binary loading pipeline:

1. **Phase 1: Bootloader & Stage-1 Initialization (`bootloader.md`):**
   * Pre-kernel boot stage validates system topology (x86_64, AArch64, RISC-V).
   * Sets up initial 4-level paging memory maps (`SimpleVMM` in `src/klib/paging.rs`).
   * Passes physical memory map descriptors (`boot_memory_map`) to the kernel physical memory manager (`bitmap_pmm.rs`).
2. **Phase 2: Core Kernel Loading (`src/kernel/main.rs`):**
   * Initializes zero-copy IPC channels, syscall table (`SyscallTable` in `src/kernel/syscall/table.rs`), and anti-rootkit shadow SSDT guard (`AntiRootkitGuard`).
   * Initializes scheduler worker queues and BORE / EEVDF scheduling policies (`src/scheduler/process.rs`).
3. **Phase 3: Kernel Module & Dynamic Driver Loading:**
   * Dynamic device drivers (`distro_device_expansion.rs`) and external modules are registered via dependency DAG resolution.
   * Drivers are dynamically bound to bus devices (PCIe, USB BOT, Bluetooth HCI, Thunderbolt DP Alt-Mode).
4. **Phase 4: Userland & Universal Package Loading:**
   * Init supervisor (`SystemdInit` / `SovereignRunitSupervisor`) initializes system services.
   * Universal package manager (`sigpkg`) loads binary executables and translates foreign package formats.

---

## 2. Dynamic Driver & eBPF Lazy Loading Rules

When managing or modifying kernel drivers and dynamic code loading:

### 2.1 Driver Loading Dependencies
* Drivers MUST specify explicit bus dependency requirements (`Pcie`, `Usb`, `Bluetooth`, `Thunderbolt`).
* Driver probe functions MUST return early if hardware signatures or vendor IDs do not match during device discovery.

### 2.2 eBPF Program Loading & JIT Verification (`src/scheduler/ebpf_scheduler.rs`)
* eBPF scheduler programs (`BpfProgram`) loaded into kernel space MUST be verified for memory safety and termination bounds before execution.
* Ensure atomic flags in `BpfProgram` use atomic load/store operations (`AtomicBool`) during module duplication or thread cloning.

---

## 3. Workload Load Balancing & Performance Scaling Laws

SigmaOS integrates mathematical workload performance scaling models in `src/performance/scaling_laws.rs`:

* **Parallel Thread Scaling (Amdahl's Law - `AmdahlScalingModel`):**
  Calculates maximum theoretical speedup based on serial execution fraction $s$ and thread count $N$:
  $$\text{Speedup} = \frac{1}{s + \frac{1-s}{N}}$$
* **Scaled Work Growth (Gustafson's Law - `GustafsonScalingModel`):**
  Evaluates scaled speedup when problem size expands proportionally with core count.
* **Contention & Coherency Overhead (Universal Scalability Model - `UniversalScalabilityModel`):**
  Models contention parameter $\alpha$ and inter-core latency parameter $\beta$ to prevent thread saturation degradation.
* **Queueing & Concurrency Limits (Little's Law - `LittleQueueModel`):**
  Maintains system concurrency $L = \lambda \cdot W$ to bound queueing delays in userland servers and kernel thread pools.

---

## 4. Checklist for AI Agents Managing Loading Subsystems

1. **Validate Memory Allocation:** Verify that page table mappings maintain 4-level PML4 indexing (`pml4_idx * 512 + pdpt_idx`).
2. **Check Driver Registration:** Ensure newly introduced drivers in `src/drivers/` are registered in `src/drivers/mod.rs` and re-exported in `src/lib.rs`.
3. **Test Loading Pipelines:**
   Run kernel scheduler and memory manager inspection tests:
   ```bash
   cargo test --lib -- kernel::syscall::table::tests
   ./run_sigma_tests.sh
   ```
