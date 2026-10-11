# File Descriptor Policy (FD_POLICY)

## File Descriptor Management
SigmaOS manages process file descriptors via `ProcessFdTable` defined in `src/fs/fd_table.rs` and `src/fs/fd.rs`.

## Allocation and Selection
- **Lowest Available Slot Policy**: New descriptors are allocated in the lowest available index in `ProcessFdTable`.
- **Reserved Descriptors**: Slots `0` (`STDIN_FILENO`), `1` (`STDOUT_FILENO`), and `2` (`STDERR_FILENO`) are reserved for standard streams during process initialization.
- **Race Prevention**: Descriptors are protected by a `Mutex` guard within `ProcessFdTable` ensuring race-free concurrent calls to `allocate_fd`, `dup_fd`, and `dup2_fd`.

## Capacity Limits and Exhaustion
- Process descriptor tables have a default maximum capacity (`DEFAULT_MAX_FDS = 1024`).
- When all table slots are occupied, allocation calls return `Err(FsError::NoFileDescriptorsAvailable)`.

## Cloexec and Execve Semantics
- `FD_CLOEXEC` flag (value `0x01`) marks descriptors for automatic closure during process image execution.
- Calling `table.handle_exec()` scans active descriptors and closes any descriptor with `is_cloexec() == true`, freeing the slot and releasing handle references.

## Handle Ownership & Lifetime
- `FileHandle` objects are reference-counted (`Arc<FileHandle>`). Duplicating a file descriptor (`dup`, `dup2`) increments the reference count without duplicating the underlying file position or lock state.
- Closing a descriptor (`close_fd`) removes the entry from `ProcessFdTable` and drops the `FileHandle` reference exactly once.
