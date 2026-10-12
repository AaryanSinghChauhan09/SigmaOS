# SigmaOS Directory Manager Policy

## Overview

The directory manager (`Directory`, `DirectoryEntry`) governs filename-to-inode resolution, entry insertion, lookup, and deletion within the VFS hierarchy.

## Key Policy Directives

1. **Explicit Entry Bounds**: Directories enforce a maximum capacity of `MAX_DIR_ENTRIES` (1024) and maximum filename length of `MAX_FILENAME_LEN` (255 bytes). Operations exceeding limits return `FsError::StorageFull` or `FsError::InvalidOperation`.
2. **Race-Safe Operations**: Directory entry lookup, insertion (`add_entry`), and deletion (`remove_entry`) execute under fine-grained lock protection.
3. **Double-Unlink Prevention**: Attempting to remove an entry that does not exist returns `FsError::NotFound`. Unlinking an entry decrements the target inode link count (`nlink`); when `nlink == 0` and active reference count is zero, the inode transitions to `Unlinked` and `Free`.
4. **Lost-Update Prevention**: Directory entry additions and unlinks perform atomic state updates. Memory allocations during directory growth check available system memory to enforce observable OOM metrics.
