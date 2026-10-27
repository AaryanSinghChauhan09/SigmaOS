# SigmaOS Thread Manager Subsystem Specification

## Overview

The SigmaOS Thread Manager provides canonical thread lifecycle management, saved CPU context management, kernel/user stack boundaries, priority/affinity controls, and thread-safe wait-channel synchronization.

## Thread States and Lifecycle

The thread manager defines a strict 7-state lifecycle transition model:

- `New`: Thread object allocated and initialized; ready for setup.
- `Ready`: In the CPU run queue waiting for scheduler dispatch.
- `Running`: Actively executing on a CPU core.
- `Blocked`: Waiting on an IPC object, timer, lock, or wait-channel signal.
- `Suspended`: Temporarily suspended by process control or debugger.
- `Zombie`: Terminated execution; awaiting parent process join or reaping.
- `Terminated`: Resources reclaimed; thread descriptor ready for deallocation.

## Ownership Categories

- **Kernel Threads**: Execute exclusively in kernel mode (`ring 0`), sharing the kernel address space.
- **User Threads**: Execute in user mode (`ring 3`), associated with a parent process address space.
- **Process Threads**: Main execution thread of a user process.
- **Detached Threads**: Reclaimed automatically upon exit without requiring an explicit join call.

## Subsystem Guarantees

- **Preemption Model**: Preemptive thread scheduling via periodic timer ticks and voluntary yield hooks.
- **Priority & Nice**: Priority ranges with dynamic boost and priority-inheritance mutex protocols.
- **Affinity Semantics**: Per-CPU core affinity bitmasks enforcing core pin constraints.
- **Stack Guards**: Configurable kernel and user stack bounds with canary guard regions.
- **Wait Channels**: Thread-safe sleep/wakeup channels with lock-order tracking and lost-wakeup prevention.

## Non-Goals & Limitations

- **POSIX Compliance**: POSIX thread API compliance (`pthread_*`) is modeled via syscall adapters but not fully claimable until libc integration tests pass.
- **Hardware Integration**: Runtime CPU context switching on real hardware depends on architecture-specific task state registers (TSS/GDT/IDT).

## Status

- Subsystem Status: `unit-tested`
- Test Coverage: 10 unit tests passing in `src/kernel/thread.rs`
- Tracked in `FEATURE_STATUS.toml` under `[kernel.thread_manager]`
