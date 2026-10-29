# SigmaOS Resource Limit Enforcement Policy

## Rlimit Specifications & Enforcement

Resource bounds are enforced across all process operations in SigmaOS:

| Limit (`rlimit`) | Enforcement Mechanism | Failure Code |
|---|---|---|
| `RLIMIT_CPU` | Monitored by timer interrupt; sends `SIGXCPU` then `SIGKILL` on breach. | `SIGXCPU` / `SIGKILL` |
| `RLIMIT_AS` / `RLIMIT_DATA` | Virtual memory page mapping check; rejects allocations exceeding limit. | `ENOMEM` |
| `RLIMIT_NOFILE` | File descriptor table allocation check during `open`/`dup`/`pipe`. | `EMFILE` |
| `RLIMIT_NPROC` | Process table PID allocation check during `fork`/`clone`. | `EAGAIN` |
| `RLIMIT_STACK` | Stack guard page hit checks stack size against ceiling before expanding. | `SIGSEGV` |
| `RLIMIT_FSIZE` | File write bytes check; sends `SIGXFSZ` if write exceeds limit. | `EFBIG` / `SIGXFSZ` |

## Enforcement Invariants

- Limits are inherited across `fork` and preserved across `exec`.
- Soft limits can be modified up to hard limit without superuser privilege.
- Hard limits require explicit capability (`CAP_SYS_RESOURCE`) to raise.
