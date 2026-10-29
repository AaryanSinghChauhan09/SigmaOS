# SigmaOS Wait & Process Reaping Policy

## Core Reaping Rules

1. **Authorized Reaper:** Only the legitimate parent process or an adopted subreaper/init process (PID 1) may reap a zombie process.
2. **Single-Reap Invariant:** A process entry may be reaped exactly once. Subsequent `wait` or `waitpid` calls targeting an already reaped process must return an error (`ECHILD` or `ProcessAlreadyReaped`).
3. **Exit Status Retention:** Exit status and resource usage metrics remain readable in the process table slot until consumed by a reap call.
4. **Orphan Adoption:** Upon parent termination, all child processes are reparented to the nearest registered subreaper or system init (PID 1).
5. **Resource Release:** Reaping triggers complete reclamation of the PID back to the PID allocator pool and removal of process table tracking.
