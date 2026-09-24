# AI Agent Motivation & Guidelines: Cache Memory Management in SigmaOS

## 💡 1. Motivation & Core Architectural Principles

In modern high-performance microkernel design, **memory latency is the ultimate system bottleneck**. While CPU clock speeds execute billions of instructions per second, main system RAM access ($DRAM$) incurs a latency penalty of $150\text{--}200$ CPU clock cycles. In contrast:
* **L1 Data Cache ($L1d$):** $4\text{--}5$ cycles ($\approx 32\text{--}64\text{ KB}$ per core)
* **L2 Unified Cache ($L2$):** $12\text{--}14$ cycles ($\approx 512\text{ KB}\text{--}1\text{ MB}$ per core)
* **L3 Shared Cache ($L3 / LLC$):** $40\text{--}60$ cycles ($\approx 16\text{--}64\text{ MB}$ shared)
* **System DRAM:** $150\text{--}200+$ cycles

As an AI agent working on **SigmaOS**, your motivation when writing kernel, subsystem, or driver code is to **maximize L1/L2 cache hit ratios ($>98\%$)** and **eliminate false sharing and cache thrashing**.

---

## ⚙️ 2. Cache Memory Architecture in SigmaOS

### 2.1 Hardware CPU Cache Coherency & Line Alignment
* **Cache Line Standard:** All cache lines on supported x86_64, AArch64, and RISC-V 64 architectures in SigmaOS are **64 bytes**.
* **Cache Line Alignment (`#[repr(C, align(64))]`):**
  * Hot per-core data structures (such as per-CPU scheduler runqueues, lock-free ring buffer heads/tails, and core activity states) must be aligned to 64-byte boundaries to prevent **false sharing** across SMP CPU cores.
* **Hot/Cold Field Separation:**
  * Separate frequently accessed fields (e.g., `state`, `head`, `tail`, `lock`) from rarely accessed cold fields (e.g., `debug_name`, `audit_trail`, `creation_time`).

### 2.2 Non-Temporal Stores & Prefetching
* **Streaming Stores (`MOVNTDQ` / Non-Temporal Stores):**
  * For bulk memory operations (DMA buffers, framebuffers, page zeroing), use non-temporal stores to write directly to DRAM, bypassing $L1/L2/L3$ pollution.
* **Software Prefetching:**
  * On predictable sequential or strided data traversals, issue software prefetch hints (`_mm_prefetch`, `prfm`) $2\text{--}4$ iterations ahead of consumption.

### 2.3 VFS Page Cache & Buffer Cache
* **Module Location:** `src/filesystem/cache.rs`, `src/filesystem/support.rs`
* **Eviction Model:** Segmented LRU (SLRU) split into *Probationary* and *Protected* segments to prevent single-pass scans from thrashing active pages.
* **Zero-Copy Splice Bypass:**
  * Network packet transfers and IPC pipes bypass the page cache entirely via DMA descriptors and page pin splicing.

### 2.4 Translation Lookaside Buffer (TLB) Management
* **PCID (Process Context Identifiers):**
  * Address space switches use PCID to prevent full TLB invalidation on context switches.
* **Selective TLB Flushing:**
  * Always prefer page-level invalidation (`invlpg` / `tlbi`) over full CR3 / ASID reloads for single-page unmaps.

---

## 🛡️ 3. AI Agent Rules & Code Patterns

1. **Prevent False Sharing:**
   ```rust
   // BAD: Head and Tail share a 64-byte cache line causing ping-ponging
   pub struct BadQueue {
       head: AtomicUsize,
       tail: AtomicUsize,
   }

   // GOOD: Padded to separate 64-byte cache lines
   #[repr(C, align(64))]
   pub struct CacheOptimizedQueue {
       head: AtomicUsize,
       _pad1: [u8; 56],
       tail: AtomicUsize,
       _pad2: [u8; 56],
   }
   ```
2. **Use Flat Arrays Over Pointer Chasing:**
   * Avoid linked structures (e.g. node-based trees) in performance-critical paths. Use flat contiguously allocated arrays or vectors (`Vec<T>`) to leverage spatial locality and hardware instruction prefetchers.
3. **Cache-Conscious Data Packing:**
   * Sort struct fields by decreasing alignment size (`u64` $\rightarrow$ `u32` $\rightarrow$ `u16` $\rightarrow$ `u8` $\rightarrow$ `bool`) to minimize struct padding bytes.

---

## 🧪 4. Verification & Testing Commands

AI agents must verify cache-aligned memory management and page cache operations via standalone tests:

```bash
# Test memory manager & performance allocator stack
rustc --test --edition=2021 src/kernel/perf_mm.rs -o build/perf_mm_tests && ./build/perf_mm_tests && rm build/perf_mm_tests

# Test filesystem page cache & buffer manager
rustc --test --edition=2021 src/filesystem/support.rs -o build/fs_cache_tests && ./build/fs_cache_tests && rm build/fs_cache_tests
```
