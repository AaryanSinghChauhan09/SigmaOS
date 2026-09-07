# AI Agent Semaphores Management Guidelines

## Purpose
These guidelines define operational rules, code patterns, and safety guardrails for AI coding agents creating, managing, or isolating counting semaphores in SigmaOS.

---

## Directives for AI Agents

1. **IPC Namespace Isolation**:
   - Always create semaphores within the active process's `IpcNamespace` to prevent unauthorized cross-container signal injection.
   - Verify `current_value <= max_value` upon instantiation.

2. **Eventfd Semaphore Semantics**:
   - Specify `is_semaphore = true` when creating `eventfd` descriptors intended for unit counter signaling (`EFD_SEMAPHORE`).

3. **Code Pattern: Creating and Inspecting Semaphores**:
```rust
let ns = IpcNamespace::new(1);
let sem_id = ns.create_semaphore("worker_sem".to_string(), 5, 10, 1)?;

let sem = ns.get_semaphore(sem_id).expect("Semaphore missing");
assert_eq!(sem.current_value, 5);
assert_eq!(sem.max_value, 10);
```

4. **Testing and Verification**:
   - Run `./run_sigma_tests.sh` to confirm semaphore isolation and IPC namespace unit tests pass.

---

## Related Files
- `src/ipc/ipc_namespace.rs`
- `src/process/advanced_process_control.rs`
- `docs/AI_AGENT_SEMAPHORES_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_SEMAPHORES_MANAGEMENT.md`
