# Architecture Decision Record: VFS, File Descriptors, Inodes, Permissions, and Locking

- **Status**: Accepted
- **Date**: 2026-03-30
- **Context**: SigmaOS requires an authoritative, safe, observable, and recoverable virtual file system (VFS) architecture that governs file descriptors, inodes, directories, permissions, locking, and file lifecycles.

## Decision Summary

1. **VFS Structure**: Adopt a request-based, inode-indexed hybrid VFS layer with clear boundary abstractions between process-local file descriptor tables (`ProcessFdTable`), system file handles (`FileHandle`), inodes (`Inode`), and filesystem backends (`SigmaFS`, `Btrfs`, `XFS`, `autofs`, `procfs`, `tmpfs`).
2. **File Descriptor Management**: Implement process-local descriptor tables with fixed bounds (`DEFAULT_MAX_FDS = 1024`), low-fd reuse policy, protected standard streams (0: stdin, 1: stdout, 2: stderr), atomic `FD_CLOEXEC` handling on exec, and typed exhaustion errors (`FsError::NoFileDescriptorsAvailable`).
3. **Inode & Directory Lifecycle**: Maintain unique 64-bit inode indexing with atomic reference counting (`inc_ref`/`dec_ref`), explicit unlinking checks preventing double-unlink, bounded directory entry capacities (`MAX_DIR_ENTRIES = 1024`), and lost-update prevention during metadata modifications.
4. **Permissions & Capabilities**: Enforce POSIX permission bit checking (read, write, execute across User, Group, Other) supplemented by Linux-style capabilities (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, `CAP_FOWNER`) for process credential evaluation.
5. **Locking Infrastructure**: Implement advisory file range locking (`FileRangeLock`) and BSD `flock` semantics (`LOCK_SH`, `LOCK_EX`, `LOCK_NB`, `LOCK_UN`) with lock overlap detection and automatic cleanup on process descriptor close or termination.

## Consequences & Safety Guarantees

- **Memory Safety & Resource Bounds**: Zero allocation leaks, deterministic cleanup on process exit or handle release.
- **Race Prevention**: Synchronization primitives and atomic flags ensure concurrent syscalls do not cause descriptor reuse races or partial metadata updates.
- **Observability**: Exhaustion and permission denial conditions yield typed `FsError` codes.
