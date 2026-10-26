# SigmaOS System-Call Development and Maintenance Plan

## Executive Summary & Existing Gaps

A comprehensive audit of SigmaOS's public inventory, `FEATURE_STATUS.toml`, and architectural specifications indicates that while 64 syscall numbers are defined with libc shims, the overall syscall ABI status remains **partial** and **compile-only**. Kernel-side dispatch, x86_64 Ring 3 entry state transitions, and safe user-memory validation constitute critical next milestones.

This document establishes the official 17-PR sequence roadmap, 10 mandatory ABI policies, suggested ABI priority, periodic maintenance cadences, component implementation rules, review and merge gates, and rollout/exit criteria for the SigmaOS System-Call Subsystem.

---

## Mandatory ABI Policies

1. **Stable ABI:** Once published to userspace, syscall numbers, argument layouts, return conventions, error behavior, and semantics are permanently stable. New interfaces must be added rather than modifying published numbers.
2. **Avoid Unnecessary Syscalls:** First consider existing syscalls, file-descriptor APIs, filesystem interfaces, or typed mechanisms. Avoid opaque multiplexer interfaces except for close analogues or simple flags.
3. **Extensible Arguments:** Prefer a `flags` parameter for simple extensions and a size-versioned structure for complex argument sets. Reject unknown flags and incompatible nonzero extension fields deterministically.
4. **Validate User Memory Safely:** Never directly dereference a userspace pointer. Validate address ranges and copy data through fault-safe user-memory helpers (`copy_from_user`, `copy_to_user`). Unmapped or invalid memory must return `EFAULT` rather than panic or corrupt kernel state.
5. **Authorization in the Kernel:** Enforce object ownership, permissions, capabilities, namespace boundaries, and resource limits at syscall entry or shared kernel helpers—never rely solely on libc or userland CLI tools.
6. **Portable ABI Types:** Use fixed-width UAPI types (`u32`, `u64`, `i32`, `i64`); define alignment, padding, endianness, and 32/64-bit compatibility explicitly. Avoid native compiler-dependent types (`usize`, `long`) in UAPI headers.
7. **File Descriptors for Kernel Objects:** Prefer FD-backed handles for pollable objects; explicitly specify close-on-exec (`O_CLOEXEC` / `FD_CLOEXEC`) and polling semantics.
8. **No Kernel-to-Syscall Calls:** Kernel subsystems must invoke shared internal kernel functions; syscall entry points are user/kernel boundary wrappers only.
9. **Status Means Tested Behavior:** "Integrated" requires a real userspace-to-kernel path on a documented target. "Supported" additionally requires repeatable test coverage and documented compatibility limits.
10. **Security & Maintenance:** Follow NIST SP 800-218 (SSDF) lifecycle practices: secure development, software protection, well-secured releases, and vulnerability responses.

---

## 17-PR Sequence Roadmap

