# AI Agent Disk Cache & Page Cache Management in SigmaOS

## Overview

SigmaOS page cache architecture (`src/filesystem/`, `src/storage/`, `src/kernel/memory/sigma_buddy.rs`, `src/filesystem/cow_snapshot.rs`) implements VFS page caching, Adaptive Replacement Cache (ARC / 2Q) block eviction, write-back & write-through dirty page flushing, and memory-mapped block caching.

AI agents (such as Jules, Herdr agentic subagents, database query runners, and file refactoring tasks) must follow these disk cache guidelines to optimize I/O latency while preventing memory starvation.

---

## Page Cache Architecture & Eviction Policies

```
Agent File Request → VFS Page Cache Lookup (Page Frame Number)
                           │
             ┌─────────────┴─────────────┐
             ▼                           ▼
        Cache Hit                    Cache Miss
  (Zero-Copy Memory Read)     (Fetch Block & Allocate Frame)
                                         │
                                         ▼
                            ARC / 2Q Eviction Policy
                           (Most Frequently / Recently Used)
```

---

## Cache Eviction Algorithms

SigmaOS supports 2 primary cache eviction algorithms for AI agent file workloads:

| Eviction Algorithm | Subsystem Parity | Best For |
|--------------------|------------------|----------|
| **Adaptive Replacement Cache (ARC)** | ZFS / FreeBSD | Dynamic workloads (balances frequency vs recency) |
| **Two-Queue (2Q) / LRU** | Linux Kernel Page Cache | High-volume sequential file reads (prevents cache pollution) |

---

## Dirty Page Flushing & Write-Back Policies

AI agents writing data to disk can configure write-back vs write-through cache flushing:

```rust
use sigmaos::filesystem::PageCacheManager;

let mut page_cache = PageCacheManager::new();

// Write dirty page under write-back policy
page_cache.write_page_dirty(block_pfn, &dirty_data)?;

// Flush dirty pages asynchronously to storage device (sys_sync / fsync)
page_cache.flush_dirty_pages_async()?;

// Force synchronous write-through flush for critical transaction logs
page_cache.fsync_file(file_handle)?;
```

---

## Page Cache Memory Limits for AI Agents

To prevent disk page caching from consuming all physical RAM and causing agent process OOM kills, page cache memory is bounded under cgroup memory pressure:

```
/sys/fs/cgroup/system.slice/sigma-agent.service/
├── memory.high = 1536M      # Triggers background page cache eviction
└── memory.max = 2048M       # Hard memory ceiling; drops clean page cache
```

---

## Directives for AI Agents

1. **`fsync()` Critical Metadata**: Always call `fsync()` after writing essential state files or database transaction logs.
2. **Advise Access Patterns (`posix_fadvise`)**: Use `POSIX_FADV_NOREUSE` or `POSIX_FADV_SEQUENTIAL` for large file scans to avoid polluting the page cache.
3. **Drop Clean Cache Under Memory Pressure**: Monitor `BuddyMemoryWatermarkEngine` and trigger clean page cache release if free memory drops below `WatermarkLow`.
