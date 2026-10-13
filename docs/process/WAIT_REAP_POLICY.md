# SigmaOS Wait and Reaping Policy

## Overview

The wait and reaping mechanism prevents resource leaks and guarantees accurate process termination status reporting.

## Reaping Rules

1. **Reaper Authorization:** Only the registered parent process or an adopted subreaper (`init` PID 1 or `PR_SET_SUBREAPER`) is authorized to reap a zombie child.
2. **Single Reap Guarantee:** `ProcessTable::reap_child` ensures that once a zombie is reaped, any subsequent reap attempt on the same PID fails cleanly with `ProcessTableError::ProcessAlreadyReaped` or `ProcessNotFound`.
3. **Status Preservation:** Exit code and termination signal details are preserved intact in the `ZOMBIE` process table entry until consumed by `wait`/`waitpid`.
4. **Orphan Adoption:** Upon parent process exit, all active child processes and unconsumed zombies are atomically reassigned to the parent's designated subreaper or `init` (PID 1).
5. **Nonblocking Options:** `waitpid` with `WNOHANG` returns immediately if no children have exited, avoiding thread blockages.
