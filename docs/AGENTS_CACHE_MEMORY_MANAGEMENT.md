# SigmaOS Cache Memory Architecture, LRU Eviction & Object Cache Guide for AI Agents

This guide provides technical specifications, cache key invalidation semantics, LRU eviction policies, package cache trimming routines, and cache line memory alignment rules for AI agents managing cache memory in SigmaOS.

---

## 1. Zero-Dependency `#![no_std]` Cache Architecture

SigmaOS implements high-performance in-memory caching and package cache management without external crate dependencies (`src/open_source_obsoletion.rs`, `src/sigpkg/pacman_contrib.rs`, `src/package/cache.rs`):

* **Sovereign Cache Engine (`SovereignCacheEngine` in `src/open_source_obsoletion.rs`):**
  Provides thread-safe in-memory key-value caching, TTL-based expiration, key purging, and LRU cache eviction (obsoletes Redis / Memcached).
* **Package Cache Trimmer (`PacCacheTrimmer` in `src/sigpkg/pacman_contrib.rs`):**
  Provides automated package archive cache pruning, retaining $N$ recent package versions or removing uninstalled package archives (native Rust parity for `paccache`).

---

## 2. Key Invalidation & Cache Retention Semantics

When working with cache entries in `SovereignCacheEngine`:

1. **Key Uniqueness & Retention Invariant:**
   `SovereignCacheEngine::set` MUST purge pre-existing entries with the same key using `self.entries.retain(|e| e.key != key)` before inserting updated values. This prevents key duplication and memory leaks.
2. **TTL Expiration Evaluation:**
   Cached items with a non-zero `ttl_seconds` MUST be evaluated against current system uptime during `get` operations. Expired items MUST be purged automatically upon access.

---

## 3. Package Cache Management (`PacCacheTrimmer`)

* **Retention Policy:**
  When trimming package cache directories, `PacCacheTrimmer` sorts cached archives by version strings and removes all candidates exceeding the specified retention count.
* **Uninstalled Cleanups:**
  When invoked with uninstalled filter flags, `PacCacheTrimmer` cross-references installed package databases and purges orphaned package archives.

---

## 4. Checklist for AI Agents Managing Cache Memory

1. **Verify Key Retention Invariant:** Ensure `set` operations in custom caches purge existing keys via `.retain(|e| e.key != key)`.
2. **Test Cache Subsystem Pipelines:**
   Run cache engine unit tests:
   ```bash
   cargo test --lib -- sigpkg::pacman_contrib::tests
   ./run_sigma_tests.sh
   ```