| PR | Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(syscall): define ABI policy and component status` | Audit `kernel/syscalls/`, syscall ABI, entry code, libc shims, and `FEATURE_STATUS.toml`. Document syscall ownership, stability, supported architectures, errors, and status vocabulary. | Every listed syscall maps to implementation, dispatch entry, wrapper, and tests—or is marked not integrated; Linux-compatibility claims are distinguished from POSIX interfaces. |
| 2 | `test(syscall): add ABI inventory consistency checker` | Add automated checker comparing syscall numbers, names, dispatch entries, wrappers, and generated/documented tables. | CI rejects duplicate/reused numbers, missing dispatch entries, undocumented entries, or wrapper/table drift. |
| 3 | `test(syscall): add user-memory access and error-path test harness` | Establish safe test infrastructure for invalid pointers, unmapped ranges, read-only pages, overflowed lengths, cross-page buffers, and fault injection. | Invalid user memory cannot panic or corrupt kernel state; results use documented `EFAULT` errors; unit/QEMU tests pass. |
| 4 | `feat(syscall): implement kernel dispatch and unknown-call handling` | Connect architecture entry path to bounded syscall dispatch table; define unknown number (`ENOSYS`), disabled call, and invalid argument behavior. | Ring-3 test invokes known call with correct result; unknown call returns `ENOSYS` cleanly; no kernel code bypasses wrapper. |
| 5 | `feat(syscall): validate x86_64 entry state and return path` | Verify register mapping (`rax`, `rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9`), privilege transition (`SYSCALL`/`SYSRET`), stack switching, saved state, and return path. | QEMU test enters from Ring 3, calls repeatedly, and returns to userspace without register/stack corruption. |
| 6 | `feat(syscall): add safe user-copy primitives` | Implement central `copy_from_user`, `copy_to_user`, and range-validation APIs tied to process address spaces. | All handlers use safe helpers; copy faults return `EFAULT`; partial copy and integer overflow semantics are tested. |
| 7 | `feat(syscall): define stable UAPI types and error conventions` | Define UAPI headers, fixed-width types, flag conventions, structure versioning, errno mapping, and reserved-number policy. | ABI layout tests assert sizes, alignment, offsets, and values; unknown flags and mismatched structures behave predictably. |
| 8 | `feat(syscall): integrate process lifecycle calls with scheduler` | Connect `exit`, `getpid`, `getppid`, `fork`/`clone`, `wait4`, `execve` dependencies required to launch a real user process. | QEMU test launches user task, exits, reaps it, and verifies status/cleanup without orphan leaks or race conditions. |
| 9 | `feat(syscall): integrate file descriptor and VFS operations` | Add/verify `openat`, `read`, `write`, `close`, `lseek`, `fstat`, FD table inheritance, and VFS permission checks. | Ring-3 tests cover valid files, invalid descriptors, permission failures, short I/O, close/reuse, and cleanup. |
| 10 | `feat(syscall): add pollable event and pipe interfaces` | Integrate `pipe2`, `eventfd`, `epoll_create1`/`epoll_wait` as FD-backed objects with readiness polling. | Tests cover data flow, readiness, EOF, close, broken pipe, blocked operations, and wakeup reliability. |
| 11 | `feat(syscall): add signals and interruption semantics` | Define signal delivery, masks, interruption/restart rules (`SA_RESTART`), `kill`, `tgkill`. | Tests cover permission checks, invalid signals, interrupted reads/waits, handler return, and documented restart behavior. |
| 12 | `feat(syscall): enforce namespace and resource-control boundaries` | Connect PID, mount, net namespace boundary checks and cgroups/rlimits (`setrlimit`/`getrlimit`). | Process cannot inspect or modify another namespace's objects without authorization; limits enforced in kernel paths. |
| 13 | `security(syscall): enforce seccomp and capability checks at dispatch` | Integrate seccomp-BPF filters, Landlock, OpenBSD pledge/unveil, and POSIX capabilities at dispatch. | Tests prove policy denial occurs before side effects; rule bypass through alternate syscall paths is tested. |
| 14 | `feat(libc): bind syscall wrappers to stable kernel ABI` | Align current libc shim and inline-assembly wrappers with actual dispatch, register ABI, error mapping, and supported architecture. | Wrapper tests verify register/stack conventions, return/error translation (`-errno`), and ABI constants. |
| 15 | `docs(syscall): add syscall reference and manual pages` | Document each implemented call, arguments, errors, permissions, blocking behavior, ABI stability, and examples. | Every public syscall has complete documentation matching implementation and unit tests. |
| 16 | `test(syscall): add architecture and userspace conformance matrix` | Add native and cross-architecture ABI/layout tests across x86_64, AArch64, and RISC-V 64 targets. | Each claimed architecture executes applicable test suite; unsupported ABI combinations are explicitly documented. |
| 17 | `ci(syscall): gate ABI changes and protect syscall ownership` | Add code ownership for UAPI headers/tables, required CI checks, branch protection, and ABI review checklist. | Every ABI change receives ABI/security review; selftests pass; protected branches enforce review/status checks. |

---

## Suggested ABI Implementation Priority

1. `write` / `exit` for Ring 3 smoke testing.
2. Safe user-memory access (`copy_from_user` / `copy_to_user`) and `EFAULT` handling.
3. Process launch, exit, and reaping (`getpid`, `fork`/`clone`, `exit`, `wait4`).
4. File descriptors and basic VFS I/O (`openat`, `read`, `write`, `close`, `lseek`, `fstat`).
5. `execve`, `pipe2`, and pollable events (`epoll`, `eventfd`).
6. Signals (`rt_sigaction`, `rt_sigprocmask`, `kill`) and resource controls (`getrlimit`, `setrlimit`).
7. Namespaces (`unshare`, `setns`) and Linux/BSD compatibility extensions.

---

## Periodic Maintenance Cadence

| Cadence | Recurring Task | Required Output |
|---|---|---|
| Every PR | `ci(syscall): run ABI and boundary checks` | Compile/tests, syscall-table consistency, ABI layout tests, and user-memory safety checks. |
| Weekly | `maint(syscall): triage failures and unresolved ABI issues` | Regression triage, open interface questions, owner assignment; no number reassignments. |
| Weekly | `test(syscall): review fuzz findings and boundary corpus` | Crash minimization, fuzz findings converted into regression tests. |
| Monthly | `audit(syscall): reconcile dispatch, wrappers, docs, and status` | Inventory audit across numbers, dispatch, libc, manuals, and feature matrices. |
| Monthly | `security(syscall): review user-copy and authorization paths` | Code review of changed handlers for pointer access, integer overflow, and permissions. |
| Monthly | `test(syscall): run QEMU ring-3 smoke suite` | Boot log and test results for Ring 3 entry/return, `write`, `exit`, and `EFAULT`. |
| Quarterly | `docs(syscall): review ABI compatibility and deprecation policy` | Audit public ABI guarantees, POSIX/Linux/BSD compatibility statements. |
| Quarterly | `test(syscall): assess architecture portability` | Calling convention, structure layout, and 32/64-bit handling audit across targets. |
| Release Candidate | `release(syscall): certify userspace ABI claims` | Tested kernel build, ABI headers, libc build, QEMU target report, selftest logs. |
| Post-Incident | `fix(syscall): add regression coverage for <issue>` | Root cause analysis, regression test, security/ABI impact assessment, backport decision. |

---

## Rules for Syscall-Related Components

- **Architecture Entry Code:** Document entry instruction (`SYSCALL`/`SYSRET`, `svc`, `ecall`), register mappings, return registers, clobbers, stack switching, and interrupt masking rules per architecture.
- **Dispatch Table:** Maintain a single canonical source of truth for syscall dispatch numbers. Reserve numbers permanently once published; handle disabled/unassigned calls cleanly (`ENOSYS`).
- **Argument Decoding:** Decode arguments into strongly-typed kernel structures before executing business logic. Validate flags and versioned structure sizes strictly.
- **User-Memory Layer:** Use distinct `copy_from_user` and `copy_to_user` primitives. Cap maximum lengths, handle partial copies, and return `EFAULT` on page faults.
- **VFS and File Descriptors:** Perform permission checks against calling process credentials. Enforce close-on-exec and proper object cleanup on all failure paths.
- **Process and Thread Calls:** Validate target process ownership and credentials. Prevent PID race conditions during process termination and reuse.
- **Signals and Interruption:** Explicitly define signal masking, `EINTR` interruption, `SA_RESTART` restartability, and blocking call cancellation.
- **Security Framework:** Enforce seccomp-BPF filters, POSIX capabilities, Landlock, and pledge/unveil policies directly in the dispatch path before executing syscall logic.
- **Libc Shim Layer:** Align inline assembly wrappers with kernel register conventions and negative `errno` return mappings.
- **Observability:** Provide structured tracing for syscall numbers, parameters, calling processes, and return codes without leaking user secrets or sensitive memory contents.
- **Cross-Distro Parity:** Clearly categorize native SigmaOS ABI calls, POSIX libc wrappers, and emulation layers for foreign OS compatibility.

---

## PR Review & Merge Gates

All syscall-related pull requests must include the standard PR review template:

- [x] ABI review completed before implementation is locked
- [x] No direct userspace-pointer dereference
- [x] Unknown flags/versions rejected deterministically
- [x] Stable number and UAPI layout checked
- [x] Selftest and documentation included
- [x] No kernel code calls syscall entry points internally
- [x] Required CI checks pass; review comments resolved

---

## Subsystem Rollout & Exit Criteria

- **Integrated:** Achieved when a user process successfully invokes kernel dispatch from Ring 3 on a documented boot target, covering both valid execution and handled error paths (`ENOSYS`, `EFAULT`).
- **Supported:** Achieved when the syscall inventory, libc wrappers, entry wiring, memory safety primitives, security controls, automated test suites, and public documentation agree and pass repeatably in CI/QEMU across all supported targets.
