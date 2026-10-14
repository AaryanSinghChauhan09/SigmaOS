# SigmaOS Block Cache Policy Specification

## 1. Overview

This document specifies the caching, eviction, and memory safety rules for `src/fs/cache.rs` (`BlockCacheManager`).

---

## 2. Cache Architecture & Eviction Semantics

### 2.1 Struct & State Management
- Cached pages are tracked using `CachePage` structures.
- Page states: `Active`, `Flushing`, `Dirty`, `Clean`.
- Eviction order: LRU (Least Recently Used) with priority given to clean pages over dirty pages.

### 2.2 Lost-Writeback Prevention
- Before a dirty page is evicted from the cache, `BlockCacheManager` MUST issue a dirty writeback to the underlying block device.
- If writeback fails, the page is preserved in cache, flagged with an I/O error status, and eviction is aborted to prevent data loss.

### 2.3 Double-Eviction Prevention
- Eviction operations lock the candidate page entry during eviction processing.
- Attempts to evict an already evicted page or a page currently undergoing flush return `CacheError::DoubleEviction`.

---

## 3. OOM & Observability

### 3.1 Watermarks & Metrics
`BlockCacheManager` exposes observable OOM metrics (`CacheOomMetrics`):
- `total_pages`: Total pages allocated in the cache pool.
- `max_pages`: Hard limit on page cache capacity.
- `dirty_pages`: Count of modified pages awaiting writeback.
- `reclaimed_pages`: Total pages successfully evicted under memory pressure.

### 3.2 OOM Reclamation Handlers
When memory watermarks cross `watermark_high`:
1. Background kswapd/reclaim triggers clean page eviction.
2. Synchronous writebacks are scheduled for dirty pages until cache usage drops below `watermark_low`.
