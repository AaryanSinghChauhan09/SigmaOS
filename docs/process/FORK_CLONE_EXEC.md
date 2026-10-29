# SigmaOS Fork, Clone, Exec, Exit, and Wait Contracts

## Operations Contract

| Operation | Required Semantics & Invariants |
|---|---|
| **`fork`** | Duplicates calling process. Copies address space (Copy-on-Write) or shares memory per flags. Duplicates file descriptor table with incremented reference counts. Preserves signal dispositions, signal mask, and user credentials. Assigns new race-free PID. |
| **`clone`** | Fine-grained control over shared vs copied resources (VM, file system, file descriptor table, signal handlers, thread group ID). Explicitly defines whether caller shares task group or creates new process. |
| **`exec`** | Replaces active address space image with new ELF executable binary. Handles close-on-exec (`FD_CLOEXEC`) descriptors. Resets custom signal handlers to `SIG_DFL` while preserving ignored set. Resets thread stack and registers. |
| **`exit`** | Flushes pending buffers, closes all open file descriptors, detaches IPC resources, notifies parent process via `SIGCHLD`, and transitions process state to `ZOMBIE`. |
| **`wait` / `waitpid`** | Reaps designated child process in `ZOMBIE` state, extracts exit code and accounting metrics, transitions state to `REAPED`, and prevents double-reaping. |
