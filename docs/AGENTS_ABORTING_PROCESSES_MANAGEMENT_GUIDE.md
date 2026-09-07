# SigmaOS AI Agent Aborting Processes Operation Management Guide

This guide defines operational protocols, signal-triggered abort handling, core dump metadata generation, and resource cleanup guidelines for AI agents managing process aborts across SigmaOS.

---

## 1. Overview of Process Aborting in SigmaOS

In SigmaOS, process aborting occurs when an unhandled fatal signal (`SIGABRT`, `SIGSEGV`, `SIGILL`, `SIGBUS`, `SIGFPE`) is delivered or when `abort_process` in `src/process/advanced_process_control.rs` is invoked.

When a process aborts, the kernel MUST:
1. **Isolate & Freeze Process Threads:** Immediately halt all userland threads belonging to the target Process ID (PID).
2. **Capture Core Dump Metadata:** Record the faulting instruction pointer, memory fault address, CPU registers, and fatal signal number.
3. **Transition Process State:** Transition the job state to `Aborted` and set the process exit status (`128 + fatal_signal`).
4. **Reparent Orphaned Child Processes:** Reparent any active child processes to Init (PID 1) to prevent orphaned process leaks.
5. **Release Allocated Resources:** Unmap virtual memory areas (VMAs), close active file descriptors, and flush IPC message queues safely without triggering kernel panics.

---

## 2. Core Dump & Abort Protocols

AI agents implementing or handling process aborts MUST observe these rules:

1. **Non-Corrupting Core Dumps:** Core dump metadata capturing MUST NOT overwrite active kernel memory pages or crash kernel interrupt handlers.
2. **IPC Channel Cleanup:** When aborting a process connected to active POSIX Message Queues or `eventfd` descriptors, the kernel MUST notify peer endpoints that the IPC channel is closed.
3. **No Kernel Panics on Userland Aborts:** Unhandled userland process faults MUST result in clean process termination and core dump generation, NEVER triggering a kernel panic.

---

## 3. Abort Process Interface Pattern

```rust
impl JobControlLifecycleEngine {
    pub fn abort_process(
        &mut self,
        pid: usize,
        signal: u32,
        fault_addr: u64,
    ) -> Result<CoreDumpMetadata, ProcessControlError> {
        let job = self
            .jobs
            .get_mut(&pid)
            .ok_or(ProcessControlError::ProcessNotFound(pid))?;

        job.state = JobState::Aborted;

        let core = CoreDumpMetadata {
            pid,
            fatal_signal: signal,
            fault_addr,
            registers: vec![0xDEADBEEF, 0x12345678, fault_addr],
            memory_dump_size: 4096,
        };

        Ok(core)
    }
}
```
