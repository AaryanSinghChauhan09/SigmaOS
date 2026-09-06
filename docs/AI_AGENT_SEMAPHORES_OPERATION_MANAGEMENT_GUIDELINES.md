# AI Agent Semaphores Operation Management Guidelines

## Purpose
These guidelines define operational protocols, implementation patterns, and safety guardrails for AI coding agents performing or tuning semaphore wait/signal operations in SigmaOS.

---

## Directives for AI Agents

1. **Atomic Operation Bounds**:
   - Always verify $0 \le \text{current\_value} \le \text{max\_value}$ when incrementing semaphores.
   - Do NOT allow integer overflow/underflow during `sys_semop` execution.

2. **`SEM_UNDO` Registration**:
   - Register process undo adjustments when executing operations on critical shared system resources to prevent unrecoverable lockouts on process termination.

3. **Code Pattern: Semaphore Wait and Signal**:
```rust
// Atomic signal (increment value)
if let Some(sem) = ns.get_semaphore_mut(sem_id) {
    if sem.current_value < sem.max_value {
        sem.current_value += 1;
        // Wake up highest priority blocked thread
    }
}
```

4. **Testing and Verification**:
   - Run `./run_sigma_tests.sh` to confirm semaphore operation and IPC namespace unit tests pass.

---

## Related Files
- `src/ipc/ipc_namespace.rs`
- `docs/AI_AGENT_SEMAPHORES_OPERATION_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_SEMAPHORES_OPERATION_MANAGEMENT.md`
