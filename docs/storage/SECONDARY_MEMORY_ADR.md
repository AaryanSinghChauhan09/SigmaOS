# SigmaOS Secondary Memory Architecture Decision Record (ADR)

## Context and Problem Statement

SigmaOS requires a unified, maintainable, and high-performance secondary memory management subsystem. Existing storage components must follow clear design guidelines to ensure crash consistency, race-free concurrency, memory safety, and long-term ABI stability.

## Architecture Decisions

### 1. Bounded Request Allocation
- **Decision**: All block device queues, request allocators, and cache structures must enforce upper memory bounds.
- **Rationale**: Unbounded allocation in storage path drivers leads to unpredictable kernel memory exhaustion under heavy I/O workloads.
- **Enforcement**: Request exhaustion returns `BlockError::RequestExhaustion`.

### 2. Exact-Once Request Completion
- **Decision**: Request completion follows strict lifecycle transitions (`Pending -> Dispatched -> Completed/Failed`).
- **Rationale**: Double-completion causes buffer corruption, while lost completion leads to task deadlocks.
- **Enforcement**: Double completion yields `BlockError::DoubleCompletion`, and lost completion yields `BlockError::LostCompletion`.

### 3. Isolated Block Cache LRU Eviction
- **Decision**: Block page cache manages clean and dirty blocks separately.
- **Rationale**: Evicting dirty blocks without writeback causes data loss.
- **Enforcement**: `BlockCacheManager` dirty writebacks precede eviction; double eviction yields `CacheError::DoubleEviction`.

### 4. Storage ABI Compatibility
- **Decision**: Kernel-internal storage APIs can be refactored with code review. Public system call ABIs (`sys_mount`, `sys_read`, `sys_write`) require strict backwards compatibility or ADR migration notes.

## Status & Conformance
All storage decisions adhere to `STORAGE_RULES.toml` and are verified through automated unit tests in `tests/storage/` and `tests/filesystem/`.
