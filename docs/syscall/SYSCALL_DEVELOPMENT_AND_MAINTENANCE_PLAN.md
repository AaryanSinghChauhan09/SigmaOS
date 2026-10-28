# SigmaOS System-Call Development and Maintenance Plan

## Architectural Vision & Gap Analysis

SigmaOS requires a secure, stable, extensible, and portable system-call interface bridging user space applications and kernel-mode services. While syscall numbers and libc wrappers are defined, kernel-side dispatch, x86_64 privilege transitions, and user-mode ring 3 isolation require systematic verification and evidence-driven integration. This plan establishes the authoritative ABI policy, a 17-PR sequence roadmap, periodic maintenance cadences, component rules, review/merge gates, and rollout/exit criteria for the system-call subsystem.

---

## Mandatory System-Call ABI Policies

All system-call interfaces in SigmaOS must strictly comply with the following 10 mandatory policies:

1. **Stable ABI:** Once published to userspace, syscall numbers, argument layouts, return conventions, error behavior, and semantics are permanently stable. Interfaces are added or versioned rather than silently modified or reassigned.
2. **Avoid Unnecessary Syscalls:** Prefer existing syscalls, file-descriptor operations, VFS interfaces, or typed kernel abstractions before introducing a new syscall. Avoid opaque multiplexer calls except for close analogues or simple flags.
3. **Extensible Arguments:** Simple extensions must use a flags parameter (rejecting unknown flags deterministically with `EINVAL`). Complex argument structures must use size-versioned layouts (rejecting undersized/oversized structures with `EINVAL` or `E2BIG`).
4. **Validate User Memory Safely:** Never directly dereference userspace pointers. Range checks and fault-safe user-memory helpers (`copy_from_user`, `copy_to_user`, `copy_string_from_user`) must be used exclusively. Faults must return `EFAULT` without panicking the kernel.
5. **Authorization in the Kernel:** Enforce object ownership, POSIX capabilities, namespace boundaries, and resource limits (`rlimits`) in kernel dispatch paths—not merely in libc wrappers or command-line utilities.
6. **Portable ABI Types:** Use fixed-width UAPI types (`u32`, `u64`, `i32`, `i64`). Explicitly define alignment, padding, endianness, and 32/64-bit compatibility behavior. Avoid ABI-visible native pointers or compiler-dependent types in public structures.
7. **File Descriptors for Kernel Objects:** Prefer file-descriptor-backed handles for pollable or manageable kernel objects. Document close-on-exec (`O_CLOEXEC` / `EFD_CLOEXEC` / `SOCK_CLOEXEC`) and readiness polling semantics.
8. **No Kernel-to-Syscall Calls:** Kernel subsystems must call shared internal kernel helpers. Syscall entry points serve strictly as user/kernel boundary wrappers and must never be invoked internally by kernel code.
9. **Status Means Tested Behavior:** "Integrated" status requires evidence of end-to-end userspace-to-kernel dispatch on a documented target. "Supported" status additionally requires repeatable test coverage, error-path validation, and documented compatibility limits.
10. **Security & Maintenance Lifecycle:** Adhere to NIST SP 800-218 (SSDF) principles: prepare secure development, protect software and build inputs, produce well-secured releases, and respond to security vulnerabilities proactively.

---

