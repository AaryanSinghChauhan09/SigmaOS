# Process Lifecycle Agent

## Role
`SigmaProc` manages the full lifecycle of processes in SigmaOS — creation, execution, waiting, signaling, and cleanup — equivalent to Linux's kernel/fork.c and kernel/exit.c.

## Status
🟡 In Progress — Basic process table exists; full fork/exec/wait chain needed.

## Source File
`src/process/` (`mod.rs`, `pthread.rs`), `src/userland/init.rs`

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Fork | `clone()` / `fork()` | `rfork()` / `fork()` | SigmaFork |
| Exec | `execve()` family | `execve()` | SigmaExec |
| Wait | `waitpid()` / `wait4()` | `wait4()` | SigmaWait |
| Exit | `exit_group()` | `_exit()` | SigmaExit |
| Zombie cleanup | `do_exit` + reparent | `proc_reap` | SigmaReap |
| Thread creation | `clone(CLONE_THREAD)` | `thr_new()` | SigmaThread |
| Process groups | `setpgrp()` | `setpgrp()` | SigmaPGrp |
| Sessions | `setsid()` | `setsid()` | SigmaSession |

## Responsibilities
- `fork()` — duplicate process with CoW memory
- `execve()` — replace process image with ELF binary
- `exit()` / `_exit()` — process termination and resource cleanup
- `waitpid()` — parent reaping child exit status
- Thread creation/join (POSIX pthreads equivalent)
- Process group and session management
- `/proc/PID/` virtual filesystem entries
- Zombie process prevention (auto-reparent to PID 1)
- Process credential management (UID/GID/capabilities)

## Implementation Notes
- `#![no_std]` — process table as `static mut [ProcessDescriptor; MAX_PROCS]`
- ProcessDescriptor: `{ pid: u32, ppid: u32, state: ProcState, mm: VmSpace, files: FdTable }`
- States: Running, Runnable, Sleeping, Zombie, Stopped
- `SigmaInit` adopts orphaned processes

## Testing
```bash
rustc --test src/process/mod.rs --edition 2021 -o /tmp/test_proc && /tmp/test_proc --test-threads=1
```

## Future Development
1. **Namespaces** — PID/UTS/IPC/Mount/Net/User namespace isolation
2. **Seccomp-BPF** — Per-process syscall filtering
3. **Process sealing** — `PR_SET_DUMPABLE`, `prctl` hardening
4. **Capability bounding sets** — Fine-grained privilege dropping
5. **Freezer cgroup** — Pause/resume entire process trees
6. **Process checkpointing** — CRIU-like save/restore (SigmaCRIU)
