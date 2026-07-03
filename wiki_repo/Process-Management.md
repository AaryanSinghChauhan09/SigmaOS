# Process Management

SigmaOS process management is implemented in `kernel/core/process_manager.rs`.

---

## Process Control Block (PCB)

Every process has a `Task` struct containing:

```rust
pub struct Task {
    pub pid:       u32,         // Process ID
    pub ppid:      u32,         // Parent PID
    pub state:     TaskState,   // Unused/Embryo/Sleeping/Runnable/Running/Zombie
    pub exit_code: i32,         // Exit status
    pub ctx:       TaskContext, // CPU register state for context switch
    pub name:      [u8; 32],    // Process name
    pub open_fds:  [i32; 256],  // Open file descriptor table
    pub cwd_ino:   u64,         // Current working directory inode
    pub uid:       u32,         // User ID
    pub kstack:    [u8; 65536], // 64 KB kernel stack
    pub sched_policy: u8,       // 0=MLFQ, 1=CFS, 2=EDF
    pub vruntime:     u64,      // CFS virtual runtime
    pub deadline:     u64,      // EDF absolute deadline (ns)
}
```

---

## Task States

```
          fork()
 UNUSED ──────────► EMBRYO ──── ready ──► RUNNABLE
                                               │
                                         context switch
                                               │
                              I/O wait ◄── RUNNING ──► exit() ──► ZOMBIE
                                │                                      │
                            wakeup                              wait() ──► UNUSED
                                │
                            SLEEPING
```

---

## System Calls

```c
// Create a child process (copy of current)
int child_pid = sigma_fork();
// parent: returns child PID
// child:  returns 0

// Terminate current process
sigma_exit(exit_code);

// Wait for a child to exit
int child_pid = sigma_wait4();

// Get current PID
uint32_t pid = sigma_getpid();

// Sleep for ms milliseconds
sigma_sleep_ms(100);
```

---

## Context Switch

Context switching is handled by `arch/x86_64/context_switch.asm`:

```
sigma_context_switch(from*, to*)
  1. Save RSP, R12-R15, RBP, RBX, RIP, CR3, RFLAGS to *from
  2. Load above from *to
  3. If CR3 differs → flush TLB (load new page table)
  4. Restore RFLAGS
  5. Jump to saved RIP (ret with pushed return address)
```

Per-task kernel stack (64 KB) ensures each task has its own call stack during
interrupt handling and kernel operations.

---

## Initial Tasks

At boot, two tasks are created:

| PID | Name | Policy | Description |
|-----|------|--------|-------------|
| 0 | `idle` | MLFQ Q3 | Runs `hlt` when nothing else is runnable |
| 1 | `init` | MLFQ Q0 | First userspace process, parent of all others |

---

## Maximum Limits

| Limit | Value |
|-------|-------|
| Max concurrent tasks | 256 |
| Open FDs per process | 256 |
| Kernel stack size | 64 KB |
| Max name length | 32 bytes |

---

*Source: `kernel/core/process_manager.rs` · `arch/x86_64/context_switch.asm`*
