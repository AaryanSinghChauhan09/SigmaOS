# SigmaOS Fork, Clone, and Exec Contracts

## Contract Matrix

| Operation | Required Semantics & Behaviors |
|---|---|
| **`fork`** | Duplicate process state; copy or share address space (Copy-on-Write / CoW); duplicate file descriptor table; preserve signal dispositions and masks. |
| **`clone`** | Fine-grained thread or child creation with explicit resource sharing flags (`CLONE_VM`, `CLONE_FS`, `CLONE_FILES`, `CLONE_SIGHAND`, `CLONE_THREAD`). |
| **`exec`** | Replace current address space with new executable image (ELF); reset signal handlers to `SIG_DFL`; handle close-on-exec (`FD_CLOEXEC`) file descriptors; re-initialize stack, environment, and credentials. |
| **`exit`** | Close/flush open file descriptors; notify parent process via signal (`SIGCHLD`); set exit status; transition process state to `ZOMBIE`. |
| **`wait`** | Read exit status of zombie child; transition child from `ZOMBIE` to `REAPED`; free process table slot and PID; prevent double-reaping. |

## Invariants

1. **Child PID Uniqueness**: `fork()` always produces a unique PID for the child process while returning child PID to parent and `0` to child.
2. **Close-on-Exec Safety**: File descriptors marked with `FD_CLOEXEC` are automatically closed during `exec` processing before executing the entry point.
3. **Signal Handler Reset**: Signal handlers pointing to userland code in the old executable image are reset to default (`SIG_DFL`) during `exec` to avoid executing invalid addresses.
