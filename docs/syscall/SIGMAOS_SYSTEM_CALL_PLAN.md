# SigmaOS System-Call Strategy & System-Call Plan

## Overview & Executive Summary

SigmaOS defines a 64-syscall inventory with user-space libc wrappers and architecture entry paths. This document outlines the comprehensive system-call ABI policy, existing gap analysis, proposed 17-PR sequence, recurring maintenance cadences, component rules, review/merge gates, and rollout/exit criteria to transition the syscall layer from compile-only status to a fully integrated, safe, and supported operating system boundary.

---

## 1. Existing Gaps and Proposed Policy

### Existing Gaps
- **Kernel-Side Dispatch:** Syscall ABI status is currently partial / compile-only in standalone targets; full user-mode dispatch and Ring 3 boundary handling require complete end-to-end integration.
- **Ring 3 User-Mode Transition:** x86_64 entry path (`SYSCALL`/`SYSRET` via `MSR_LSTAR`, `MSR_STAR`, `MSR_SFMASK`, `swapgs`) needs full validation with user-space execution tests.
- **Safety & Verification:** User-pointer dereferencing must strictly use fault-safe user-memory helpers with range validation.
- **Enforcement Boundary:** Security policies (Capabilities, Seccomp, Pledge, Unveil, Landlock) must be enforced at entry/dispatch rather than in libc shims.

### Core Policy Principles
1. **Stable ABI:**
   - Once published to userspace, syscall numbers, argument layouts, return conventions, error behavior, and semantics are immutable.
   - Extend existing calls via flags or size-versioned structures rather than reusing or altering published syscall numbers.
2. **Avoid Unnecessary Syscalls:**
   - Reuse existing syscalls, file-descriptor APIs, filesystem interfaces, or typed mechanisms before inventing new syscall numbers.
3. **Extensible Arguments:**
   - Prefer flags parameters for simple extensions and size-versioned structures for complex parameters.
   - Deterministically reject unknown flags or non-zero extension fields with `EINVAL`.
4. **Validate User Memory Safely:**
   - Never directly dereference userspace pointers in kernel code.
   - Validate address ranges and copy data through fault-safe helpers (`copy_from_user`, `copy_to_user`). Invalid memory returns defined error codes (`EFAULT`).
5. **Kernel Authorization:**
   - Enforce ownership, permissions, capabilities, namespace boundaries, and resource limits at kernel syscall dispatch.
6. **Portable ABI Types:**
   - Use fixed-width UAPI types (`u32`, `u64`, `i32`, `i64`). Explicitly define alignment, padding, and endianness. Avoid ABI-visible native `usize`, `long`, or raw pointers.
7. **File Descriptors for Kernel Objects:**
   - Prefer FD-backed handles for pollable objects; support `O_CLOEXEC` / close-on-exec and standard polling (`poll`/`epoll`/`kqueue`).
8. **No Kernel-to-Syscall Calls:**
   - Kernel code must call shared internal kernel helpers, never user-kernel boundary wrappers.
9. **Status Vocabulary:**
   - **Proposed:** Documented design without kernel implementation.
   - **Prototype:** Experimental kernel code without end-to-end integration.
   - **Integrated:** Verified user-to-kernel dispatch path on a documented boot target.
   - **Supported:** Integrated path backed by repeatable CI tests, user-memory safety checks, and public reference documentation.
10. **Security & Maintenance:**
    - Follow NIST SSDF lifecycle principles: secure development, protected build inputs, secured releases, and vulnerability response.

---

