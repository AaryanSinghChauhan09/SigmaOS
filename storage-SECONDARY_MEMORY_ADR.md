# Secondary Memory Architecture Decision Record (ADR)

## Status
Accepted

## Context
SigmaOS requires a production-grade, safe, observable, and recoverable secondary-memory storage subsystem. Historically, standalone storage models existed in isolation without runtime lifecycle contracts or explicit error semantics.

## Decision
Establish an authoritative secondary memory lifecycle model (`probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery`) governed by:

1. Bounded block request queues (`BoundedBlockDeviceManager`) preventing lost or double completions.
2. LRU block cache management (`BlockCacheManager`) with dirty page writeback tracking and OOM watermark metrics.
3. Capacity-bounded volume manager (`FilesystemCoreManager`) with lost-update prevention.
4. Reference-counted mount manager (`MountManager`) supporting atomic mount rollbacks and double-unmount protection.
5. Typed error propagation across all storage layers.

## Consequences
- Unbounded queue growth or untracked double completions are strictly prevented.
- All storage status claims require verification tests and boot-to-shell evidence.
- Full compatibility with no_std kernel execution and safe Rust memory bounds.