## 17-PR Sequence Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria | Status |
|---|---|---|---|---|
| 1 | `docs(syscall): define ABI policy and component status` | Audit `kernel/syscalls/`, syscall ABI, entry code, libc shims, and status matrix. Document syscall numbering, errors, and current gaps. | Every listed syscall maps to implementation, dispatch entry, wrapper, and tests—or is marked not integrated. | Supported |
| 2 | `test(syscall): add ABI inventory consistency checker` | Add automated checker comparing syscall numbers, names, dispatch entries, wrappers, and documentation tables. | CI rejects duplicate/reused numbers, missing dispatch entries, undocumented calls, or table drift. | Supported |
| 3 | `test(syscall): add user-memory access and error-path test harness` | Fault-injection test infrastructure for invalid pointers, unmapped ranges, read-only buffers, and overflowed lengths. | Invalid user memory cannot panic kernel; results return defined `EFAULT` errors; unit/QEMU tests pass. | Prototype |
| 4 | `feat(syscall): implement kernel dispatch and unknown-call handling` | Connect architecture entry path to bounded syscall dispatch table; define unknown number and disabled call policy (`ENOSYS`). | Ring 3 test invokes known call with correct result; unknown call returns `ENOSYS` cleanly. | Prototype |
| 5 | `feat(syscall): validate x86_64 entry state and return path` | Verify register mapping (`rax`, `rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9`), privilege transition, stack switching, saved state, and return path. | QEMU test enters from Ring 3, makes repeated calls, and returns without register/stack corruption. | Prototype |
| 6 | `feat(syscall): add safe user-copy primitives` | Implement central copy-in/copy-out and range-validation APIs tied to process address space boundaries. | All handlers use safe helpers; copy faults return `EFAULT`; integer overflow and partial-copy semantics are tested. | Prototype |
| 7 | `feat(syscall): define stable UAPI types and error conventions` | Define UAPI headers, fixed-width types, flag conventions, structure versioning, negative `errno` mapping, and reserved number policy. | ABI layout tests assert sizes, alignment, offsets; unknown flags and struct sizes behave as documented. | Supported |
| 8 | `feat(syscall): integrate process lifecycle calls with scheduler` | Connect `exit`, `getpid`, `getppid`, `fork`/`clone`, `wait4`, `execve` dependencies to launch real user tasks. | QEMU test launches user task, exits, reaps it, and verifies status/cleanup without orphan leaks. | Prototype |
| 9 | `feat(syscall): integrate file descriptor and VFS operations` | Add/verify `openat`, `read`, `write`, `close`, `lseek`, `fstat`, FD inheritance, and VFS permission checks. | Ring-3 tests cover valid files, invalid descriptors, permission failures, short I/O, close/reuse. | Prototype |
| 10 | `feat(syscall): add pollable event and pipe interfaces` | Integrate `pipe2`, `eventfd`, `epoll_create1`/`epoll_wait` as FD-backed objects with readiness polling. | Tests cover data flow, readiness, EOF, close, broken pipe, blocked operations, and wakeup reliability. | Prototype |
| 11 | `feat(syscall): add signals and interruption semantics` | Define signal delivery, masks, interruption/restart rules (`SA_RESTART`), `kill`, `tgkill`, `rt_sigaction`. | Tests cover permission checks, invalid signals, interrupted reads/waits, handler return, restart behavior. | Prototype |
| 12 | `feat(syscall): enforce namespace and resource-control boundaries` | Connect PID, mount, network namespace boundaries and resource limits (`setrlimit`/`getrlimit`, cgroups). | Process cannot inspect or modify another namespace's objects without authorization; limits enforced in kernel paths. | Prototype |
| 13 | `security(syscall): enforce seccomp and capability checks at dispatch` | Integrate seccomp-BPF filters, Landlock, OpenBSD pledge/unveil, and POSIX capabilities at dispatch. | Tests prove policy denial occurs before side effects; rule bypass through alternate syscall paths is tested. | Prototype |
| 14 | `feat(libc): bind syscall wrappers to stable kernel ABI` | Align inline-assembly libc shims with actual dispatch, register ABI, negative `errno` translation, and target architecture. | Wrapper tests verify register/stack conventions, return/error translation, and ABI constants. | Prototype |
| 15 | `docs(syscall): add syscall reference and manual pages` | Document each implemented call, arguments, errors, permissions, blocking behavior, ABI stability, and examples. | Every public syscall has complete documentation matching tests and implementation. | Supported |
| 16 | `test(syscall): add architecture and userspace conformance matrix` | Add native and cross-architecture ABI/layout tests across x86_64, AArch64, and RISC-V 64 targets. | Each claimed architecture executes applicable test suite; unsupported ABI combinations are explicitly documented. | Prototype |
| 17 | `ci(syscall): gate ABI changes and protect syscall ownership` | Add code ownership for UAPI headers/tables, required CI checks, branch protection, and ABI review checklist. | Every ABI change gets ABI/security review; selftests pass; protected branches enforce status checks. | Supported |

