# SigmaOS Fork, Clone, and Exec Contracts (`docs/process/FORK_CLONE_EXEC.md`)

## 1. Scope & Objective

This document specifies the legal behavior, resource sharing, address space transformations, and signal mask inheritances for process creation (`fork`, `clone`), image replacement (`exec`), and termination (`exit`, `wait`).

## 2. Operation Contracts

| Operation | Resource & State Semantics |
|---|---|
| **`fork`** | Duplicates calling process. Copies address space (Copy-on-Write), duplicates file descriptor table (`fd` refcounts incremented), copies signal dispositions/masks, and assigns a new unique PID. Returns `0` to child and child PID to parent. |
| **`clone`** | Explicit flag-driven thread/process creation. Supports selective sharing of address space (`CLONE_VM`), file descriptors (`CLONE_FILES`), signal handlers (`CLONE_SIGHAND`), and thread group (`CLONE_THREAD`). |
| **`exec`** | Replaces current executable image with target binary. Flashes virtual memory map, resets signal handlers (except ignored signals), closes file descriptors marked `FD_CLOEXEC`, retains PID, parent PID, environment, credentials, and signal mask. |
| **`exit`** | Terminates calling process. Closes all open file descriptors, releases address space mappings, sends `SIGCHLD` to parent, reparents active child processes to subreaper (`PID 1`), and transitions state to `ZOMBIE`. |
| **`wait`** | Reaps a terminated child process. Retrieves exit code, removes zombie entry from process table, and releases retained kernel accounting structures. Prevents double-reaping. |

## 3. Resource Invariants

- **CLOEXEC Handling**: Any file descriptor created with `FD_CLOEXEC` must be closed automatically during `exec` before control reaches the new entry point.
- **Copy-on-Write Isolation**: Memory modified by parent or child post-`fork` must trigger page fault copy-on-write isolation without corrupting peer address spaces.

## 4. Verification

`fork`, `exec`, and `clone` contracts are tested in `tests/process/fork_exec/fork_exec_test.rs`.
