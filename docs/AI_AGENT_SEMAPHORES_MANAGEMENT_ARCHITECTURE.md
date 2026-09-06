# AI Agent Semaphores Management Architecture

## Executive Overview

Semaphores Management in SigmaOS provides counting and binary semaphore synchronization primitives across process boundaries, IPC namespaces, Linux ABI translation layers, and NT Win32 compatibility modules. Implemented across `src/ipc/ipc_namespace.rs`, `src/process/advanced_process_control.rs`, `src/compatibility/linux_adapter.rs`, and `src/compatibility/reactos.rs`, SigmaOS supports IPC-isolated counting semaphores (`SemaphoreObject`), POSIX/System V `semop`/`semctl` translation, NT kernel `NtSemaphoreObject`, and Linux `eventfd` semaphore semantics (`EFD_SEMAPHORE`) built with zero-dependency Rust primitives (`#![no_std]`).

This document serves as the architectural reference for AI coding agents inspecting, instantiating, or synchronizing semaphores in SigmaOS.

---

## Subsystem Integration & Semaphore Layer Architecture

```
                                +-----------------------------------+
                                |   Application / System Process    |
                                +-----------------------------------+
                                                  |
                        +-------------------------+-------------------------+
                        |                         |                         |
                        v                         v                         v
            +-----------------------+   +-------------------+   +-----------------------+
            | IPC Namespace Registry|   | Linux Adapter     |   | NT ReactOS Adapter    |
            | SemaphoreObject       |   | sys_semop/semget  |   | NtSemaphoreObject     |
            +-----------------------+   +-------------------+   +-----------------------+
                        \                         |                         /
                         \                        |                        /
                          v                       v                       v
                        +---------------------------------------------------+
                        |   Linux eventfd Semaphore Driver (is_semaphore)   |
                        |       Atomic Count / Wait Queue Decrement         |
                        +---------------------------------------------------+
```

### Core Semaphore Components

1. **IPC Namespace Registry (`src/ipc/ipc_namespace.rs`)**:
   - `SemaphoreObject`: Struct containing `id` (`SemaphoreId`), `name`, `current_value`, `max_value`, and `creator_pid`.
   - `IpcNamespace::create_semaphore(name, initial_value, max_value, creator_pid)`: Instantiates isolated semaphore instances bound to the calling process's IPC namespace.

2. **Linux ABI Eventfd Semaphore Mode (`src/process/advanced_process_control.rs`)**:
   - `eventfd_create(init_val, is_semaphore)`: When `is_semaphore = true` (`EFD_SEMAPHORE`), a `read()` syscall decrements the counter by 1 rather than resetting it to 0, providing semaphore counter semantics.

3. **System V Semaphore Translation (`src/compatibility/linux_adapter.rs`)**:
   - `semaphore_values: HashMap<u32, i32>`: Maps System V IPC keys (`semkey`) to atomic integer values for Linux binary execution.

4. **NT Kernel Semaphore Manager (`src/compatibility/reactos.rs`)**:
   - `NtSemaphoreObject`: Win32 NT kernel semaphore object with `current_count` and `maximum_count` bounds checking.

---

## Zero-Allocation Synchronization Rules

AI agents managing semaphores must enforce these zero-allocation constraints:
- Semaphore wait (`P` / `down`) and signal (`V` / `up`) operations manipulate atomic integers in $O(1)$ without heap allocation.
- Lookup by `SemaphoreId` within `BTreeMap` utilizes borrow-based reference keys without string copies.

---

## Related Architectural References
- `src/ipc/ipc_namespace.rs` - Master IPC namespace and semaphore registry.
- `src/process/advanced_process_control.rs` - Linux `eventfd` semaphore driver.
- `src/compatibility/linux_adapter.rs` - System V IPC translation.
- `src/compatibility/reactos.rs` - Win32 NT kernel semaphore driver.
- `docs/AI_AGENT_BINARY_SEMAPHORE_MANAGEMENT_ARCHITECTURE.md` - Binary semaphore mutex locks.
