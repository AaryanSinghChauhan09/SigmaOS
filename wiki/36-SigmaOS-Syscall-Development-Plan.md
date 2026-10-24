# 36. SigmaOS System-Call Development and Maintenance Plan

## Executive Summary

The **SigmaOS System-Call Development and Maintenance Plan** establishes the authoritative engineering strategy, ABI governance principles, implementation sequence, component rules, periodic maintenance cadences, and merge gates for the SigmaOS kernel system-call subsystem.

Aligned with Linux ABI stability guidance (e.g., stable userspace contracts, safe pointer dereferencing primitives, size-versioned arguments, and file-descriptor handle conventions) and POSIX.1-2024 standards, this plan transitions SigmaOS from compile-only syscall stubs to an evidence-driven, fully dispatched, Ring 3 validated, and secure kernel interface.

---

## 1. Mandatory ABI Governance Policies

To ensure long-term stability, user-kernel boundary isolation, and robust security, all system-call implementations in SigmaOS must adhere to the following ten mandatory rules:

1. **Stable ABI:** Once published to userspace, system call numbers, argument layouts, return register conventions, error codes, and semantics are permanently stable. No published syscall number may be silently altered or reused.
2. **Avoid Unnecessary Syscalls:** New capabilities must first consider existing syscalls, file-descriptor operations, or VFS interfaces before introducing a new system call. Opaque "multiplexer" calls are prohibited unless strictly mirroring standard historical interfaces.
3. **Extensible Arguments:** Interfaces must support future growth using flags bitmasks or size-versioned structures. Unknown flags and unrecognized nonzero struct extension fields must be deterministically rejected with `EINVAL` / `E2BIG`.
4. **Validate User Memory Safely:** User-space pointers must never be directly dereferenced. Range checks and fault-safe copy primitives (`copy_from_user` / `copy_to_user`) must be used exclusively, returning `EFAULT` on invalid memory without kernel panic or state corruption.
5. **Authorization in the Kernel:** Permissions, object capabilities, ownership, cgroups, and resource limits must be enforced inside the kernel syscall entry or shared kernel helper—never relying solely on userland libc wrappers or CLI tools.
6. **Portable UAPI Types:** Interfaces must exclusively use fixed-width UAPI types (`u32`, `u64`, `i32`, `i64`). Padding, alignment, endianness, and 32/64-bit compatibility must be explicitly defined and tested. Native pointer types or compiler-dependent layouts are forbidden in UAPI headers.
7. **File Descriptors for Kernel Objects:** Pollable or operable kernel resources must be exposed as file descriptors, supporting standard close-on-exec (`O_CLOEXEC`) flags and non-blocking I/O semantics.
8. **No Kernel-to-Syscall Calls:** Kernel subsystems must invoke shared internal functions. System call entry points are strict boundary wrappers and must never be called internally by kernel code.
9. **Status Means Tested Behavior:** A syscall subsystem is labeled "Integrated" only when a real user process executes user-to-kernel entry on a documented boot target. It is "Supported" only when repeatable test coverage and documented limits exist.
10. **Security & SSDF Lifecycle:** Security controls follow NIST SP 800-218 (SSDF), ensuring secure build inputs, boundary validation, threat modeling, and proactive vulnerability response.

---

## 2. Proposed PR Implementation Sequence (17 PRs)

The implementation of the system-call subsystem follows a strict 17-PR sequence ordered by dependency:

