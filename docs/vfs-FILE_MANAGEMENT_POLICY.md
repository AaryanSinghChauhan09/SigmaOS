# SigmaOS File Management Subsystem Policy

## Objective

Maintain SigmaOS file management as a safe, observable, recoverable subsystem: validate paths, enforce permissions, manage file descriptors and inodes, handle locks and events, and release resources exactly once.

The first dependency is a validated firmware-to-shell path. SigmaOS identifies boot-to-shell validation, recoverable installer/update paths, and connecting subsystem models to runtime paths as near-term priorities.

## Authoritative File Lifecycle Contract

SigmaOS defines one authoritative file lifecycle:

```text
open -> read/write -> seek -> lock -> metadata -> close -> cleanup
```

Every state defines required and deterministic runtime behavior:

| State | Required behavior |
|---|---|
| Open | File descriptor allocated, inode resolved, permissions checked |
| Read/Write | Requests validated, queued, completed exactly once |
| Seek | Offset updated race-safe |
| Lock | Lock lifecycle failure-safe |
| Metadata | Metadata updates coherent |
| Close | Descriptor and references released exactly once |
| Close-on-exec | Descriptor semantics explicit |
| Error | Failure does not corrupt filesystem state |

No storage path may be unbounded, and I/O errors must be typed, diagnosable, and propagated or explicitly handled.

## Subsystem Architecture & Classification

- **VFS Class**: Hybrid request-based and inode-backed VFS architecture (`src/fs/inode.rs`, `src/fs/fd.rs`).
- **File Descriptor Class**: FD-based process table with file handle abstraction (`ProcessFdTable` in `src/fs/fd_table.rs`).
- **Inode Class**: Inode-based reference counting with bounded directory entry tracking (`InodeManager` in `src/fs/inode.rs`).
- **Directory Semantics**: Bounded filename lengths (`MAX_FILENAME_LEN = 255`) and maximum entry bounds (`MAX_DIR_ENTRIES = 1024`) per directory (`src/fs/directory.rs`).
- **Permission Semantics**: POSIX mode bit masks combined with process credential evaluation (`ProcessCredentials`) and capability overrides (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, `CAP_FOWNER`) in `src/fs/permission.rs`.
- **Lock Semantics**: Non-blocking and blocking file range locks (`FileRangeLock`) and process flock management (`FileLockManager`) in `src/fs/flock.rs` / `src/fs/lock.rs`.
- **Metadata Semantics**: Coherent update of inode metadata (mode, uid, gid, size, nlink, mtime/ctime/atime) upon every modifying VFS call.
- **Error Policy**: Exhaustive typed error responses (`FsError` enum) for all VFS operations.
- **Limits**: Bounded process descriptor tables (`DEFAULT_MAX_FDS = 1024`), bounded directory entries, bounded cache pages, and explicit OOM metrics.
- **NUMA Behavior**: NUMA domain affinity bindings for buffer and block cache allocation when supported.
- **Hotplug Behavior**: Failure-safe unmount and device removal handling without orphan dangling references or kernel lockups.

## Periodic Task Calendar

| Frequency | File-management task | Output | Required PR |
|---|---|---|---|
| Every PR | Build, lint, library, and standalone tests | CI result | `feat`, `fix`, `test`, or `docs` PR |
| Daily | CI/nightly triage | Root cause + fix | `fix(vfs)` or `fix(filesystem)` PR |
| Weekly | File subsystem review | Gap list + status | `docs(file-management)` PR |
| Weekly | Linux/BSD file study | Study note | `docs(studies)` PR |
| Biweekly | Descriptor audit | Leak/race findings | `fix(fd_table)` / `test(fd_table)` PR |
| Weekly | Inode and mapping audit | Mapping findings | `fix(inode)` PR |
| Monthly | Boot-to-shell file validation | Boot log + smoke report | `test(vfs)` PR |
| Monthly | File stress/fuzz triage | Crash reports + fixes | `fix(vfs)` PR |
| Monthly | Lock rehearsal | Lock enforcement report | `test(flock)` PR |
| Quarterly | VFS architecture review | ADR + roadmap | `docs(adr)` PR |
| Quarterly | Security review | Threat-model update | `security(vfs)` PR |
| Quarterly | Release readiness review | Release checklist | `docs(release)` PR |

## File Health Metrics

| Metric | Target |
|---|---|
| File CI pass rate | 100% |
| Boot-to-boot pass rate | 100% |
| Orphan descriptors after stress test | 0 |
| Double-unlink incidents | 0 |
| Descriptor reuse races | 0 |
| File corruption incidents | 0 |
| Declared limits with enforcement tests | 100% |
| File claims linked to evidence | 100% |
| Security-sensitive PRs reviewed | 100% |
