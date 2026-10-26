# Block Cache Policy Specification

## Architectural Overview

`BlockCacheManager` manages secondary memory block caching, LRU eviction, dirty page flushing, and OOM pressure mitigation.

## Core Requirements & Rules

### 1. Race-Free Eviction
- Page cache eviction executes atomically under lock discipline.
- Clean pages are evicted immediately; dirty pages trigger synchronous writeback to storage before slot release.

### 2. Double-Eviction Prevention
- Evicting an unmapped page returns `CacheError::NotCached`.
- Attempting to evict a page already undergoing eviction returns `CacheError::DoubleEviction`.

### 3. Observable OOM Enforcement
- `BlockCacheManager` tracks memory usage and watermark thresholds.
- Metrics (`total_pages`, `dirty_pages`, `evictions`, `watermark_hits`) are exposed for system diagnostics and observability.
