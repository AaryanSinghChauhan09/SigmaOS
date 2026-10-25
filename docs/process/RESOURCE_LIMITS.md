# SigmaOS Resource Limits Policy (`rlimit`)

## Enforced Limits & Enforcement Behaviors

| Resource Limit | Enforcement Behavior & Error Code |
|---|---|
| **CPU Time (`RLIMIT_CPU`)** | Preempt process upon soft limit expiration (`SIGXCPU`); send `SIGKILL` upon hard limit breach. |
| **Memory (`RLIMIT_AS` / `RLIMIT_DATA`)** | Memory allocation requests (`brk`/`mmap`) beyond limit fail with `ENOMEM` / refusal. |
| **File Descriptors (`RLIMIT_NOFILE`)** | File descriptor allocation (`open`/`dup`/`pipe`) beyond limit fails with `EMFILE`. |
| **Process Count (`RLIMIT_NPROC`)** | `fork`/`clone` requests exceeding maximum user process count fail with `EAGAIN`. |
| **Stack Size (`RLIMIT_STACK`)** | Stack growth beyond limit triggers stack-guard fault and `SIGSEGV` signal. |
| **Open Files System-wide** | Global file table allocation refusal with `ENFILE`. |

## Invariants

- Soft limits can be modified by unprivileged processes up to the hard limit.
- Raising hard limits requires kernel administrative capability (`CAP_SYS_RESOURCE`).
- Limits are inherited across `fork()` and preserved across `exec()`.
