# Block Cache Manager Policy Specification

## 1. Scope & Purpose

This document specifies the operational policies, eviction semantics, and memory safety invariants enforced by the SigmaOS Block Cache Manager (`src/fs/cache.rs`).

---

## 2. Cache Semantics & Invariants

### 2.1 LRU Eviction & Race Freedom
- Block cache pages (`CachePage`) are managed by `BlockCacheManager`.
- Cache insertion (`insert_page`) checks page capacity. When capacity is reached, the least recently used (LRU) clean or dirty page is selected based on `last_access_ms`.
- Eviction is race-free and deterministic.

### 2.2 Lost-Writeback Prevention
- If an LRU page selected for eviction is marked `is_dirty = true`, its contents are automatically written back to the underlying block device before eviction.
- If writeback fails, eviction is aborted and `Err(CacheError::WritebackFailed)` is returned.

### 2.3 Double-Eviction Prevention
- Explicit page eviction (`evict_page`) transitions page state to `Evicting` then `Evicted`.
- Attempting to evict an already evicted or non-cached page returns `Err(CacheError::NotCached)` or `Err(CacheError::DoubleEviction)`.

---

## 3. OOM Watermark Metrics & Observability

`BlockCacheManager` measures cache memory usage and OOM pressure using `CacheOomMetrics`:

| Watermark Level | Threshold | Action Taken |
|---|---|---|
| **Normal** | Usage < 80% | Standard LRU caching and access tracking |
| **HighPressure** | Usage >= 80% | Proactive clean LRU page eviction triggered |
| **OomCritical** | Usage >= 95% | Immediate flush of dirty pages and aggressive clean page reclamation |

### Observable Metrics
- `total_pages`: Current active cache pages
- `max_pages`: Total allocated page capacity
- `dirty_pages`: Current dirty cache pages awaiting writeback
- `cache_hits` & `cache_misses`: Cache hit/miss ratio
- `eviction_count`: Total evicted pages
- `reclaimed_bytes`: Total memory reclaimed under OOM pressure

---

## 4. Verification

Enforced via unit test suite `tests/filesystem/cache.rs`:
- `test_cache_lru_eviction_and_dirty_writeback`
- `test_cache_double_eviction_prevention`
- `test_cache_oom_watermark_metrics`
