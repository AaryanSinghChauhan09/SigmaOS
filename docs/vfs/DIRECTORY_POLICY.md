# Directory Policy (DIRECTORY_POLICY.md)

## Overview
The Directory Manager in SigmaOS governs directory inode lifecycle, directory entry (`DirectoryEntry`) indexing, lookup, insertion, and unlinking.

## Requirements & Enforcements
- **Explicit Lifecycle**: Directory inodes track state transitions from `Allocated` -> `Active` -> `Unlinked` -> `Reclaimed`.
- **Bounded Directory Capacity**: Directories enforce `MAX_DIR_ENTRIES = 4096` and `MAX_FILENAME_LEN = 255`. Exceeding entry limits returns `FsError::DirectoryNotEmpty` or `FsError::InvalidArgument`.
- **Race Safety**: Access and modifications to directory entry lists are synchronized via reader-writer locks or atomic spinlocks.
- **Lost-Update Prevention**: Directory entry additions and removals mutate entry structures under lock protection and update directory modification timestamps coherently.
- **Double-Unlink Prevention**: Unlinking an entry verifies that the target exists and transitions the child inode state to `Unlinked`. Subsequent unlink attempts on non-existent or unlinked paths return `FsError::NotFound`.
- **OOM Observability**: Memory allocation for new directory entries is bounded and monitored, returning typed errors on allocation failure.
