# Directory & Inode Policy

## Overview
SigmaOS manages directory entries (`src/fs/directory.rs`) and inode metadata (`src/fs/inode.rs`) with explicit lifecycle bounds and concurrency safety.

## Directives
1. **Explicit Inode Lifecycle**: Inodes track link counts, size, permissions, modification times, and active open reference counts.
2. **Bounded Capacity**: Directory child entries are bounded to prevent memory allocation exhaustion under untrusted user input.
3. **Race-Safe Operations**: Directory lookups, insertions, and deletions execute under fine-grained locks or lock-free reference structures.
4. **Lost-Update Prevention**: Concurrent updates to directory entries or inode attributes are linearized to prevent stale metadata overwrites.
5. **Double-Unlink Prevention**: Unlinking an entry checks link counts atomically; unlinking an unlinked or non-existent inode returns an error and avoids negative link counts.
6. **OOM Observability**: Failures to allocate inode or directory entry structures emit observable diagnostics and typed memory errors (`FsError::NoMemory`).
