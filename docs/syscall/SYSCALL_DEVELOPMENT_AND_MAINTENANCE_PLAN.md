# SigmaOS System-Call Development and Maintenance Plan

## Summary

This document establishes the authoritative SigmaOS System-Call Development and
Maintenance Plan. The syscall subsystem defines the core boundary between untrusted
user-mode applications (Ring 3) and the privileged operating system kernel (Ring 0).

The approach is deliberately evidence-driven: first prove a minimal syscall from a real
user process reaches the kernel and returns correctly, then stabilize the ABI and add
process, file, memory, and IPC interfaces in dependency order.

## Existing Gaps and Proposed Policy

The repository contains syscall-related interfaces, libc wrappers, and modules for
`eventfd`, `ioctl`, signals, `pipe`, process groups, resource limits, namespaces, and
file descriptors. However, their presence alone is not proof that they are safely
dispatched from user mode, enforce permissions, or operate end to end.

### Mandatory ABI Policies

1. **Stable ABI:** Once published to userspace, syscall number, argument layout,
   return convention, error behavior, and semantics are stable. Add interfaces rather
   than silently changing or reusing published numbers.
2. **Avoid unnecessary syscalls:** First consider an existing syscall, file-descriptor
   API, filesystem interface, or other typed mechanism. Avoid opaque multiplexer
   interfaces except for close analogues or simple flags.
3. **Extensible arguments:** Prefer a flags parameter for simple extensions and a
   size-versioned structure for complex argument sets. Reject unknown flags and
   incompatible nonzero extension fields rather than guessing.
4. **Validate user memory safely:** Never directly dereference a userspace pointer.
   Validate address ranges and copy data through fault-safe user-memory helpers;
   invalid/unmapped memory must return a defined error (`EFAULT`) rather than crash
   or panic the kernel.
5. **Authorization in the kernel:** Enforce object ownership, permissions, capabilities,
   namespace boundaries, and resource limits at syscall entry or the shared kernel
   helper—not only in libc or command-line tools.
6. **Portable ABI types:** Use fixed-width UAPI types; define alignment, padding,
   endianness where relevant, and 32/64-bit compatibility behavior explicitly. Avoid
   ABI-visible native pointers, `usize`, `long`, or compiler-dependent layouts unless
   the ABI intentionally specifies them.
7. **File descriptors for kernel objects:** Prefer FD-backed handles for pollable objects;
   document close-on-exec and polling semantics.
8. **No kernel-to-syscall calls:** Kernel subsystems call shared internal helpers;
   syscall entry points are user/kernel boundary wrappers, not internal kernel APIs.
9. **Status means tested behavior:** "Integrated" requires a real userspace-to-kernel
   path on a documented boot target. "Supported" additionally requires repeatable test
   coverage and documented compatibility limits.
10. **Security and maintenance:** Follow NIST SSDF's lifecycle approach: prepare secure
    development, protect software and build inputs, produce well-secured releases, and
    respond to vulnerabilities.

## Proposed PR Sequence