---

## Periodic Maintenance Cadence

To maintain software health, prevent regression, and secure the system-call boundary, maintainers observe the following periodic cadences:

- **Every PR:** Execute ABI inventory consistency checks, UAPI layout assertions, user-memory fault harness, and unit tests.
- **Weekly:** Triage test failures, fuzz findings, open ABI design questions, and assigned issue priorities.
- **Monthly:** Audit dispatch tables against libc shims, manual pages, `FEATURE_STATUS.toml`, and component status matrices. Perform security audits on user-pointer copy paths.
- **Quarterly:** Review ABI compatibility promises, cross-architecture calling conventions, 32/64-bit compatibility, and deprecation proposals.
- **Each Release Candidate:** Certify userspace ABI claims, run full QEMU ring 3 integration smoke suites, and verify zero regressions.
- **Post-Incident:** Add regression test coverage for resolved vulnerability or panic reports immediately.

---

## Rules for Syscall-Related Components

1. **Architecture Entry Code:** Each architecture specifies its trap/instruction mechanism (`SYSCALL`/`SYSRET` for x86_64, `SVC` for AArch64, `ECALL` for RISC-V), register allocations, saved user frame, stack transition, and interrupt state.
2. **Dispatch Table:** Bounded dispatch table with `ENOSYS` fallback for unassigned, out-of-bounds, or disabled calls.
3. **Argument Decoding:** Validate and convert raw register values into typed kernel structures before business logic execution.
4. **User Memory Safety:** Rely on fault-safe primitives (`copy_from_user`, `copy_to_user`). Address range checking alone is insufficient without fault trapping.
5. **VFS & File Descriptors:** Perform authorization checks against effective calling context; enforce descriptor table limits and close-on-exec flags.
6. **Process & Thread Boundaries:** Validate PID/TID references against active process tables and PID namespaces before object access.
7. **Signals & Interruption:** Handle `EINTR` returns and `SA_RESTART` behavior cleanly across all blocking syscalls.
8. **Security Enforcement:** Seccomp, Landlock, Pledge, and Capability filters must execute at dispatch entry before any kernel state modifications occur.

---

## PR Review & Merge Gate Checklist

Every system-call PR must supply the following gate checklist in its submission:

```text
## Summary
What user-visible capability or correctness issue does this PR address?

## ABI Proposal
Name, number/namespace, architecture(s), arguments, return values, errno/status,
blocking behavior, permissions, and compatibility considerations.

## Why a Syscall?
Explain why an existing syscall, FD operation, VFS interface, or typed API is
not appropriate.

## ABI Stability
Versioning/flags strategy, reserved fields, unknown-field behavior, and future
extension plan.

## Security
User-pointer handling, authorization, object lifetime, resource limits,
side effects, and failure cleanup.

## Implementation
Dispatch, architecture entry, shared helper, libc wrapper, and disabled-call behavior.

## Tests
Unit/model tests, invalid pointers, authorization failures, selftest, QEMU/Ring 3,
architectures/ABIs tested, and anything not tested.

## Documentation and Status
Man page/API docs, ABI inventory, feature matrix, and Proposed/Prototype/
Integrated/Supported evidence.

## Checklist
- [ ] ABI review completed before implementation is locked
- [ ] No direct userspace-pointer dereference
- [ ] Unknown flags/versions rejected deterministically
- [ ] Stable number and UAPI layout checked
- [ ] Selftest and documentation included
- [ ] No kernel code calls syscall entry points internally
- [ ] Required CI checks pass; review comments resolved
```

---

## Rollout & Exit Criteria

- **Integrated Status Criteria:**
  - Syscall entry path connected to kernel dispatch on a documented target.
  - Ring 3 user process successfully invokes the call and receives expected success/failure return values.
  - Safe user-copy primitives enforced across all argument buffers.

- **Supported Status Criteria:**
  - Complete test coverage including invalid pointer, buffer boundary, permission, and unknown argument fault cases.
  - UAPI fixed-width headers, libc shims, and public manual pages published and synchronized.
  - Zero duplicate numbers or table drift detected across CI test matrices.