## 2. Proposed 17-PR Implementation Sequence

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(syscall): define ABI policy and component status` | Audit `kernel/syscalls/`, syscall ABI, architecture entry code, libc shims, `FEATURE_STATUS.toml`, and wiki. Document syscall ownership, ABI stability, supported archs, errors, status vocabulary, and gaps. | Every listed syscall maps to implementation, dispatch, wrapper, and tests—or is marked not integrated; Linux compatibility distinguished from POSIX. |
| 2 | `test(syscall): add ABI inventory consistency checker` | Add a checker comparing syscall numbers, names, dispatch entries, wrappers, and generated tables. Prevent duplicate numbers and stale entries. | CI rejects duplicate numbers, missing dispatch mappings, undocumented entries, or wrapper drift. |
| 3 | `test(syscall): add user-memory access and error-path test harness` | Establish safe test infrastructure for invalid pointers, unmapped ranges, read-only pages, integer overflow, cross-page buffers, and fault handling. | Invalid memory cannot panic kernel; results return `EFAULT`; tests run in unit mode and QEMU ring-3 mode. |
| 4 | `feat(syscall): implement kernel dispatch and unknown-call handling` | Connect architecture entry path to bounded syscall dispatch table; handle unknown numbers, disabled calls, and invalid arguments. | Ring-3 test invokes known call and gets expected result; unknown calls return `ENOSYS`; wrappers strictly isolate kernel logic. |
| 5 | `feat(syscall): validate x86_64 entry state and return path` | Verify register mapping, privilege transition, stack switching, saved state, return path, and syscall masking rules on x86_64 target. | QEMU test enters from Ring 3, makes repeated calls, and returns without state/register corruption. |
| 6 | `feat(syscall): add safe user-copy primitives` | Implement copy-in/copy-out and range validation tied to process address space; audit handlers for direct pointer access. | All handlers use safe helpers; copy faults return `EFAULT`; integer overflows and partial-copy semantics tested. |
| 7 | `feat(syscall): define stable UAPI types and error conventions` | Define UAPI headers, fixed-width types, flag conventions, structure versioning, errno mapping, and reserved number policy. | ABI layout tests assert sizes, alignment, and offsets; unknown flags and mismatched structures return errors; docs match UAPI. |
| 8 | `feat(syscall): integrate process lifecycle calls with scheduler` | Implement `exit`, PID/TID identity, `wait4`/reap, `spawn`/`execve` required to launch user processes. | QEMU test launches user task, exits, reaps, and verifies status/cleanup; permission failures tested; scheduler integrated. |
| 9 | `feat(syscall): integrate file descriptor and VFS operations` | Implement/verify `open`, `read`, `write`, `close`, `lseek`, `stat`, FD table inheritance, and VFS permission checks. | Ring-3 tests cover valid/invalid files, permissions, short I/O, close/reuse, and resource cleanup. |
| 10 | `feat(syscall): add pollable event and pipe interfaces` | Integrate `pipe`, event notification, and `poll`/`select`/`kqueue` as FD-backed objects; define readiness and EOF semantics. | Tests cover data flow, readiness, close, broken pipe, blocking, wakeups, and resource limits. |
| 11 | `feat(syscall): add signals and interruption semantics` | Define signal delivery, masks, interruption/restart rules, and interactions with blocking calls. | Tests cover permissions, invalid signal numbers, interrupted reads/waits, handler return, and restart. |
| 12 | `feat(syscall): enforce namespace and resource-control boundaries` | Connect namespace, cgroup/resource, and limit modules to dispatch and target objects. | Processes cannot inspect/modify other namespace objects without authorization; limits enforced in kernel paths. |
| 13 | `security(syscall): enforce seccomp and capability checks at dispatch` | Integrate Seccomp, Pledge, Landlock, and Capability policies at syscall entry; apply checks consistently. | Policy denial occurs before side effects; rule bypass through alternate paths tested; status marked Integrated when enforced. |
| 14 | `feat(libc): bind syscall wrappers to stable kernel ABI` | Align libc shim and inline-assembly wrappers with dispatch, register ABI, error mapping, and architecture target. | Wrapper tests verify register/stack conventions, return/error translation, and ABI constants. |
| 15 | `docs(syscall): add syscall reference and manual pages` | Document each implemented call, arguments, errors, permissions, blocking behavior, ABI stability, and manual pages. | Every public syscall has complete documentation matching tests; no placeholder marked supported without implementation. |
| 16 | `test(syscall): add architecture and userspace conformance matrix` | Add native and cross-architecture ABI/layout tests; use selftests and behavior references. | Each claimed architecture executes applicable tests; unsupported ABI combinations documented. |
| 17 | `ci(syscall): gate ABI changes and protect syscall ownership` | Add code ownership for syscall tables/UAPI/entry paths, required CI, branch protection, and review checklist. | ABI changes require security/ABI review, selftests pass, and protected branch rules enforced. |

---

## 3. Periodic Maintenance Cadence

| Cadence | Recurring Maintenance Task | Required Output |
|---|---|---|
| Every PR | `ci(syscall): run ABI and boundary checks` | Compilation/tests pass, syscall table consistency verified, UAPI layout tests pass. |
| Weekly | `maint(syscall): triage failures and ABI issues` | Test regressions and open interface issues triaged; no syscall number edits without ABI review. |
| Weekly | `test(syscall): review fuzz findings` | Crashes minimized; fixed findings added to regression test suite. |
| Monthly | `audit(syscall): reconcile dispatch, wrappers, docs` | Full inventory audit across numbers, dispatch, libc wrappers, manual pages, and status matrix. |
| Monthly | `security(syscall): review user-copy & permissions` | Audit updated handlers for direct pointer dereferences, integer overflow, and permission gates. |
| Monthly | `test(syscall): run QEMU Ring-3 smoke suite` | Boot logs and execution tests for entry/return, `write`, `exit`, invalid calls, and memory faults. |
| Quarterly | `docs(syscall): review ABI compatibility & deprecation` | Review public ABI promises, Linux/BSD/POSIX compatibility statements, and deprecation policy. |
| Quarterly | `test(syscall): assess architecture portability` | Audit calling conventions, structure layout, and 32/64-bit handling across supported targets. |
| Release Candidate | `release(syscall): certify userspace ABI claims` | Certified test matrix, ABI headers, libc build, target boot logs, selftest results, and compatibility statement. |
| Post-Incident | `fix(syscall): add regression coverage` | Root cause analysis, affected versions identified, regression test added, and backport decision recorded. |

---

## 4. Component Rules & Merge Gates

### Rules for Syscall Subsystems
- **Architecture Entry:** Document mechanism (`SYSCALL`/`SYSRET`, `SVC`, `ECALL`), argument registers, return register, clobbers, stack switching, and interrupt masking for each architecture separately.
- **Dispatch Table:** Maintain a single canonical source of truth for syscall mapping. Reserve numbers permanently once published.
- **Argument Decoding:** Validate and decode raw registers into typed kernel values before executing business logic.
- **User-Memory Layer:** Enforce strict separation between user-space pointers and kernel memory.
- **VFS & Filesystems:** Perform permission and access control checks against the effective calling context.
- **Process & Thread Operations:** Validate PID/TID parameters against current process credentials and namespace boundaries.
- **Security & Sandboxing:** Policy checks (Capability, Pledge, Unveil, Seccomp) must run *before* any state modifications.

### PR Review & Merge Checklist Template
```text
- [ ] ABI review completed before implementation is locked
- [ ] No direct userspace-pointer dereference (fault-safe copy used)
- [ ] Unknown flags/versions rejected deterministically with EINVAL
- [ ] Stable number and UAPI layout verified with unit tests
- [ ] Selftest and documentation included matching implementation
- [ ] No kernel code calls syscall entry points internally
- [ ] Required CI checks pass; review comments resolved
```

---

## 5. Rollout & Exit Criteria

### Milestone Criteria
1. **Integrated:** A real user process invokes kernel dispatch on a documented target, executing both successful and error return paths cleanly.
2. **Supported:** The ABI inventory, libc wrappers, architecture entry, safe user-memory layer, permission tests, and documentation are completely aligned, backed by automated CI test suites.
