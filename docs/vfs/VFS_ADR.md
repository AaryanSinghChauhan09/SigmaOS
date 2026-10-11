# VFS Architecture Decision Record (ADR)

## Title: VFS Lifecycle Subsystem and Abstraction Layer

### Context
SigmaOS requires a safe, observable, recoverable, and POSIX-compatible Virtual Filesystem (VFS) architecture. The file management subsystem governs file descriptors, inode lifecycles, directory operations, permission evaluation, and file locking across process boundaries.

### Decision
1. **Hybrid Descriptor and Inode Architecture**:
   - `ProcessFdTable` maintains a bounded array of `FileDescriptor` slots per process.
   - Standard input, output, and error handles (`STDIN_FILENO`, `STDOUT_FILENO`, `STDERR_FILENO`) occupy reserved slots 0, 1, and 2.
   - `FileHandle` encapsulates open file description state (seek offset, flags, inode ID).
   - `InodeManager` manages global in-memory inodes with explicit reference counting and atomic state transitions.

2. **Explicit Lifecycle State Machine**:
   - Enforce the state machine: `open -> read/write -> seek -> lock -> metadata -> close -> cleanup`.
   - Resource release must occur exactly once. Double-close or double-unlink operations return typed `FsError` responses without panicking or leaking handles.

3. **POSIX Compatibility & Capabilities**:
   - Support standard file flags (`O_RDONLY`, `O_WRONLY`, `O_RDWR`, `O_CREAT`, `O_EXCL`, `O_TRUNC`, `O_APPEND`, `O_NONBLOCK`, `O_DIRECTORY`).
   - Implement Linux/POSIX capabilities (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, `CAP_FOWNER`) for privilege elevation checks.

4. **Locking & Concurrency**:
   - Provide non-blocking and blocking BSD `flock` locks alongside non-overlapping byte-range locks (`FileRangeLock`).
   - Lock teardown occurs automatically on descriptor closure or process exit.

### Consequences
- Eliminates file descriptor leaks, race conditions during `dup`/`dup2`, and dangling inode references.
- All storage I/O and file management operations are bounded, observable, and failure-safe.
