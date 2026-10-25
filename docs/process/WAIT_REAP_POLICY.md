# SigmaOS Wait and Reaping Policy

## Invariants & Rules

1. **Reaper Authorization**:
   - Only the legitimate parent process or an adopted subreaper/init (`PID 1`) may reap a process.
2. **Single-Reap Guarantee**:
   - A process entry can be reaped exactly once. Subsequent calls to `wait`/`waitpid` on an already reaped process fail safely with `ECHILD` or `ProcessStateError::ProcessAlreadyReaped`.
3. **Exit Status Retention**:
   - The exit status code and termination cause (`exit`, signal) remain readable in the process table entry as long as the process remains in `ZOMBIE` state.
4. **Orphan Adoption Protocol**:
   - Upon parent process termination, all its child processes (whether `RUNNING`, `SLEEPING`, or `ZOMBIE`) are reparented to `init` (`PID 1`) or an designated subreaper.
   - `init` actively reaps adopted zombie children to prevent resource leaks.
5. **Kernel Resource Cleanup**:
   - Successful reaping frees the process table entry slot, deallocates PID reservation, and releases remaining accounting structures.
6. **`waitpid` Execution Modes**:
   - **Blocking**: Suspends calling parent until target child exits.
   - **Non-blocking (`WNOHANG`)**: Returns immediately (`0`) if target child is running/alive.
   - **No-children Error**: Returns `ECHILD` if calling process has no matching child processes.
