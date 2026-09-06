# AI Agent Cloned Process Management Architecture

## Executive Overview

Cloned process management in SigmaOS provides fine-grained control over process and thread creation, address space duplication, file descriptor inheritance, signal handler sharing, and job object containment. Implemented across `src/process/manager.rs`, `src/process/spawn.rs`, `src/process/job_objects.rs`, and `src/process/linux_proc.rs`, SigmaOS supports POSIX `fork()`, POSIX `vfork()`, and Linux-compatible `clone()` semantics with zero-dependency Rust kernel primitives (`#![no_std]`).

This document serves as the architectural reference for AI coding agents inspecting, spawning, or managing cloned processes and execution threads in SigmaOS.

---

## Process Cloning Architecture & Flags

```
                                +-----------------------------------+
                                |    Parent Process (PID: N)        |
                                +-----------------------------------+
                                                  |
                                                  | sys_clone(flags, stack)
                                                  v
                                +-----------------------------------+
                                |    Cloned Process Engine          |
                                |    (src/process/manager.rs)       |
                                +-----------------------------------+
                                 /            |            \
                                /             |             \
            +----------------------+  +---------------+  +------------------------+
            | Memory Space (VM)    |  | FD Table      |  | Job Object Container   |
            | Shared (CLONE_VM) or |  | Shared or     |  | Limit Validation &     |
            | COWed Copy           |  | Duplicated    |  | Child Propagation      |
            +----------------------+  +---------------+  +------------------------+
                                \             |             /
                                 \            |            /
                                  v           v           v
                                +-----------------------------------+
                                |    Child Process (PID: N+1)       |
                                +-----------------------------------+
```

### Core Clone Flags & Semantics

1. **`CLONE_VM` (0x00000100)**:
   - If set, parent and child execution units share the same Virtual Memory address space. Memory writes in one thread are immediately visible to the other (POSIX thread semantics).
   - If clear, the parent's page tables are copied using Copy-on-Write (COW) page mapping rules (standard `fork()` semantics).

2. **`CLONE_FS` (0x00000200)**:
   - If set, parent and child share filesystem attributes (current working directory `cwd`, root directory `chroot`, and umask).

3. **`CLONE_FILES` (0x00000400)**:
   - If set, child inherits references to the parent's file descriptor table. Opening or closing descriptors in child alters parent state.
   - If clear, FD table is duplicated using deep structural copying (`clone()`).

4. **`CLONE_SIGHAND` (0x00000800)**:
   - If set, parent and child share signal handler disposition tables. Requires `CLONE_VM`.

5. **`CLONE_THREAD` (0x00001000)**:
   - If set, child is placed in the same thread group as the parent (TGID = parent PID). Child shares parent PPID and process signals.

---

## Subsystem Lifecycle & Job Object Containment

When `ProcessManager::fork(ppid)` or `fork_process(ppid)` is invoked:
1. **PID Generation**: Unique atomic allocation yields `new_pid`.
2. **State Duplication**:
   - `ProcessInfo` metadata (priority, state, resource limits) is duplicated from parent.
   - Name is formatted as `<parent_name>-fork`.
3. **Job Object Limit Enforcement (`src/process/job_objects.rs`)**:
   - `JobObject::handle_fork(parent_pid, child_pid)` validates whether adding `child_pid` violates process count quotas (`max_processes`).
   - If approved, `child_pid` is registered under all active job object containers associated with the parent.
   - If quota is exceeded, `JobLimitViolation::ActiveProcessLimitExceeded` aborts process creation before allocation.

---

## Memory Safety & Resource Cleanup

- **Zombie Process Handling**: Upon child exit, state transitions to `ProcessState::Zombie` until reaped by parent wait call (`sys_wait4`).
- **Resource Limits**: Cloned processes inherit parent's `ResourceLimits` struct (`max_memory_bytes`, `max_fds`, `cpu_affinity`).

---

## Related Architectural References
- `src/process/manager.rs` - Master process table and fork lifecycle.
- `src/process/job_objects.rs` - Cgroups v2 / job object container limits.
- `src/process/spawn.rs` - Process lifecycles and signal dispatch.
- `docs/AI_AGENT_PROCESS_MANAGEMENT_ARCHITECTURE.md` - EEVDF scheduler and task states.
