# SigmaOS File Management Subsystem Policy

## Objective

Maintain SigmaOS file management as a safe, observable, recoverable subsystem: validate paths, enforce permissions, manage file descriptors and inodes, handle locks and events, and release resources exactly once.

## Authoritative File Lifecycle Contract

SigmaOS defines one authoritative file lifecycle contract:

```text
open -> read/write -> seek -> lock -> metadata -> close -> cleanup
```

### State Behaviors and Invariants

| State | Required Behavior | Invariants & Guarantees |
|---|---|---|
| **Open** | File descriptor allocated, inode resolved, permissions checked | Race-free FD allocation, path/permission validation before handle assignment. |
| **Read/Write** | Requests validated, queued, completed exactly once | Bounds checked against file limits and handle offsets, no partial corrupted writes. |
| **Seek** | Offset updated race-safe | Seek operation modifies atomic handle position, bounds checked against maximum file offset. |
| **Lock** | Lock lifecycle failure-safe | Enqueue/dequeue of flock/range locks are race-free, automatic cleanup on process termination. |
| **Metadata** | Metadata updates coherent | Inode mtime/ctime/atime and size updates synchronized, dirty metadata flushed cleanly. |
| **Close** | Descriptor and references released exactly once | FD freed, underlying file handle reference count decremented, no double-free. |
| **Close-on-exec** | Descriptor semantics explicit | Descriptors marked with `O_CLOEXEC` / `FD_CLOEXEC` are closed atomically during `execve`. |
| **Error** | Failure does not corrupt filesystem state | Errors are typed (`FsError`), diagnosable, and propagated without partial state leaking. |

## Subsystem Architecture & Classification

- **VFS Class**: Request-based, inode-indexed hybrid VFS layer.
- **File Descriptor Class**: Process-local fd-table mapping non-negative integer descriptors to shared `FileHandle` ref-counted references.
- **Inode Class**: Inode-based with unique 64-bit inode IDs and underlying filesystem drivers (SigmaFS, Btrfs, XFS, procfs, tmpfs).
- **Directory Semantics**: Bounded capacity, entries stored as `(name, inode_id, entry_type)` tuples, protected against double-unlink and lost-updates.
- **Permission Semantics**: POSIX mode bit mask (owner, group, other) + capability overrides (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, `CAP_FOWNER`).
- **Lock Semantics**: Advisory process-level (`flock`) and range-based byte locking (`FileRangeLock`) with non-blocking support (`LOCK_NB`).

## Periodic Task Calendar

| Frequency | File-management Task | Output | Required PR |
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

## File Subsystem Health Metrics

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
