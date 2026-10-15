# SigmaOS Resource Limit Enforcement Policy (`docs/process/RESOURCE_LIMITS.md`)

## 1. Scope & Objective

This document defines process resource limits (`rlimit`), tracking metrics, enforcement behaviors, and typed error returns in SigmaOS.

## 2. Resource Limits & Enforcement Matrix

| Resource | Enforced Property | Behavior on Exceeding Soft Limit | Failure Return / Signal |
|---|---|---|---|
| **`RLIMIT_CPU`** | Max CPU time in seconds | Generates `SIGXCPU` signal; terminates if unhandled. | `SIGXCPU` / `SIGKILL` |
| **`RLIMIT_AS` / `RLIMIT_DATA`** | Max virtual memory allocation | Memory allocation / `brk` / `mmap` refused. | `ENOMEM` |
| **`RLIMIT_NOFILE`** | Max open file descriptors | File opening (`open`, `socket`, `pipe`) refused. | `EMFILE` |
| **`RLIMIT_NPROC`** | Max processes for real UID | `fork` / `clone` refused. | `EAGAIN` / `ProcessTableError::TableFull` |
| **`RLIMIT_STACK`** | Max stack size | Guard page fault on stack expansion refusal. | `SIGSEGV` |

## 3. Limit Inheritance & Hard/Soft Limits

- **Soft Limit**: Value enforced by kernel operations.
- **Hard Limit**: Ceiling for soft limit; unprivileged processes can lower hard limit or raise soft limit up to hard limit. Privilege (`CAP_SYS_RESOURCE`) required to raise hard limit.
- **Inheritance**: `fork` and `exec` preserve process resource limit configurations.

## 4. Verification

Resource limit enforcement and typed errors are verified in `tests/process/rlimits/rlimit_test.rs`.
