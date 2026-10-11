# 29. SigmaOS System-Call Strategy & Implementation Plan

## Overview
SigmaOS defines a 64-syscall inventory with user-space libc wrappers and architecture entry paths. This document details the syscall ABI policy, current gap analysis, 17-PR roadmap, maintenance cadence, component rules, merge gates, and rollout criteria.

---

## 1. Existing Gaps and Proposed Policy

### Key Gaps Identified
- **Kernel-Side Dispatch:** Syscall ABI status is partial / compile-only in standalone targets; complete Ring 3 boundary handling requires full runtime integration.
- **Ring 3 Transition Validation:** x86_64 entry path (`SYSCALL`/`SYSRET` via `MSR_LSTAR`, `MSR_STAR`, `MSR_SFMASK`, `swapgs`) requires validation with Ring 3 process execution.
- **Memory Safety:** Direct pointer dereferences must be replaced with fault-safe user-memory helpers (`copy_from_user`, `copy_to_user`).
- **Kernel Enforcement:** Security checks (Capabilities, Seccomp, Pledge, Unveil, Landlock) must be enforced at entry/dispatch rather than solely in libc.

### Proposed Policy Rules
1. **Stable ABI:** Published syscall numbers, argument layouts, return conventions, and error codes are immutable.
2. **Avoid Unnecessary Syscalls:** Prefer existing FD-backed or typed kernel APIs before allocating new syscall numbers.
3. **Extensible Arguments:** Use flags fields and size-versioned structures. Deterministically reject unknown flags with `EINVAL`.
4. **Safe User Memory Validation:** Never directly dereference userspace pointers. Range-check and copy through fault-safe helpers; return `EFAULT` on bad memory.
5. **Kernel Authorization:** Enforce credentials, permissions, and resource limits inside kernel dispatch handlers.
6. **Fixed-Width UAPI Types:** Use `u32`, `u64`, `i32`, `i64` in UAPI definitions to ensure cross-architecture layout consistency.
7. **FD Handles:** Return FD handles for pollable objects; support `O_CLOEXEC` and standard polling primitives.
8. **No Kernel-to-Syscall Calls:** Internal kernel subsystems call shared internal helpers, not user/kernel entry wrappers.
9. **Status Vocabulary:** Standardize statuses across Proposed, Prototype, Integrated, and Supported.
10. **Security Lifecycle:** Follow NIST SSDF principles for secure development and vulnerability management.

---

## 2. 17-PR Sequence Roadmap

1. `docs(syscall): define ABI policy and component status`
2. `test(syscall): add ABI inventory consistency checker`
3. `test(syscall): add user-memory access and error-path test harness`
4. `feat(syscall): implement kernel dispatch and unknown-call handling`
5. `feat(syscall): validate x86_64 entry state and return path`
6. `feat(syscall): add safe user-copy primitives`
7. `feat(syscall): define stable UAPI types and error conventions`
8. `feat(syscall): integrate process lifecycle calls with scheduler`
9. `feat(syscall): integrate file descriptor and VFS operations`
10. `feat(syscall): add pollable event and pipe interfaces`
11. `feat(syscall): add signals and interruption semantics`
12. `feat(syscall): enforce namespace and resource-control boundaries`
13. `security(syscall): enforce seccomp and capability checks at dispatch`
14. `feat(libc): bind syscall wrappers to stable kernel ABI`
15. `docs(syscall): add syscall reference and manual pages`
16. `test(syscall): add architecture and userspace conformance matrix`
17. `ci(syscall): gate ABI changes and protect syscall ownership`

---

## 3. Recurring Maintenance Cadences

- **Every PR:** `ci(syscall): run ABI and boundary checks`
- **Weekly:** Triage test regressions, review fuzzing findings.
- **Monthly:** Audit dispatch/wrapper/doc sync, security review user-copy paths, run QEMU Ring-3 smoke tests.
- **Quarterly:** Review public ABI deprecation policy, assess architecture portability.
- **Release Candidate:** Certify userspace ABI claims with test suite matrix.
- **Post-Incident:** Add regression test coverage for resolved issues.

---

## 4. Rollout and Exit Criteria

- **Integrated:** Real user process invokes kernel dispatch on documented target, verifying success and error paths.
- **Supported:** Inventory, libc wrappers, architecture entry, safe user-memory layer, permission tests, and documentation are aligned and verified in CI.
