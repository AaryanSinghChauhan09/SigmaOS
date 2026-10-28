# Directory Policy (DIRECTORY_POLICY)

## Directory Structure & Bounds
- Directories contain ordered entries of `DirectoryEntry` (`name`, `inode_id`, `inode_type`).
- Directory capacity is bounded by `MAX_DIR_ENTRIES = 4096`.
- Maximum filename length is enforced at `MAX_FILENAME_LEN = 255`.

## Directory Lifecycle & Operations
- Entry insertion checks for duplicate names (`FsError::FileAlreadyExists`).
- Entry removal checks for existence (`FsError::NotFound`).
- Directory deletion requires the directory to be empty (`FsError::DirectoryNotEmpty`).
- Double-unlink protection ensures an unlinked entry cannot be removed twice or leave stale child pointers.
