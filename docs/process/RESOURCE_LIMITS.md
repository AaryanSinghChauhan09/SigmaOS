# SigmaOS Resource Limit Enforcement Policy

## Overview

SigmaOS enforces resource bounds (`rlimit`) to protect kernel stability and prevent resource exhaustion or denial-of-service conditions.

## Managed Resource Limits

| Resource Limit | Enforcement Behavior |
|---|---|
| **CPU Time (`RLIMIT_CPU`)** | Sends `SIGXCPU` signal when soft limit is reached. If execution continues to hard limit, process is terminated with `SIGKILL`. |
| **Memory / Address Space (`RLIMIT_AS` / `RLIMIT_DATA`)** | Memory allocation requests exceeding limits return `ENOMEM` typed allocation error or invoke kernel OOM killer. |
| **File Descriptors (`RLIMIT_NOFILE`)** | `FdTable::allocate` returns `EMFILE` error when open file descriptor count meets or exceeds maximum limit. |
| **Processes (`RLIMIT_NPROC`)** | `fork`/`clone` returns `EAGAIN` or `PidError::ProcessLimitExceeded` when user exceeds process count limit. |
| **Stack Size (`RLIMIT_STACK`)** | Stack growth past soft limit triggers guard page fault and `SIGSEGV`. |
| **File Size (`RLIMIT_FSIZE`)** | File write operations past maximum size limit trigger `SIGXFSZ` signal. |
