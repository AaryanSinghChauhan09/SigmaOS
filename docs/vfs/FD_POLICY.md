# File Descriptor Policy (FD_POLICY)

## Allocation & Bounds
- Descriptor tables are process-local (`ProcessFdTable`).
- Allocation finds the lowest available slot starting from index 0 or specified `min_fd`.
- Default table limit is `DEFAULT_MAX_FDS = 1024`. Exhaustion returns `FsError::NoFileDescriptorsAvailable`.
- Standard descriptors (0: STDIN, 1: STDOUT, 2: STDERR) are pre-reserved or protected from accidental implicit allocation.

## Race Safety & Memory Safety
- Descriptors wrap `Arc<FileHandle>` or reference-counted file objects.
- Closing a descriptor releases the slot atomically and decrements handle refcount.
- Double-close attempts return `FsError::InvalidFileDescriptor`.

## Close-on-Exec (FD_CLOEXEC)
- `FD_CLOEXEC` flag (bit 0x01) marks descriptors to be automatically closed during `execve` transitions via `close_cloexec_fds()`.
