# SigmaOS PID & Process Table Policy

## Overview

PID allocation and process table maintenance are central to safe concurrency and lifecycle management in SigmaOS.

## PID Allocation Invariants

1. **Race-Free Allocation**: PID selection and tracking are guarded by atomic bitmasks or concurrent mutex locks (`PidAllocator`).
2. **Deterministic PID Ranges**:
   - `PID 0`: Kernel idle task.
   - `PID 1`: Init process / default subreaper.
   - `PIDs 2..MAX_PID`: Dynamically allocated process IDs.
3. **Exhaustion Handling**: When all available PIDs are in use, `allocate()` returns `PidError::PidExhausted`.
4. **PID Reuse Protection**: A freed PID cannot be reused until the terminating process has fully transitioned to `REAPED`.
5. **Safe Lookup**: Concurrency-safe lookup operations (`get_process(pid)`) return strong arc pointers (`Arc<ProcessEntry>`) that prevent use-after-free during concurrent process exit.

## Process Table Policy

- **Bounded Capacity**: The process table is initialized with a maximum capacity bound to prevent kernel memory exhaustion.
- **Atomic State Access**: Status updates within entries use atomic locks (`Mutex<ProcessLifecycleTracker>`).
- **Subreaper Adoption**: When a parent process terminates, its child processes are automatically reparented to the designated subreaper or PID 1.
