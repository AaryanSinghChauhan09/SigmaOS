# AI Agent Semaphores Operation Management Architecture

## Executive Overview

Semaphores Operation Management in SigmaOS governs atomic wait (`P` / `down`), signal (`V` / `up`), undo logging (`SEM_UNDO`), wait queue wakeups, and priority inheritance unblocking across System V IPC, POSIX semaphores, Linux `eventfd`, and NT kernel objects. Implemented across `src/ipc/ipc_namespace.rs`, `src/process/advanced_process_control.rs`, `src/compatibility/linux_adapter.rs`, and `src/compatibility/reactos.rs`, SigmaOS ensures zero-allocation atomic state transitions and thread synchronization built with `#![no_std]` Rust primitives.

This document serves as the architectural reference for AI coding agents inspecting, executing, or optimizing semaphore operations in SigmaOS.

---

## Subsystem Architecture & Operations Lifecycle

```
                                +-----------------------------------+
                                |    Process Thread Execution       |
                                +-----------------------------------+
                                           /             \
                   sys_semop(semop_array) /               \ sys_semop(semop_array)
                                         /                 \
                                        v                   v
                        +-----------------------+   +-----------------------+
                        |  Atomic Wait (P/down) |   | Atomic Signal (V/up)  |
                        | sem_val -= abs(op)    |   | sem_val += abs(op)    |
                        +-----------------------+   +-----------------------+
                                    |                           |
                            (if sem_val < 0)                    |
                                    v                           v
                        +-----------------------+   +-----------------------+
                        | Block Thread / Enqueue|   | Wakeup Blocked Thread |
                        | Priority Inheritance  |   | Process SEM_UNDO      |
                        +-----------------------+   +-----------------------+
```

### Core Operational Protocols

1. **Atomic Wait (`P` / `down` / `sys_semop` with $op < 0$ )**:
   - Decrements target `SemaphoreObject.current_value` by requested value.
   - If `current_value >= 0`, execution continues immediately.
   - If `current_value < 0`, calling thread is moved to `ProcessState::Blocked(BlockReason::SemaphoreWait)` and added to kernel wait queue.

2. **Atomic Signal (`V` / `up` / `sys_semop` with $op > 0$ )**:
   - Increments target `SemaphoreObject.current_value` by requested value up to `max_value`.
   - Wakes up highest-priority blocked process from the semaphore's wait queue.

3. **`SEM_UNDO` Auto-Reversal**:
   - Per-process undo tracking array records net semaphore operations performed with `SEM_UNDO` flag.
   - If process exits unexpectedly or crashes, kernel automatically reverses net adjustments to prevent permanent deadlock.

---

## Zero-Allocation Guardrails

AI agents modifying semaphore operations must adhere to these zero-allocation constraints:
- Wait and signal counter updates use in-place atomic fetch-add/fetch-sub primitives.
- Wait queue manipulation uses ring buffers or slab-allocated node links without heap reallocation.

---

## Related Architectural References
- `src/ipc/ipc_namespace.rs` - Master IPC namespace and semaphore object map.
- `docs/AI_AGENT_SEMAPHORES_MANAGEMENT_ARCHITECTURE.md` - Semaphores structure and creation architecture.
- `docs/AI_AGENT_BINARY_SEMAPHORE_MANAGEMENT_ARCHITECTURE.md` - Mutex binary semaphore locks.
