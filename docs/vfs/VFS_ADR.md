# SigmaOS VFS Architecture Decision Record (VFS_ADR)

## Status
Accepted

## Context
SigmaOS requires a unified, safe, observable, and recoverable Virtual File System (VFS) architecture capable of supporting standard POSIX file operations, process isolation, descriptor inheritance, directory hierarchy traversal, capability-based permission evaluation, and byte-range advisory/mandatory file locking.

Earlier experimental implementations resulted in fragmented descriptor tables and inconsistent state handling during process execution or abnormal termination.

## Decisions

### 1. File Descriptor Table Architecture
- **Decision:** Implement a bounded, per-process `ProcessFdTable` storing `Option<FileDescriptor>` entries up to `MAX_DESCRIPTORS_PER_PROCESS` (1024).
- **Rationale:** Ensures race-free allocation (`lowest free FD`), prevents descriptor exhaustion attacks, and provides exact-once resource release on `close` or process termination.
- **Inheritance:** `fork()` clones the descriptor table while incrementing reference counts on underlying `FileHandle` instances. `execve()` inspects `FD_CLOEXEC` flags and releases marked descriptors.

### 2. Inode & Handle Reference Isolation
- **Decision:** Separate `FileDescriptor` (process-local view), `FileHandle` (shared open file description with offset and open flags), and `Inode` (filesystem metadata and storage object).
- **Rationale:** Supports POSIX `dup()`, `dup2()`, and cross-process descriptor sharing while maintaining independent file offsets where required or shared offsets across duplicated descriptors.

### 3. Inode Lifecycle & Delayed Unlinking
- **Decision:** Inodes maintain an explicit lifecycle state machine (`Allocated`, `UnlinkedPendingClose`, `Freed`).
- **Rationale:** Unlinking an open file removes the directory entry immediately while deferring physical storage reclamation until the final process holding a descriptor or handle to the inode closes it.

### 4. Permission & Capability Model
- **Decision:** Implement bitwise mode evaluation (`0rwx-rwx-rwx`) against effective process credentials (`UID`, `GID`, supplementary `GIDs`) combined with capability overrides (`CAP_DAC_OVERRIDE`, `CAP_FOWNER`).
- **Rationale:** Guarantees standard POSIX file permissions while allowing administrative capabilities to bypass checks deterministically.

### 5. File Range Lock Management
- **Decision:** Maintain a dedicated `FileLockManager` per inode tracking shared (read) and exclusive (write) range locks (`FileLock`).
- **Rationale:** Enforces non-overlapping lock checks, enqueues blocked lock requests failure-safely, and automatically cleans up held locks when process descriptors are closed.

### 6. Error Enforcement Policy
- **Decision:** All VFS and file management methods return strongly typed `Result<T, FsError>` values (`FsError::NotFound`, `PermissionDenied`, `InvalidFileDescriptor`, `NoFileDescriptorsAvailable`, etc.).
- **Rationale:** Prevents panic on I/O failure or resource exhaustion, making storage errors diagnosable and recoverable.

## Consequences
- Clean separation between process-local descriptors and kernel handles.
- Zero memory leaks under descriptor/inode stress tests.
- High observability via explicit error codes and lifecycle state tracking.