| PR | Title | Scope & Objectives | Acceptance Criteria |
|---|---|---|---|
| **PR 1** | `docs(syscall): define ABI policy and component status` | Audit existing `kernel/syscalls/`, architecture entry code, libc shims, and `FEATURE_STATUS.toml`. Document syscall number ownership, ABI stability, supported targets, error mapping, and gaps. | Every listed syscall maps to implementation, dispatch entry, wrapper, and tests—or is marked not integrated; POSIX/Linux interfaces distinguished. |
| **PR 2** | `test(syscall): add ABI inventory consistency checker` | Implement automated checker comparing syscall numbers, names, dispatch entries, libc wrappers, and tables. | CI rejects duplicate numbers, missing dispatch entries, undocumented calls, or table drift. |
| **PR 3** | `test(syscall): add user-memory access and error-path test harness` | Create fault-injection infrastructure for invalid user pointers, unmapped pages, read-only buffers, length overflows, and output copy faults. | Invalid memory cannot panic or corrupt kernel state; results return `EFAULT`; tests run in unit mode and QEMU. |
| **PR 4** | `feat(syscall): implement kernel dispatch and unknown-call handling` | Connect architecture entry points to a bounded syscall dispatch table; handle unknown numbers (`ENOSYS`) and disabled calls. | Ring 3 test invokes known call and receives expected output; unknown call returns `ENOSYS`; no direct internal kernel calls. |
| **PR 5** | `feat(syscall): validate x86_64 entry state and return path` | Verify register mapping (`rax`, `rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9`), privilege transition (`SYSCALL`/`SYSRET`), kernel stack switching, saved state, and interrupt masking rules. | QEMU test enters from Ring 3, makes repeated calls, and returns without register/stack corruption. |
| **PR 6** | `feat(syscall): add safe user-copy primitives` | Implement central copy-in, copy-out, and string-copy helpers tied to process address space validation; audit current handlers for direct pointer access. | All handlers use safe helpers; copy faults return `EFAULT`; integer overflow and partial-copy semantics tested. |
| **PR 7** | `feat(syscall): define stable UAPI types and error conventions` | Define fixed-width UAPI headers, flag conventions, structure versioning rules, negative `errno` mapping, and reserved number policy. | ABI layout tests assert sizes, alignment, offsets; unknown flags and mismatched struct sizes fail deterministically. |
| **PR 8** | `feat(syscall): integrate process lifecycle calls with scheduler` | Implement and connect `exit`, `getpid`, `getppid`, `fork`/`clone`, `wait4`, and `execve` to scheduler and task lifecycle. | QEMU test spawns user process, executes, exits, reaps, and verifies status/cleanup. |
| **PR 9** | `feat(syscall): integrate file descriptor and VFS operations` | Implement and verify `openat`, `read`, `write`, `close`, `lseek`, `fstat`, FD table inheritance, and VFS permission checks. | Ring 3 tests cover valid files, invalid FDs, permission failures, short I/O, and resource cleanup. |
| **PR 10** | `feat(syscall): add pollable event and pipe interfaces` | Integrate `pipe2`, `eventfd`, `epoll_create1`/`epoll_wait` as FD-backed objects with readiness polling and wakeup guarantees. | Stress tests verify data flow, readiness, EOF, close, broken pipe, and wakeup reliability. |
| **PR 11** | `feat(syscall): add signals and interruption semantics` | Connect `rt_sigaction`, `rt_sigprocmask`, `kill`, `tgkill`, signal frame delivery, and `EINTR` syscall restart rules (`SA_RESTART`). | Permission checks, invalid signals, interrupted reads/waits, handler execution, and restart rules tested. |
| **PR 12** | `feat(syscall): enforce namespace and resource-control boundaries` | Connect PID, mount, and network namespace boundary checks and cgroups/rlimits (`setrlimit`/`getrlimit`) to syscall dispatch. | Process cannot access/modify isolated namespace objects without credentials; resource limits enforced in kernel dispatch. |
| **PR 13** | `security(syscall): enforce seccomp and capability checks at dispatch` | Integrate seccomp-BPF filters, Landlock, OpenBSD pledge/unveil, and POSIX capabilities at syscall dispatch entry. | Policy denial occurs before side effects; rule bypass via alternate syscall numbers tested; status marked Integrated. |
| **PR 14** | `feat(libc): bind syscall wrappers to stable kernel ABI` | Align inline-assembly libc shims with actual kernel dispatch, register ABI, error mapping, and architecture conventions. | Wrapper tests verify register usage, error return translation (`-errno`), and ABI constants. |
| **PR 15** | `docs(syscall): add syscall reference and manual pages` | Publish complete man-page reference for all implemented syscalls detailing arguments, return values, errors, permissions, and blocking behavior. | Every public syscall has complete documentation matching implementation and tests. |
| **PR 16** | `test(syscall): add architecture and userspace conformance matrix` | Implement cross-architecture ABI/layout conformance test matrix across x86_64, AArch64, and RISC-V targets. | Claimed targets execute applicable test suite; unsupported ABI combinations explicitly documented. |
| **PR 17** | `ci(syscall): gate ABI changes and protect syscall ownership` | Configure code ownership for UAPI headers/tables, required CI checks, branch protection, and ABI review checklist. | ABI changes require review from syscall owners, passing selftests, and branch protection approval. |

