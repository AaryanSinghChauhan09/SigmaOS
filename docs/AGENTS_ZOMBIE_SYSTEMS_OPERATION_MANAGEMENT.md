# AI Agent Guidelines: Zombie Systems & Process Operation Management in SigmaOS

## 📌 1. Overview & Zombie Process Architecture

In **SigmaOS**, zombie processes (`ProcessState::Zombie` / `Z` state) are terminated processes whose virtual memory address space, page tables, and file descriptors have been unmapped, but whose Process Control Block (PCB) slot and exit status code (`exit_code`) remain in the kernel process table awaiting collection by a parent process.

As an AI agent developing process managers, init supervisors, or process control tools, you must ensure **rapid zombie harvesting, orphan adoption by Init PID 1, and zero PCB table slot leaks**.

---

## ⚙️ 2. Zombie Lifecycle & Resource Cleanup Stages

```
+-----------------------------------------------------------------------------------+
|                        PROCESS TERMINATION & ZOMBIE REAPING                       |
+-----------------------------------------------------------------------------------+
|  1. Process Exit (`exit(code)`)  ---> Free Page Tables, Unmap Memory, Close FDs   |
|  2. Transition to `Zombie`       ---> Retain PCB Slot + Exit Status Code          |
|  3. Parent Notification           ---> Send `SIGCHLD` Signal to Parent Process      |
|  4. Parent `waitpid()` Collection ---> Free PCB Slot & PID; Return Exit Code      |
+-----------------------------------------------------------------------------------+
```

### Two-Stage Resource Reclamation:
1. **Stage 1 (Immediate Execution Exit Time):**
   * Virtual memory space (`VirtualMemoryManager`), page directory roots, user stacks, and open file descriptors are unmapped immediately.
   * Transition process state to `ProcessState::Zombie`.
2. **Stage 2 (Collection Time via `waitpid`):**
   * Parent process collects exit status (`exit_code`).
   * Kernel releases the PCB entry and returns the PID to the free PID allocation pool.

---

## 👶 3. Orphan Process Adoption & Init PID 1 Reaper

* **Module Location:** `src/userland/init.rs`, `src/process/advanced_process_control.rs`

### 3.1 Orphan Re-Parenting
When a parent process terminates while its child processes are still running:
1. The process manager iterates through the active process hierarchy.
2. All children of the dying parent are automatically re-parented to `InitPID1` (`ppid = 1`).

### 3.2 Init PID 1 Async Reaper Loop
`InitPID1` operates a non-blocking background reaper loop to harvest orphaned zombies:

```rust
// Init PID 1 Non-Blocking Zombie Reaper Pattern
pub fn reap_orphaned_zombies(&mut self) -> usize {
    let mut reaped_count = 0;
    while let Ok(Some((reaped_pid, status))) = self.waitpid(-1, WNOHANG) {
        reaped_count += 1;
        SigmaDebug::log_trace("Init PID 1 reaped orphan zombie", reaped_pid);
    }
    reaped_count
}
```

---

## 🛡️ 4. POSIX `waitpid` Flags & Signal Controls

* **`SIGCHLD` Auto-Reap Behavior:**
  * If a parent process explicitly sets its `SIGCHLD` signal disposition to `SIG_IGN` or includes the `SA_NOCLDWAIT` flag, terminated child processes bypass the `Zombie` state and are reaped immediately upon exit.
* **`waitpid` Flag Semantics:**
  * `WNOHANG`: Return immediately with `0` if no child has exited.
  * `WUNTRACED`: Report stopped children (job control `SIGTSTP` / `SIGSTOP`).
  * `WCONTINUED`: Report resumed children (`SIGCONT`).

---

## 🚫 5. AI Agent Rules & Code Patterns

1. **Never Block Init PID 1:**
   * `InitPID1` MUST always invoke `waitpid()` with `WNOHANG`. Blocking Init on a single child halts orphan reaping for the entire operating system.
2. **Avoid PCB Slot Exhaustion:**
   * Ensure parent process loops invoke `waitpid()` or handle `SIGCHLD` to prevent un-reaped zombies from exhausting the kernel process table capacity ($MAX\_PROCESSES = 65536$).
3. **Double-Fork Daemonization Pattern:**
   * When spawning background daemons, use the double-fork technique (`fork() -> fork()`) so the daemon's intermediate parent exits immediately, re-parenting the daemon to `InitPID1` for automatic lifecycle management.

---

## 🧪 6. Standalone Testing Procedures

AI agents can verify zombie process transitions, `waitpid` collecting, and orphan re-parenting via standalone unit compilation:

```bash
# Test advanced process control, job control lifecycle, and waitpid harvesting
rustc --test --edition=2021 src/process/advanced_process_control.rs -o build/proc_tests && ./build/proc_tests && rm build/proc_tests

# Test Init PID 1 orphan reaper loop
rustc --test --edition=2021 src/userland/init.rs -o build/init_tests && ./build/init_tests && rm build/init_tests
```
