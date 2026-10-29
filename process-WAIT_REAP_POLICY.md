# SigmaOS Wait and Reaping Policy

## Rules & Invariants

Process status collection and resource reclamation in SigmaOS follow these rules:

1. **Reaper Authorization**:
   - Only the direct parent process or an adopted subreaper (e.g. PID 1 `init`) may reap a zombie process.
2. **Single-Reap Guarantee**:
   - A process entry can be reaped exactly once.
   - Subsequent or concurrent `waitpid` calls on an already reaped process return `ProcessStateError::InvalidStateTransition`.
3. **Exit Status Retention**:
   - The exit code remains readable in the process table while in `ZOMBIE` state.
   - Transition to `REAPED` extracts exit code and frees process table entry.
4. **Orphan Adoption**:
   - Parent termination causes all child processes to be atomically reparented to PID 1 or active subreaper (`PR_SET_CHILD_SUBREAPER`).
5. **Resource Release**:
   - Reaping releases PID back to `PidAllocator`, frees remaining kernel memory structures, and clears table entries.
6. **`waitpid` Blocking Modes**:
   - `WNOHANG`: Non-blocking check; returns immediately if no zombie child exists.
   - Default blocking: Blocks parent task until child process transitions to `ZOMBIE`.
