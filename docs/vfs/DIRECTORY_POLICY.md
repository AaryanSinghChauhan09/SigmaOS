# Directory Management Policy (DIRECTORY_POLICY)

## Inode and Directory Lifecycle
SigmaOS directory operations are governed by `InodeManager` (`src/fs/inode.rs`) and `Directory` (`src/fs/directory.rs`).

## Directory Semantics
- **Entry Structure**: `DirectoryEntry` binds a string filename (`MAX_FILENAME_LEN = 255`) to an inode identifier (`inode_id`) and `InodeType` (File, Directory, Symlink, Pipe, Socket, Device).
- **Capacity Limits**: Directory capacity is bounded by `MAX_DIR_ENTRIES = 4096`. Adding entries past capacity returns `Err(FsError::StorageFull)`.
- **Lookup & Mutability**: Filename resolution is case-sensitive and thread-safe.

## Lost-Update and Unlink Prevention
- **Unlink Lifecycle**: Unlinking a filename removes the entry from the parent directory and decrements `nlink` on the corresponding `Inode`.
- **Double-Unlink Prevention**: If `nlink` reaches `0`, the inode state transitions to `InodeState::Unlinked`. Subsequent unlink calls on the same inode return `Err(FsError::NotFound)`.
- **Orphan Cleanup**: Unlinked inodes with active `ref_count > 0` remain accessible to existing open file handles until `ref_count` drops to zero, at which point storage blocks are reclaimed.

## Concurrency and Atomic Operations
- All directory updates (`add_entry`, `remove_entry`) are protected by mutex locks.
- Directory removal requires the directory to be empty (`DirectoryNotEmpty` error returned if child entries exist).
