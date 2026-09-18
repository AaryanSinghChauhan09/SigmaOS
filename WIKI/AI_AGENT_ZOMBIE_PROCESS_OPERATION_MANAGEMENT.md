# 🧟 AI Agent Zombie Process Operation Management in SigmaOS

## Executive Summary
A zombie process (`TASK_ZOMBIE` state) in SigmaOS occurs when a process terminates execution via `exit()` or an unhandled signal, but its parent process has not yet collected its exit status via `wait()`, `waitpid()`, or `waitid()`. While a zombie process releases its address space memory, page tables, and file descriptors, its Process Control Block (PCB) entry and PID remain in the process table to hold the termination status (`wstatus`). Autonomous AI Agents (**Bolt ⚡**, **Palette 🎨**, and **Sentinel 🛡️**) managing process trees, containers, and service supervisors must manage zombie process lifecycles properly to prevent process table PID exhaustion and resource leaks.

---

## 1. Zombie Process Lifecycle & States

```
[ Active Process ] ──► sys_exit(code) ──► [ TASK_ZOMBIE ]
                                               │
               ┌───────────────────────────────┴───────────────────────────────┐
               ▼                                                               ▼
  Parent calls `waitpid()`                                       Parent exits without waiting
  • `wstatus` collected                                           • Process orphaned
  • PCB freed & PID recycled                                     • Re-parented to `init` (PID 1)
  • Transition to `TASK_DEAD`                                     • `init` reaper calls `waitpid()`
                                                                  • PCB freed & PID recycled
```

### Key Stages
1. **Termination & State Transition**: The kernel sets process state to `TASK_ZOMBIE`, closes open file descriptors, releases VMM virtual memory pages, and posts `SIGCHLD` signal to the parent process.
2. **Exit Code Retention**: The exit code (0–255) or terminating signal number is stored in `task_struct.exit_code`.
3. **Reaping (`waitpid`)**: The parent receives the termination status, and the kernel deallocates the remaining PCB slab memory and returns the PID to the PID allocator.
4. **Orphan Adoption**: If a parent terminates before its children, the kernel re-parents the children to the system `init` daemon (PID 1) or an explicit subreaper process (`PR_SET_CHILD_SUBREAPER`), which periodically reaps zombies.

---

## 2. Signal Handling & Automatic Reaping (`SIG_IGN`)

To avoid zombie accumulation without explicit `waitpid()` loops:
- **`SIGCHLD` Explicit Ignore (`SIG_IGN`)**: Setting the `SIGCHLD` signal handler disposition to `SIG_IGN` instructs the kernel to automatically reap child processes immediately upon termination without entering `TASK_ZOMBIE`.
- **`SA_NOCLDWAIT` Flag**: Registering a signal action with the `SA_NOCLDWAIT` flag prevents zombie creation for child processes.

---

## 3. AI Agent Operational Guidelines

1. **Bolt ⚡ (Performance & Resource Management)**:
   - Monitor total system PID usage and alert if zombie counts exceed threshold (`zombie_count > 128`), indicating a leaking parent supervisor process.
   - Implement asynchronous `SIGCHLD` handler loops using `waitpid(-1, &status, WNOHANG)` to reap terminated children in $O(1)$ constant time without blocking worker threads.

2. **Palette 🎨 (UX & Monitoring)**:
   - Display zombie process counts accurately in process monitoring interfaces (`htop`, `top`, control center) with distinct color highlights (e.g. `Z` state marker).

3. **Sentinel 🛡️ (Security & Hardening)**:
   - Prevent PID exhaustion Denial-of-Service (DoS) attacks caused by rogue fork-bomb or un-reaped processes filling the PID table (`MAX_PID`).
   - Ensure container runtime entrypoints (such as `SigmaContainer`) establish a lightweight reaper process to handle containerized subprocess termination.
