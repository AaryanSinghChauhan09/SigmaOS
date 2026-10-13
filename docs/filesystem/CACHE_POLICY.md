# SigmaOS Block Cache Policy Specification

## 1. Overview

The Block Cache Manager (`BlockCacheManager` in `src/fs/cache.rs`) manages memory-resident page cache blocks to accelerate disk read/write throughput while ensuring write-back coherence and memory safety under OOM pressure.

## 2. Invariants & Operational Policy

### 2.1 Bound Limits & LRU Eviction
- Cache size is bounded by `max_pages`.
- Insertion into a full cache triggers LRU eviction of the least recently accessed clean or flushed page.

### 2.2 Dirty Page Writeback & Lost-Writeback Prevention
- Mutated pages are tagged `is_dirty = true`.
- Eviction of a dirty page forces immediate synchronous writeback to the underlying block device before eviction completes, preventing data loss.

### 2.3 Double-Eviction Prevention
- Evicting a non-existent or already evicted page returns `Err(CacheError::NotCached)` or `Err(CacheError::DoubleEviction)`.
- Eviction operations clear the page's cache tag atomically or under lock.

### 2.4 Observable OOM Metrics & Watermarks
- The cache reports observable usage via `get_oom_metrics() -> CacheOomMetrics`.
- High watermark (`high_watermark_pct`, e.g., 80%) triggers asynchronous background writeback.
- Low watermark (`low_watermark_pct`, e.g., 50%) stops writeback tasks once cache pressure is relieved.
