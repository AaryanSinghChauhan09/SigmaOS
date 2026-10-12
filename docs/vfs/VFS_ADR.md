# Architectural Decision Record: Hybrid Request-Based and Inode-Based Virtual Filesystem (VFS)

## Context

SigmaOS requires a high-performance, safe, observable, and POSIX-compliant file management subsystem. The filesystem architecture must support process-local isolation, race-free concurrency, lock safety, and deterministic resource release.

## Decision

SigmaOS adopts a hybrid VFS design:
1. **Request-Based Block Layer**: Integrates high-throughput buffer caching (`BufferCache`) and LRU page caching (`BlockCacheManager`) for raw block and page IO operations.
2. **Inode-Based VFS Layer**: Manages tree hierarchies, path resolutions, directory structures (`Directory`), inodes (`Inode`), file descriptors (`ProcessFdTable`), permission checking (`PermissionEngine`), and locking (`FileLockManager`).

## Consequences

- **Safety & Isolation**: Process-local descriptor tables isolate process file handles. Reserved file descriptors (`STDIN=0`, `STDOUT=1`, `STDERR=2`) are strictly protected from accidental double-closure or unallocated reuse.
- **Race-Free Concurrency**: Atomic operations and granular mutex/spinlocks prevent descriptor and inode allocation races.
- **Resource Bounds**: Hard limits on file descriptors per process and entries per directory prevent unbounded memory consumption under load.
- **POSIX Compatibility**: Implements standard POSIX seek, `flock`, byte-range locking, umask, and permission evaluation rules.
