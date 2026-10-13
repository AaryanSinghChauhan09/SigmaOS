# SigmaOS Fork, Clone, Exec, Exit, and Wait Contracts

## Overview

SigmaOS defines strict POSIX-compatible contracts for process creation (`fork`, `clone`), execution replacement (`exec`), termination (`exit`), and status consumption (`wait`).

## Subsystem Contracts

| Operation | Required Contract |
|---|---|
| **fork** | Duplicates the calling process. Copies address space (Copy-on-Write), duplicates file descriptor table entries with incremented reference counts, preserves signal dispositions/masks, inherits credentials and resource limits. |
| **clone** | Creates a child execution context with caller-specified resource sharing flags (`CLONE_VM`, `CLONE_FS`, `CLONE_FILES`, `CLONE_SIGHAND`, `CLONE_THREAD`). |
| **exec** | Replaces current address space with an ELF executable image. Flushes existing memory mappings, resets signal handlers (except ignored signals), closes file descriptors marked `FD_CLOEXEC`, updates process name/args/environment, and preserves PID and credentials. |
| **exit** | Initiates process termination. Closes open file descriptors, notifies active children and reparents orphans to `init`/subreaper, sends `SIGCHLD` to parent, and transitions process state to `ZOMBIE`. |
| **wait / waitpid** | Blocks (or polls if `WNOHANG`) for state changes in child processes. On reaping a zombie child, retrieves exit code/signal status, transitions child to `REAPED`, and releases PID and kernel accounting structures. |
