# Cache Memory Block Size & Line Management Guidelines for AI Agents (`docs/ai_agents_cache_block_size.md`)

This document provides AI agents with directives, architectural standards, Rust types, and safety rules for managing **Cache Line and Memory Block Sizes** across the SigmaOS CPU cache simulation and architecture features.

---

## 1. Overview of Cache Block Size in SigmaOS

In modern high-performance microarchitectures and the SigmaOS CPU cache simulator (`src/kernel/mm/cpu_cache.rs`), memory is transferred between main memory (DRAM) and CPU cache hierarchies (L1I, L1D, L2, L3) in fixed-size contiguous chunks called **Cache Blocks** or **Cache Lines**.

Key parameters:
* **Standard Cache Block Size:** 64 bytes (`[u8; 64]`).
* **Bitwise Address Shift:** `addr >> 6` (since $2^6 = 64$).
* **Auto-Detection API:** `detect_cache_line_size()` in `src/arch/cpu_features.rs` dynamically queries CPUID / system registers to detect hardware cache line size (defaulting to 64 bytes).

---

## 2. Core Block Structures & Address Decomposition

### 2.1 Cache Line Structure (`src/kernel/mm/cpu_cache.rs`)
In the kernel's CPU cache model, every cache line consists of metadata and a 64-byte payload:

```rust
pub struct CacheLine {
    pub tag: u64,
    pub state: MesiState,   // Modified, Exclusive, Shared, Invalid
    pub mru_bit: bool,      // Pseudo-LRU bit
    pub data: [u8; 64],     // 64-byte payload block
}
```

### 2.2 Address Decomposition Algorithm
To locate a byte address `addr` inside the cache hierarchy, the address is decomposed using bitwise operations:

1. **Absolute Block Address:**
   ```rust
   let block_addr = addr >> 6; // Divides address by 64
   ```
2. **Set Index Calculation:**
   ```rust
   let set_index = (block_addr as usize) % self.num_sets;
   ```
3. **Tag Extraction:**
   ```rust
   let tag = block_addr >> num_set_bits;
   ```

---

## 3. Cache-Line Aligned Memory Transfers (`src/arch/cpu_features.rs`)

When performing high-throughput memory copies or vector operations in `cpu_features.rs`, SigmaOS chunks transfers into cache-line aligned blocks:

```rust
pub fn detect_cache_line_size(&self) -> usize {
    self.cache_line_size.load(Ordering::Relaxed) // Defaults to 64
}

pub fn optimized_memcpy(&self, dst: &mut [u8], src: &[u8]) {
    let cache_line = self.detect_cache_line_size();
    if src.len() >= cache_line {
        let chunks = src.len() / cache_line;
        for i in 0..chunks {
            let start = i * cache_line;
            let end = (i + 1) * cache_line;
            // Transfer 64-byte cache block chunk...
        }
    }
}
```

---

## 4. Eviction & Dirty Block Writeback

When a cache miss occurs and all lines in a set are occupied, the cache controller selects a victim line for eviction:

* **Clean Line Eviction:** If `state` is `Shared` or `Exclusive`, the line is discarded or unlinked.
* **Dirty Line Writeback:** If `state` is `Modified`, the 64-byte `data` block is evicted down the hierarchy (L1D $\rightarrow$ L2 $\rightarrow$ L3 $\rightarrow$ DRAM) before being overwritten by the new block.

---

## 5. Directives & Safety Rules for AI Agents

When designing algorithms, kernel structures, or memory routines:

1. **Align High-Frequency Concurrency Locks:**
   Align spinlocks, atomic counters, and per-CPU data structures to 64-byte boundaries (`data: [u8; 64]`) to prevent **false sharing** across SMP CPU cores.
2. **Process Memory in 64-Byte Chunks:**
   When writing vectorized memory loops or DMA buffers, structure inner loops to operate on multiples of 64 bytes (`detect_cache_line_size()`).
3. **Preserve Block Indexing Alignment:**
   When calculating block indices, always shift address bits by 6 (`addr >> 6`).

---

## 6. Verification & Testing Procedure

1. **Run CPU Cache & Features Unit Tests:**
   ```bash
   cargo test --lib kernel::mm::cpu_cache
   cargo test --lib arch::cpu_features
   ```

2. **Run Full Kernel Test Suite:**
   ```bash
   ./run_sigma_tests.sh
   ```

---
*Maintained by the SigmaOS Core Architecture & Memory Team.*
