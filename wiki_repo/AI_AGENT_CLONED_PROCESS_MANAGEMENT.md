# AI Agent Cloned Process Management Guide

## Overview
This wiki guide details cloned process and execution thread management protocols for AI coding agents operating on SigmaOS. It covers POSIX `fork()`, POSIX `vfork()`, Linux-compatible `clone()` flags (`CLONE_VM`, `CLONE_FS`, `CLONE_FILES`, `CLONE_SIGHAND`, `CLONE_THREAD`), job object containment limits, and process table state transitions.

## Key Principles
1. **Zero-Dependency Cloning**: Kernel process creation operates entirely in pure Rust `#![no_std]` without libc dependency.
2. **Quota Enforcement**: Cloned processes must pass `JobObject` active process limits (`max_processes`) before PID allocation.
3. **COW Memory Copying**: Unshared memory clones (`CLONE_VM` clear) use Copy-on-Write page table duplication.

## Fork Implementation (`src/process/manager.rs`)
```rust
pub fn fork(&mut self, ppid: u32) -> Result<u32, ProcessError> {
    let parent = self.get_process(ppid)
        .cloned()
        .ok_or(ProcessError::NotFound)?;

    let new_pid = self.next_pid.fetch_add(1, Ordering::SeqCst);
    let mut child = ProcessInfo::new(new_pid, ppid, format!("{}-fork", parent.name));
    child.priority = parent.priority;
    child.resource_limits = parent.resource_limits.clone();

    self.processes.insert(new_pid, child);
    Ok(new_pid)
}
```

## Related Documents
- `docs/AI_AGENT_CLONED_PROCESS_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_CLONED_PROCESS_MANAGEMENT_GUIDELINES.md`
- `wiki/AI_AGENTS_PROCESS_MANAGEMENT_GUIDE.md`