| PR | Proposed title | Scope | Acceptance criteria |
|---|---|---|---|
| 1 | `docs(syscall): define ABI policy and component status` | Audit `kernel/syscalls/`, syscall ABI, architecture entry code, libc shims, `FEATURE_STATUS.toml`, and relevant wiki pages. Document syscall number ownership, ABI stability, supported architectures, errors, status vocabulary, and current gaps. | Every listed syscall maps to implementation, dispatch entry, wrapper, and tests—or is marked not integrated; Linux-compatibility claims are distinguished from POSIX interfaces. |
| 2 | `test(syscall): add ABI inventory consistency checker` | Add a checker that compares syscall numbers, names, dispatch entries, wrappers, and generated/documented tables. Prevent accidental duplicate numbers and stale entries. | CI rejects duplicate/reused numbers, missing dispatch mappings, undocumented entries, or wrapper/table drift; intentional reserved numbers are explicitly declared. |
| 3 | `test(syscall): add user-memory access and error-path test harness` | Establish safe test infrastructure for invalid pointers, unmapped ranges, read-only pages, overflowed lengths, cross-page buffers, and output-copy faults. | Invalid user memory cannot panic or corrupt kernel state; results use documented errors; tests run in hosted/unit mode where possible and QEMU once ring-3 execution is available. |
| 4 | `feat(syscall): implement kernel dispatch and unknown-call handling` | Connect the architecture entry path to a bounded syscall dispatch table; define unknown number, disabled call, and invalid argument behavior. | A ring-3 test invokes a known syscall and gets the expected result; unknown number returns a documented “not implemented” error; no kernel function bypasses the syscall wrapper. |
| 5 | `feat(syscall): validate x86_64 entry state and return path` | Verify register mapping, user/kernel privilege transition, stack switching, saved state, return path, and syscall masking rules for the declared x86_64 target. | QEMU test enters from Ring 3, makes repeated calls, and returns to userspace without register/stack corruption; fault and signal/interrupt interactions are tested or explicitly deferred. |
| 6 | `feat(syscall): add safe user-copy primitives` | Implement central copy-in/copy-out and range-validation APIs tied to process address spaces; audit current handlers and wrappers for direct pointer access. | All current syscall handlers use safe helpers; copy faults return defined errors; integer overflow and partial-copy semantics are documented and tested. |
| 7 | `feat(syscall): define stable UAPI types and error conventions` | Define UAPI headers, fixed-width types, flag conventions, structure versioning, errno/status mapping, and reserved-number policy. | ABI layout tests assert sizes, alignment, offsets, and values; unknown flags and undersized/oversized structures behave as documented; generated docs match headers. |
| 8 | `feat(syscall): integrate process lifecycle calls with scheduler` | Prioritize exit, process/thread identity, wait/reap, and spawn/exec dependencies required to launch a real user process. Keep each syscall in a separate sub-PR if independent. | QEMU test launches a user task, exits, reaps it, and verifies status/cleanup; ownership and permission failures are tested; scheduler integration is demonstrated, not modeled. |
| 9 | `feat(syscall): integrate file descriptor and VFS operations` | Add/verify open/read/write/close/seek/stat equivalents, FD table semantics, descriptor inheritance, and VFS permission checks. | Ring-3 tests cover valid files, invalid descriptors, permission failures, short I/O, close/reuse, and cleanup; user buffers always use safe-copy APIs. |
| 10 | `feat(syscall): add pollable event and pipe interfaces` | Integrate `pipe`, event notification, and poll/wait mechanisms as FD-backed objects where appropriate; define readiness and EOF behavior. | Tests cover data flow, readiness, close, broken pipe, blocked operations, wakeups, and resource limits; no missed wakeups in stress tests. |
| 11 | `feat(syscall): add signals and interruption semantics` | Define signal delivery, masks, interruption/restart rules, and interactions with blocking calls. | Tests cover permission checks, invalid signal numbers, interrupted reads/waits, handler return, and documented restart behavior. |
| 12 | `feat(syscall): enforce namespace and resource-control boundaries` | Connect existing namespace, cgroup/resource, and limits modules to dispatch and target objects. | A process cannot inspect or modify another namespace’s objects without authorization; limits are enforced in kernel paths; tests demonstrate denial and allowed delegation. |
| 13 | `security(syscall): enforce seccomp and capability checks at dispatch` | Integrate seccomp/pledge/Landlock/capability policy only where the underlying policy model is sufficiently specified; apply checks consistently. | Tests prove policy denial occurs before side effects; rule bypass through alternate syscall paths is tested; feature status says “Integrated” only when enforced in dispatch/VFS/network paths. |
| 14 | `feat(libc): bind syscall wrappers to stable kernel ABI` | Align the current libc shim and inline-assembly wrappers with actual dispatch, register ABI, error mapping, and supported architecture. | Wrapper tests verify register/stack conventions, return/error translation, and ABI constants; unsupported calls fail predictably rather than appearing implemented. |
| 15 | `docs(syscall): add syscall reference and manual pages` | Document each implemented call, arguments, errors, permissions, blocking behavior, ABI stability, and examples; add man-page style references. | Every public syscall has documentation matching tests and implementation; no placeholder syscall is documented as fully supported. |
| 16 | `test(syscall): add architecture and userspace conformance matrix` | Add native and cross-architecture ABI/layout tests for supported targets; use selftests and selected Linux/BSD behavior references without assuming binary compatibility. | Each claimed architecture executes applicable tests; unsupported ABI combinations are documented; test results are stored per target. |
| 17 | `ci(syscall): gate ABI changes and protect syscall ownership` | Add code ownership for syscall tables/UAPI/entry paths, required CI, branch protection, and a syscall-change review checklist. | Every ABI change gets an ABI/security review, selftests pass, and protected branches require review/status checks. |

### Suggested ABI Priority

1. `write`/`exit` for a ring-3 smoke test.
2. Safe user memory access and error handling.
3. Process launch/exit/reap.
4. File descriptors and basic VFS I/O.
5. `exec`/`wait`, pipes, and pollable events.
6. Signals and resource controls.
7. Optional namespaces and Linux/BSD compatibility extensions.

