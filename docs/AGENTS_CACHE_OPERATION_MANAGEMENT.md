# AI Agent Guidelines: Cache Operation Management in SigmaOS

## 📌 1. Overview & Core Directives

In **SigmaOS**, cache operation management governs explicit, hardware-level CPU cache line flushes, write-backs, invalidations, and instruction-data cache synchronization ($I\$ / D\$$ coherence).

As an AI agent developing microkernel components, device drivers, virtual memory systems, or JIT translators, you must strictly manage hardware cache operations to guarantee **data durability, DMA coherency, and execution safety** across x86_64, AArch64, and RISC-V 64 architectures.

---

## ⚙️ 2. Hardware Cache Operations & ISA Mapping

### 2.1 x86_64 Cache Control Instructions
| Operation | Architecture Instruction | Intrinsic / Assembly | Operational Semantics |
| :--- | :--- | :--- | :--- |
| **Flush Line** | `clflush` / `clflushopt` | `_mm_clflush(ptr)` | Flushes cache line containing `ptr` from all cache levels ($L1/L2/L3$) to DRAM. |
| **Write-Back Line** | `clwb` | `_mm_clwb(ptr)` | Writes back dirty cache line to memory without evicting line from cache ($L1/L2/L3$). |
| **Full Invalidation** | `wbinvd` | `asm!("wbinvd")` | Writes back and invalidates ALL CPU caches (Privileged Ring 0 only; HIGH LATENCY). |
| **Store Fence** | `sfence` | `_mm_sfence()` | Guarantees all preceding stores and cache flushes retire before subsequent stores. |
| **Full Fence** | `mfence` | `_mm_mfence()` | Serializes all load and store operations across the memory pipeline. |

### 2.2 AArch64 Cache Control Operations
* **Data Cache Clean (Write-Back) to PoC (Point of Coherency):** `dc cvac, xt`
* **Data Cache Clean & Invalidate:** `dc civac, xt`
* **Instruction Cache Invalidation to PoU (Point of Unification):** `ic ivau, xt`
* **Barrier Synchronization:** `dsb ish` (Data Synchronization Barrier), `isb` (Instruction Synchronization Barrier)

### 2.3 RISC-V 64 Cache Operations (Zicbom / Zicboz Extension)
* **Clean Block:** `cbo.clean`
* **Flush Block:** `cbo.flush`
* **Invalidate Block:** `cbo.inval`
* **Instruction Cache Fence:** `fence.i`

---

## 🛡️ 3. Key Subsystem Cache Operation Patterns

### 3.1 DMA Buffer Coherency
When preparing a buffer for non-cache-coherent PCIe or AHCI DMA devices:
1. **Pre-DMA Transmit (CPU Write $\rightarrow$ Device Read):**
   * Perform $D\$$ clean/write-back for all 64-byte cache lines covering the buffer range.
   * Issue `sfence` / `dsb ish` before signaling device doorbell register.
2. **Post-DMA Receive (Device Write $\rightarrow$ CPU Read):**
   * Invalidate CPU $D\$$ lines covering target buffer to force subsequent CPU reads to fetch fresh data from DRAM.

### 3.2 Self-Modifying Code & eBPF JIT Cache Sync
When generating dynamic machine code in memory before execution:
1. Write generated instructions to target memory buffer.
2. Flush/Clean Data Cache line ($D\$$ write-back): `clwb` / `dc cvau`.
3. Issue Store Fence: `sfence` / `dsb ish`.
4. Invalidate Instruction Cache ($I\$$ invalidate): `ic ivau` / `fence.i`.
5. Issue Pipeline Instruction Barrier: `isb`.
6. Execute function pointer.

### 3.3 Persistent Memory & NVDIMM Durability
For persistent memory writes (`src/filesystem/ext4.rs`, `src/filesystem/btrfs_inspired.rs`):
```rust
pub unsafe fn flush_persistent_range(ptr: *const u8, len: usize) {
    let mut addr = ptr as usize & !63; // Align to 64-byte boundary
    let end = (ptr as usize + len + 63) & !63;

    while addr < end {
        #[cfg(target_arch = "x86_64")]
        core::arch::x86_64::_mm_clwb(addr as *const _);

        #[cfg(target_arch = "aarch64")]
        core::arch::aarch64::__dc_cvac(addr as *const _);

        addr += 64;
    }

    #[cfg(target_arch = "x86_64")]
    core::arch::x86_64::_mm_sfence();
}
```

---

## 🚫 4. AI Agent Safety Rules for Cache Operations

1. **Avoid `wbinvd` in Hot Paths:**
   * Never execute `wbinvd` inside kernel interrupt handlers or system call dispatchers. It stalls all execution pipelines for up to several milliseconds.
2. **Align Range Flushes to 64 Bytes:**
   * Always round start address down (`addr & !63`) and end address up (`(addr + len + 63) & !63`) when flushing cache ranges to prevent partial line misses.
3. **Always Pair Non-Temporal Stores with Fences:**
   * Every non-temporal streaming store (`_mm_stream_si128`) or `clwb` flush sequence MUST conclude with an `sfence` / `dsb ish` before returning or notifying external hardware.

---

## 🧪 5. Verification & Standalone Testing Procedures

AI agents can verify cache operation helper routines via standalone unit compilation:

```bash
# Test memory manager & performance allocator stack (includes cache flushing helpers)
rustc --test --edition=2021 src/kernel/perf_mm.rs -o build/perf_mm_tests && ./build/perf_mm_tests && rm build/perf_mm_tests

# Test eBPF JIT translator & code cache synchronization
rustc --test --edition=2021 src/kernel/linux_bsd_innovations.rs -o build/ebpf_tests && ./build/ebpf_tests && rm build/ebpf_tests
```
