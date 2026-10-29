# SigmaOS Fork, Clone, and Exec Contracts

## Contract Specifications

SigmaOS process creation and program execution follow strict contracts:

### 1. `fork` Contract

- **Address Space**: Copy-on-write or explicit address space duplication per memory policy.
- **File Descriptors**: Duplicates descriptor table references; increments open file description reference counts.
- **Credentials & State**: Inherits user/group credentials, signal masks, working directory, and resource limits.
- **PID**: Allocates a new unique PID for the child; returns child PID to parent and 0 to child.

### 2. `clone` Contract

- **Flag Control**: Supports explicit sharing flags:
  - `CLONE_VM`: Shared virtual memory address space (threading).
  - `CLONE_FS`: Shared filesystem info (root, cwd, umask).
  - `CLONE_FILES`: Shared file descriptor table.
  - `CLONE_SIGHAND`: Shared signal handlers.
  - `CLONE_NEWPID` / `CLONE_NEWNET` / `CLONE_NEWNS`: Isolation in new namespaces.

### 3. `exec` Contract

- **Image Replacement**: Replaces existing address space with executable ELF image binary segments.
- **File Descriptors**: Closes all file descriptors marked with `FD_CLOEXEC` / close-on-exec.
- **Signal Dispositions**: Resets non-ignored signal handlers to default dispositions.
- **Credentials & Environment**: Preserves or updates credentials, environment variables, argv arrays, and capability sets.

### 4. `exit` Contract

- **Resource Tear-Down**: Closes file descriptors, unmaps virtual memory regions, notifies child processes.
- **Parent Notification**: Delivers `SIGCHLD` signal to parent process.
- **State Transition**: Transitions tracker state to `ZOMBIE` and retains exit code.

### 5. `wait` / `waitpid` Contract

- **Reaping**: Parent or subreaper consumes zombie process status, transitions child to `REAPED`, and frees PID.
- **Double-Reap Prevention**: Guarantees exact-once reaping semantics. Duplicate calls return typed error.
