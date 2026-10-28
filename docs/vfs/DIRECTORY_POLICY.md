# Inode & Directory Management Policy

## Overview

The inode and directory manager (`src/fs/inode.rs` and `src/fs/directory.rs`) handles inode metadata, link counts, directory entry lookup/insertion/removal, and storage capacity limits.

## Key Rules & Invariants

1. **Inode Lifecycle**:
   - Inodes are created with an initial reference count (`nlink`) of 1.
   - `inc_ref()` increments `nlink`; `dec_ref()` decrements `nlink`.
   - When `nlink` reaches 0 and no active file handle references the inode, the inode storage blocks are reclaimed.
2. **Directory Entry Limits**:
   - Directory structures are bounded by `MAX_DIR_ENTRIES` (1024 entries per directory).
   - Filenames are bounded by `MAX_FILENAME_LEN` (255 characters).
   - Exceeding directory capacity returns `FsError::StorageFull`.
3. **Double-Unlink Prevention**:
   - Removing an entry from a directory checks if the entry exists. If not, returns `FsError::NotFound`.
   - Decrementing an inode's link count below 0 is prevented; link count operations are safe.
4. **Lost-Update Prevention**:
   - Directory mutation operations (add, remove, rename) update directory metadata (`mtime`, `ctime`) atomically.
   - Lookups search entries deterministically and check for deleted entries.

## Testing & Verification

Inode and directory operations are tested via `tests/fs/inode.rs` and `tests/filesystem/inode.rs`.
