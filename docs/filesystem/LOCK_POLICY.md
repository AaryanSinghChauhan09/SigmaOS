# Lock Policy (LOCK_POLICY.md)

## Overview
The File Lock Manager in SigmaOS manages advisory file locking for byte ranges (`FileRangeLock`) and full files (`LOCK_SH`, `LOCK_EX`).

## Requirements & Enforcements
- **Advisory & Range Lock Types**: Supports shared (`LockType::Shared` / `LOCK_SH` / `F_RDLCK`) and exclusive (`LockType::Exclusive` / `LOCK_EX` / `F_WRLCK`) locking across specified byte ranges (`start`, `len`).
- **Race-Free Enqueue & Dequeue**: Lock acquisitions check for overlapping ranges from other process owners (`owner_pid`). Conflicting non-blocking requests (`LOCK_NB`) return `FsError::WouldBlock` or `FsError::ResourceBusy`.
- **Bounded Metadata**: Active lock entries are stored in bounded per-inode structures.
- **Lost-Lock & Double-Unlock Prevention**: Unlocking (`LOCK_UN` / `F_UNLCK`) verifies lock ownership (`owner_pid`, `fd_id`) before releasing matching locks. Explicit close of a file descriptor automatically purges all locks owned by that descriptor.
- **OOM Observability**: Memory usage for lock tables is capped, and allocation failures surface safely without corrupting existing locks.
