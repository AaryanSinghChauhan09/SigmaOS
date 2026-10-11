# SigmaOS Block Cache Policy

## Overview

The Block Cache Subsystem (`src/fs/cache.rs` and `src/filesystem/cache.rs`) manages in-memory caching of storage blocks to accelerate read/write operations while guaranteeing data writeback durability and preventing OOM exhaustion.

---

## Core Requirements & Specifications

### 1. LRU Eviction & Writeback Semantics
- **Eviction Order**: Pages are evicted in Least Recently Used (LRU) order when cache capacity is exceeded.
- **Dirty Page Writeback**: Dirty pages (`CachePageState::Dirty`) are automatically written back to underlying block storage before eviction.
- **Writeback Errors**: Writeback failures preserve dirty status and return `CacheError::WritebackFailed`.

### 2. Double-Eviction Prevention
- Evicting an unmapped or already evicted page returns `CacheError::DoubleEviction` or `CacheError::NotCached`.
- Page state transitions (`Active` -> `Evicted`) are protected by mutex/atomic state flags.

### 3. OOM Metrics & Watermark Enforcement
- Cache size is bounded by `max_pages`.
- Watermark tracking exposes metrics via `CacheOomMetrics`:
  - `total_pages`: Total allocated cache pages.
  - `active_pages`: Currently active cached pages.
  - `dirty_pages`: Pages awaiting writeback.
  - `high_watermark_events`: Count of times cache usage exceeded 80% watermark.
  - `critical_watermark_events`: Count of times cache usage exceeded 90% watermark.

---

## Verification & Testing

Compliance is verified via `tests/filesystem/cache.rs` and `tests/filesystem_cache_tests.rs`:
- LRU eviction order verification.
- Automatic dirty page writeback on eviction.
- Double eviction error prevention.
- OOM watermark metrics recording.