## Periodic Maintenance PR Cadence

| Cadence | Proposed recurring PR/task | Required output |
|---|---|---|
| Every PR | `ci(syscall): run ABI and boundary checks for touched code` | Compile/tests, syscall-table consistency, ABI layout tests, and relevant user-memory/security tests; skipped checks explained. |
| Weekly | `maint(syscall): triage failures and unresolved ABI issues` | New regressions, failing tests, open interface questions, and owner/priority assigned; no number changes without an ABI review. |
| Weekly | `test(syscall): review fuzz findings and boundary corpus` | Crashes minimized; fixed findings become regression tests; target limits and fuzz commands recorded. |
| Monthly | `audit(syscall): reconcile dispatch, wrappers, docs, and status` | Inventory audit across syscall numbers, dispatch, libc, manuals, feature matrix, and component pages. |
| Monthly | `security(syscall): review user-copy and authorization paths` | Review every changed handler for pointer access, integer overflow, permissions, object lifetime, and partial side effects. |
| Monthly | `test(syscall): run QEMU ring-3 smoke suite` | Boot log and tests for entry/return, `write`, `exit`, invalid syscall, and invalid buffer behavior on the primary target. |
| Quarterly | `docs(syscall): review ABI compatibility and deprecation policy` | Review public ABI promises, Linux/BSD/POSIX compatibility statements, and migration/deprecation proposals. |
| Quarterly | `test(syscall): assess architecture portability` | Audit calling conventions, structure layout, syscall numbering, and 32/64-bit handling for every architecture claimed as supported. |
| Each release candidate | `release(syscall): certify userspace ABI claims` | Tested kernel revision, ABI headers, libc build, QEMU/hardware target, selftest results, known gaps, and compatibility statement. |
| After a syscall incident | `fix(syscall): add regression coverage for <issue>` | Root cause, affected versions, regression test, security/ABI impact, and backport/release decision. |

## Rules for Syscall-Related Components

- **Architecture entry code:** Each architecture documents syscall instruction/trap mechanism,
  argument registers, return register, clobbers, stack transition, interrupt masking, and compatibility ABI.
- **Dispatch table:** Have one source of truth or generated tables; reserve numbers permanently once published;
  define feature-disabled and unknown-call behavior; detect duplicates in CI.
- **Argument decoding:** Decode into typed kernel values before business logic. Reject unknown flags and invalid
  enum values; guard size arithmetic and structure versioning.
- **User-memory layer:** Separate safe copy-in, copy-out, and string-copy helpers; cap lengths; define behavior
  for faults and partial copies. Do not rely on an address-range check alone as proof the memory remains accessible.
- **VFS and file descriptors:** Perform permission checks against the effective calling context; define descriptor
  lifetime, inheritance, close-on-exec, poll readiness, and cleanup on every error path.
- **Process and thread calls:** Specify target ownership and authorization; define races with exit/reuse; never trust
  user-supplied process IDs as object references without validation.
- **Signals and blocking:** Define interruption, restart, timeout units, and cancellation behavior consistently; test
  actual blocked tasks rather than only state-machine models.
- **Security framework:** A seccomp, capability, pledge, unveil, Capsicum, or MAC module is only a policy model until
  syscall dispatch or the relevant subsystem enforces it.
- **Libc and compatibility layer:** Treat wrappers as part of the ABI; test their register/stack calling convention
  and error translation against the kernel, and keep architecture-specific details out of generic code where possible.
- **Diagnostics and observability:** Provide trace/logging that can identify syscall name/number, process, and error
  without leaking pointer contents, secrets, or unnecessary user data. Keep tracing off or access-controlled by default.
- **Cross-distro inspiration:** Linux, FreeBSD, and OpenBSD interfaces differ; reuse concepts thoughtfully, but document
  which ABI is native, which is POSIX-facing through libc, and which is an emulation layer.

## PR Review and Merge Gates

Use this template for any syscall PR:

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

## Rollout and Exit Criteria

- **Proposed:** ABI proposal, alternative analysis, and review checklist defined.
- **Prototype:** Kernel helper/dispatch, architecture entry, or userspace harness modeled and covered by unit tests.
- **Integrated:** A user process invokes the real kernel dispatch path on a documented target, including successful
  and failing calls.
- **Supported:** The ABI inventory, libc wrappers, architecture wiring, user-memory safety, permission tests,
  and public documentation agree and pass repeatably.
