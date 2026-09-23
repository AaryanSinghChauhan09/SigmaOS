# AI Agent Cloned Process Management Guidelines

## Purpose
These guidelines define operational rules, code patterns, and safety constraints for AI coding agents managing, spawning, or modifying cloned process hierarchies in SigmaOS.

---

## Directives for AI Agents

1. **Job Object Containment**:
   - Always notify job objects when spawning or cloning processes via `JobObject::handle_fork(parent_pid, child_pid)`.
   - Never bypass job object process quota checks.

2. **Clone Flag Validation**:
   - `CLONE_THREAD` requires `CLONE_SIGHAND`, which in turn requires `CLONE_VM`.
   - Ensure thread group IDs (TGID) are assigned correctly when `CLONE_THREAD` is specified.

3. **Code Pattern: Process Forking**:
```rust
let mut pm = ProcessManager::new();
// Fork parent PID 1
let child_pid = pm.fork(1)?;

// Check process state
if let Some(child_info) = pm.get_process(child_pid) {
    assert_eq!(child_info.ppid, 1);
    assert_eq!(child_info.state, ProcessState::Ready);
}
```

4. **Testing and Verification**:
   - Execute `./run_sigma_tests.sh` to verify process lifecycle unit tests.
   - Test child process cleanup and job object propagation under load.

---

## Related Files
- `src/process/manager.rs`
- `src/process/job_objects.rs`
- `docs/AI_AGENT_CLONED_PROCESS_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_CLONED_PROCESS_MANAGEMENT.md`
