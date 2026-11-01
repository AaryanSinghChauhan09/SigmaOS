# SigmaOS Cache Subsystem Policy

## Overview
The cache subsystem bridges block devices and higher-level filesystems, caching disk sectors in memory to
accelerate read and write I/O. It enforces LRU eviction, dirty page writeback, bounds on metadata, and
OOM watermark metrics.

## Core Requirements

### 1. Bounded Cache Metadata and Pages
- The cache manager (`BlockCacheManager`) enforces an upper limit on total cached pages (`max_pages`).
- Memory allocation for page buffers and index trees is tracked and strictly bounded.

### 2. Race-Free LRU Eviction
- Cache pages record their last access timestamp (`last_accessed`).
- When page allocation reaches `max_pages`, an eviction pass identifies the least recently accessed page.
- Clean pages are discarded immediately; dirty pages are scheduled for writeback before eviction.

### 3. Lost-Writeback Prevention
- Any modified page is flagged as `is_dirty = true`.
- An eviction or flush operation ensures dirty page data is successfully transferred to the block device.
- Evicting a dirty page without writeback is illegal.

### 4. Double-Eviction Prevention
- Pages already evicted from cache cannot be re-evicted.
- Calling `evict_page()` on a missing or unallocated page returns `CacheError::NotCached`.

### 5. OOM Enforcement and Observability
- Cache pressure is continuously tracked through `WatermarkLevel`:
  - `Normal`: Cache utilization < 70%.
  - `PressureWarn`: Cache utilization >= 70% and < 90%.
  - `OomCritical`: Cache utilization >= 90%.
- System memory managers can inspect cache metrics (`total_pages`, `dirty_pages`, `watermark_level`)
  to trigger synchronous reclamation before OOM killer activation.
