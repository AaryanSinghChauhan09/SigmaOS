# Child Process Management Guidelines for AI Agents (`docs/ai_agents_child_process_management.md`)

This document provides AI agents with directives, architectural standards, Rust structures, and safety rules for managing **Child Processes** across the SigmaOS process management subsystem (`src/kernel/process.rs`).

---

## 1. Overview of Child Process Management in SigmaOS

In SigmaOS, process creation and hierarchy follow POSIX process lifecycle semantics. A parent process spawns a child process using the `fork()` method on `Process`, duplicating parent execution state, file descriptors, and security credentials while assigning a unique Process ID (`ProcessId`).

Key lifecycle phases:
1. **Forking (`fork()`):** Creates a child process with an isolated page table physical address (`new_pt_phys`) while inheriting parent environment metadata.
2. **State Management (`ProcessState`):** Tracks process lifecycle states (`New`, `Ready`, `BlockedWaiting`, `BlockedSuspended`, `Zombie`).
3. **Reaping / Harvesting (`waitpid()`):** Parent processes harvest child exit codes, freeing kernel process table entries.
4. **Orphan Reparenting:** When a parent process terminates before its children, surviving child processes are automatically reparented to `init` (`PID 1`).

---

## 2. Process Duplication & Inherited Attributes (`src/kernel/process.rs`)

When calling `Process::fork(&self, new_pt_phys: usize) -> Self`, the child process inherits the following attributes from its parent:

```rust
impl Process {
    pub fn fork(&self, new_pt_phys: usize) -> Self {
        let mut child = Process::new(new_pt_phys, &self.name);
        child.ppid = self.pid;             // Parent Process ID
        child.pgid = self.pgid;           // Process Group ID
        child.session_id = self.session_id; // Session ID
        child.uid = self.uid;               // User ID
        child.gid = self.gid;               // Group ID
        child.open_files = self.open_files.clone(); // File descriptor table
        child.sig_mask = self.sig_mask;     // Signal mask
        child.sig_actions = self.sig_actions.clone(); // Signal handlers
        child.brk = self.brk;               // Heap break pointer
        child.start_brk = self.start_brk;
        child.mmap_base = self.mmap_base;   // Memory mapping base
        child.cwd = self.cwd.clone();       // Current working directory
        child
    }
}
```

---

## 3. Child Process Harvesting & Reparenting

### 3.1 Harvesting Exit Codes
When a child process terminates, its exit status (`exit_code`) is recorded in the process table. Calling `waitpid()` retrieves the exit code and cleans up the child process record.

### 3.2 Reparenting to Init (`PID 1`)
If a parent process exits while child processes are still running, the kernel reparents orphaned processes to `init` (`PID 1`). Init periodically invokes `waitpid()` to reap terminated orphans and prevent resource leaks.

---

## 4. CLI Commands for AI Agents

AI agents can inspect process hierarchies and child process relationships using standardized shell commands:

```bash
# List process tree including PPID and state
sigma-ps tree --json

# Query child processes of a specific parent PID
sigma-ps children 1024 --json

# Harvest zombie children or terminate background process group
sigma-kill --pgid 1024 SIGTERM
```

---

## 5. Directives & Safety Rules for AI Agents

1. **Always Harvest Terminated Children:**
   Ensure parents invoke `waitpid()` or register SIGCHLD handlers to prevent process table accumulation.
2. **Isolate Page Tables for Forked Processes:**
   Child processes MUST receive a distinct, valid physical page table address (`new_pt_phys`) during `fork()` to enforce memory safety and CoW isolation.
3. **Preserve Credential Integrity:**
   Ensure child processes inherit appropriate User IDs (`uid`), Group IDs (`gid`), and capability sets without unintended privilege escalation.

---

## 6. Verification & Testing Procedure

When modifying process management or child process lifecycle logic:

1. **Run Process Management Unit Tests:**
   ```bash
   cargo test --lib kernel::process
   ```

2. **Run Full OS Test Suite:**
   ```bash
   ./run_sigma_tests.sh
   ```

---
*Maintained by the SigmaOS Core Kernel Process Team.*
