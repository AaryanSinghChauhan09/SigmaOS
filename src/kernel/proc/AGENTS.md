# AI Agent Development Instructions for Kernel Process Lifecycle Subsystem (`src/kernel/proc/`)

This directory implements core process lifecycle state transitions, POSIX signal handling, ptrace debugging traps, cgroups v1/v2 controller accounting, and namespace isolation primitives for SigmaOS.

## Subsystem Architecture & Directives

1. **Process Lifecycle State Machine (`process_lifecycle.rs`)**
   - Valid state transitions: `Ready` -> `Running` -> `Blocked`/`Stopped` -> `Zombie` -> `Dead`.
   - Never remove a `Zombie` process entry before `waitpid` / `sys_wait4` cleanups have harvested exit codes to prevent process table ID leakage.

2. **Signal Handling & Masking (`signals.rs`)**
   - Uncatchable signals: `SIGKILL` (9) and `SIGSTOP` (19) must immediately trigger termination/suspension without invoking user-registered signal handlers.
   - Atomically restore signal masks (`sigset_t`) during `rt_sigreturn` frame unwinding.

3. **cgroups & Resource Accounting (`cgroups.rs`)**
   - Resource enforcement (CPU bandwidth, memory limits, pids max) must be non-blocking in hot path scheduler calls.
   - Support hierarchical resource inheritance for child tasks spawned via `clone` / `fork`.

4. **Namespace Isolation (`namespaces.rs`)**
   - Isolate PID, Mount, Network, UTS, IPC, and User namespaces (`CLONE_NEW*` flags).
   - Ensure root capability checks (`CAP_SYS_ADMIN`) are evaluated within the target user namespace.

5. **`no_std` Pure Rust Guidelines**
   - Maintain strict `no_std` compatibility using `core::` and `alloc::` modules.
