# SigmaOS Wait and Reaping Policy (`docs/process/WAIT_REAP_POLICY.md`)

## 1. Scope & Objective

This document defines process exit handling, zombie retention, orphan adoption, and single-reap invariants for process lifecycle termination in SigmaOS.

## 2. Reaping Hierarchy & Authorization

1. **Reaper Ownership**: A process in `ZOMBIE` state may only be reaped by its direct parent process or an assigned subreaper (`PID 1` init daemon or designated subreaper process).
2. **Single-Reap Invariant**: A process entry can be reaped exactly once. Subsequent `wait` or `reap` calls on a reaped PID must fail safely with `ProcessTableError::ProcessNotFound` or `ProcessStateError::ProcessAlreadyReaped`.
3. **Exit Status Retention**: Until reaped, a zombie process retains its exit status, termination signal, and CPU timing metrics while releasing address space, file descriptors, and IPC channels.
4. **Parent Termination & Orphan Adoption**: When a parent process terminates while child processes are active or zombie, all child processes are atomically reparented to the nearest subreaper ancestor.

```text
Parent Exit Event:
  Parent (PID 10) [Exits]
    ├── Child A (PID 11) [Active]  ---> Reparented to Init (PID 1)
    └── Child B (PID 12) [Zombie]  ---> Reparented to Init (PID 1) & Reaped
```

## 3. Verification

Single-reap guarantees and orphan adoption rules are verified in `tests/process/wait_reap/wait_reap_test.rs`.
