# ⚡ AI Agents Buffer Cache Management Specification (`docs/AI_AGENTS_BUFFER_CACHE_MANAGEMENT.md`)

This specification defines unified page/buffer cache architectures, cache eviction algorithms, asynchronous writeback daemons, and memory pressure policies for autonomous AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) in SigmaOS.

---

## 1. Unified Page & Buffer Cache Architecture (`src/kernel/mm/page_cache.rs`)

AI agents manage unified page and block buffer caches:
- **Radix Tree Indexing**: $O(\log N)$ file offset to physical page frame lookup.
- **Dirty Page Tracking**: Bitmap and lockless state flags marking cached pages modified in memory requiring storage flush.
- **Read-Ahead Prefetching**: Sequential I/O pattern detection prefetching contiguous block clusters into cache ahead of application requests.

---

## 2. Cache Eviction & Replacement Policies

- **Adaptive Replacement Cache (ARC)**: Dynamic balancing between recency (LRU) and frequency (LFU) workloads.
- **2Q Eviction**: Multi-queue buffer management separating single-access transient blocks from frequent hot cache blocks.
- **Writeback Flush Daemons**: Background flushing threads persisting dirty buffers when dirty memory exceeds `20%` of physical RAM or after 5-second max age timeout.

---

## 3. Synchronous / Asynchronous Flush & OOM Shrinking

- **Flush Operations (`sync`, `fsync`, `fdatasync`)**: Synchronous barrier enforcement ensuring dirty buffers are committed to persistent storage.
- **Memory Pressure Shrinking**: Dynamic buffer cache page reclamation triggered by the kernel OOM killer under high memory pressure.

---

## 4. AI Agent Buffer Cache Responsibilities

- **⚡ Bolt**: Profiles cache hit ratios, measures writeback flush latency, and tunes read-ahead prefetch window sizes.
- **🎨 Palette**: Renders live page cache utilization charts, dirty buffer ratios, and memory pressure state in control center diagnostic views.
- **🛡️ Sentinel**: Audits dirty page zeroization upon unmap, verifies encrypted storage block cache buffers, and prevents memory leak side channels.
