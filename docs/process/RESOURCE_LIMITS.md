# SigmaOS Resource Limit Enforcement Policy

## Managed Limits & Behavior

SigmaOS enforces resource bounds per process (`rlimit`) to ensure system stability and prevent resource exhaustion:

| Resource Limit | Enforcement & Failure Behavior |
|---|---|
| **CPU Time (`RLIMIT_CPU`)** | Preempts execution upon soft limit; delivers `SIGXCPU`. Terminates process with `SIGKILL` upon reaching hard limit. |
| **Memory / Address Space (`RLIMIT_AS` / `RLIMIT_DATA`)** | Memory allocation requests (`mmap`, `brk`, `sbrk`) exceeding limit return allocation refusal (`ENOMEM`). |
| **File Descriptors (`RLIMIT_NOFILE`)** | `open`, `socket`, `pipe`, `dup` calls exceeding open file count return typed error `EMFILE`. |
| **Process Count (`RLIMIT_NPROC`)** | `fork` / `clone` calls by user exceeding maximum active processes return `EAGAIN`. |
| **Stack Size (`RLIMIT_STACK`)** | Stack growth exceeding boundary hits stack guard page, raising `SIGSEGV`. |
| **System Open Files (`sys.fs.file-max`)** | System-wide descriptor allocation failure returns `ENFILE`. |
