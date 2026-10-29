# ADR 0002: SigmaOS Virtual Filesystem Architecture

## Context

SigmaOS requires a safe, high-performance, and observable virtual filesystem (VFS) architecture that governs file descriptors, inodes, directories, permissions, file locks, and path resolution across kernel and userland environments. The system needs clear POSIX compatibility guarantees while enforcing strict safety invariants such as bounded resource allocation, race-free concurrency, exact-once completion, and zero memory leaks under stress.

## Decision

We adopt a **Hybrid Request-Based and Inode-Backed VFS** architecture for SigmaOS:

1. **Process File Descriptor Tables (`ProcessFdTable`)**: Bounded array of file descriptor slots (`DEFAULT_MAX_FDS = 1024`) per process. Standard streams (`STDIN = 0`, `STDOUT = 1`, `STDERR = 2`) are strictly reserved and protected. Allocations choose the lowest available index race-free.
2. **File Handle Abstraction (`FileHandle`)**: Process-independent reference-counted file handle encapsulating open mode flags (`O_RDONLY`, `O_WRONLY`, `O_RDWR`, `O_CREAT`, `O_CLOEXEC`), current offset, and underlying inode pointer (`Inode`).
3. **Inode Management (`InodeManager` & `Inode`)**: Inodes maintain reference counts (`inc_ref`, `dec_ref`), hard link counts (`nlink`), file size, mode, uid/gid, and type (`File`, `Directory`, `Symlink`, `Pipe`, `Device`). Double-unlink and lost-update conditions are prevented via lock ordering and atomic refcount checks.
4. **Directory Invariants (`Directory`)**: Directory entries map filename strings to inode IDs with capacity bounds (`MAX_DIR_ENTRIES = 1024`, `MAX_FILENAME_LEN = 255`).
5. **Permissions & Capabilities (`PermissionEngine`)**: Standard POSIX permission checks (owner, group, other) supplemented by capability bitmasks (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, `CAP_FOWNER`).
6. **File Lock Architecture (`FileLockManager`)**: Supports process `flock` semantics (`LOCK_SH`, `LOCK_EX`, `LOCK_UN`, `LOCK_NB`) and byte-range locks (`FileRangeLock`).

## Status

Accepted and Implemented.

## Consequences

- Direct POSIX file I/O operations (`open`, `read`, `write`, `lseek`, `close`, `dup`, `dup2`, `flock`, `chmod`, `unlink`) are fully supported.
- Resource limits are explicitly bounded and return typed `FsError` failures on exhaustion instead of panicking or causing kernel memory corruption.
- Full traceability across open descriptors, referenced inodes, and active lock regions simplifies system debugging and observability.
