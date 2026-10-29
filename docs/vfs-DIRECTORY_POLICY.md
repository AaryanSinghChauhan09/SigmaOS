# SigmaOS Directory and Inode Subsystem Policy

## Objective

Govern inode creation, reference counting, hard links, unlinking, directory entries, directory lookup, and directory capacity bounds in `src/fs/inode.rs` and `src/fs/directory.rs`.

## Requirements & Rules

1. **Inode Lifecycle & Reference Counting**:
   - `Inode` objects are managed by `InodeManager`.
   - `inc_ref()` increments the in-memory reference count; `dec_ref()` decrements it.
   - An inode's data and disk/memory backing store is freed if and only if `ref_count == 0` AND `nlink == 0`.

2. **Directory Capacity Bounds**:
   - Filenames are bounded to `MAX_FILENAME_LEN = 255` bytes.
   - Directories are bounded to `MAX_DIR_ENTRIES = 1024` entries.
   - Exceeding directory entry capacity returns `Err(FsError::StorageFull)` or `Err(FsError::OutOfMemory)`.

3. **Directory Operations**:
   - `add_entry(name, inode_id, entry_type)` inserts an entry if capacity permits and `name` does not already exist. Duplicate names return `Err(FsError::FileAlreadyExists)`.
   - `remove_entry(name)` finds and removes the entry, returning its inode ID. If the entry does not exist, it returns `Err(FsError::NotFound)`.
   - `lookup(name)` provides path resolution.

4. **Double-Unlink & Lost-Update Prevention**:
   - Unlinking a file decrements `nlink`.
   - Unlinking an already unlinked file (`nlink == 0`) is prevented and returns `Err(FsError::NotFound)`.
   - Concurrent modification of directory entries is synchronized to prevent lost updates or directory table corruptions.

5. **Observable OOM & Error Reporting**:
   - Exhaustion of inode table slots or directory entries is observable via `InodeManager` metrics and returns typed `FsError` failures.
