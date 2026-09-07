# AI Agent Semaphores Management Guide

## Overview
This wiki guide details counting and binary semaphores management protocols for AI coding agents operating on SigmaOS. It covers IPC namespace semaphore isolation (`SemaphoreObject`), POSIX/System V `semop`/`semctl` translation, NT Win32 kernel semaphores (`NtSemaphoreObject`), and Linux `eventfd` semaphore driver semantics (`EFD_SEMAPHORE`).

## Key Principles
1. **Namespace Isolation**: Semaphores are registered within `IpcNamespace` maps to enforce strict IPC boundary isolation across containers.
2. **Counter Upper Bounds**: All counting semaphores enforce upper limits (`current_value <= max_value`).
3. **Eventfd Decrements**: In `EFD_SEMAPHORE` mode, `read()` returns `1` and decrements counter by 1.

## Semaphore Creation (`src/ipc/ipc_namespace.rs`)
```rust
let ns = IpcNamespace::new(1);
let sem_id = ns.create_semaphore("sync_sem".to_string(), 3, 10, 1)?;
let sem = ns.get_semaphore(sem_id).unwrap();
assert_eq!(sem.current_value, 3);
```

## Related Documents
- `docs/AI_AGENT_SEMAPHORES_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_SEMAPHORES_MANAGEMENT_GUIDELINES.md`
- `docs/AI_AGENT_BINARY_SEMAPHORE_MANAGEMENT_ARCHITECTURE.md`