---

## 3. Periodic Maintenance Cadence

To prevent ABI drift, regressions, and security vulnerabilities, the system-call subsystem follows recurring maintenance cadences:

- **Every PR:** Execute syscall table consistency checks, UAPI layout tests, user-memory fault harness, and unit test suites.
- **Weekly:** Triage failing syscall tests, ABI questions, and regressions; maintain zero unassigned ABI issues.
- **Weekly:** Review fuzzing results against user-copy and argument decoding primitives; expand regression corpus.
- **Monthly:** Audit dispatch table, libc wrappers, UAPI headers, and documentation for 100% synchronization.
- **Monthly:** Security audit of user-pointer handling, integer overflow guards, capability checks, and object lifetimes.
- **Monthly:** Execute QEMU Ring 3 smoke test suite covering entry/return, `write`, `exit`, invalid calls, and memory faults.
- **Quarterly:** Review public ABI compatibility guarantees, deprecation proposals, and POSIX/Linux/BSD alignment.
- **Quarterly:** Assess cross-architecture portability (x86_64, AArch64, RISC-V 64) for calling conventions and structure packing.
- **Each Release Candidate:** Certify userspace ABI claims with full QEMU/bare-metal test evidence logs and compatibility statements.
- **Post-Incident:** Add mandatory regression tests for any discovered syscall bug or security vulnerability.

---

## 4. Component Implementation Rules

- **Architecture Entry Code:** Must document entry instruction (`SYSCALL`, `SVC`, `ECALL`), argument/return registers, clobbers, kernel stack switching, and interrupt masking.
- **Dispatch Table:** Single source of truth defining bounded table size, permanent number assignments, `ENOSYS` behavior, and duplicate detection.
- **Argument Decoding:** Arguments must be decoded into typed kernel values before business logic. Unknown flags and unrecognized enum values must be deterministically rejected.
- **User-Memory Layer:** Safe copy-in/copy-out primitives with length caps and page fault recovery must be used for all user pointers.
- **VFS & File Descriptors:** File permissions must be validated against effective calling credentials. Descriptors must enforce thread-safe reference counting and cleanup on error.
- **Process & Thread Operations:** Target process references must be validated and authorized; user-supplied PIDs must be verified before object lookup.
- **Signals & Blocking Operations:** Define `EINTR` interruption, `SA_RESTART` restartability, and timeout handling consistently across all blocking calls.
- **Security & Sandboxing:** Seccomp, Landlock, Pledge/Unveil, and Capabilities must enforce access rules at syscall entry before any kernel state change or side effect occurs.
- **Libc & Shims:** Wrappers are part of the ABI contract; they must correctly translate negative kernel error return codes into standard `errno` variables.
- **Observability:** Syscall tracing must record syscall name, PID, and return status without leaking secrets or raw memory pointers.
- **Cross-Distro Inspiration:** System call interfaces must explicitly document whether their behavior is native SigmaOS, POSIX-compliant, or a Linux/BSD compatibility wrapper.

---

## 5. PR Review Gate Checklist

All PRs introducing or modifying system calls must include the following completed template:

```text
## Summary
What user-visible capability or correctness issue does this PR address?

## ABI proposal
Name, number/namespace, architecture(s), arguments, return values, errno/status,
blocking behavior, permissions, and compatibility considerations.

## Why a syscall?
Explain why an existing syscall, FD operation, VFS interface, or typed API is
not appropriate.

## ABI stability
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

## Documentation and status
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

## 6. Subsystem Rollout and Exit Criteria

1. **Integrated Status Criteria:**
   - A real user process executes from Ring 3 through x86_64 `SYSCALL` into kernel dispatch and receives a valid return via `SYSRET` on a QEMU boot target.
   - Fault-safe memory copy primitives handle invalid pointers without kernel panic.
2. **Supported Status Criteria:**
   - All 17 PRs are completed with 100% passing test coverage in CI.
   - The UAPI layout tests assert zero structure padding/alignment drift across x86_64, AArch64, and RISC-V 64.
   - Complete manual pages and API documentation exist for every public system call.
