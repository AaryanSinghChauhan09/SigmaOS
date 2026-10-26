# SigmaOS Block Cache Policy

## 1. Overview

The SigmaOS block cache manager (`src/fs/cache.rs`) manages in-memory caching for block device data, maintaining data consistency, dirty writeback, and memory bounds.

## 2. Invariants and Safety Guarantees

### 2.1 Cache Eviction and Reuse
- Cache eviction is race-free and follows an LRU (Least Recently Used) policy based on access timestamps (`last_access_ms`) and access counts (`access_count`).
- Cached pages transition through states: `Active` -> `Flushing` -> `Evicting` -> `Evicted`.

### 2.2 Lost-Writeback Prevention
- Dirty pages (`is_dirty == true`) must be flushed to the underlying block device before eviction completes.
- Evicting a dirty page without writeback returns `CacheError::WritebackFailed`.

### 2.3 Double-Eviction Prevention
- Evicting a page already in `Evicted` or `Evicting` state returns `CacheError::DoubleEviction`.

### 2.4 Bounded Metadata and OOM Enforcement
- Cache size is strictly bounded by `max_pages`.
- Watermark levels (`Normal`, `HighPressure`, `OomCritical`) dictate eviction aggressiveness.
- OOM metrics (`CacheOomMetrics`) continuously measure total pages, dirty pages, hits, misses, evictions, and reclaimed bytes.
