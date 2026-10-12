# SigmaOS process-manager plan

SigmaOS’s published inventory says `src/userspace/init.rs` contains a PID 1 service manager with dependency ordering and service-state tracking, but its stated next milestone is **process spawning via `execve` and dependency enforcement**. The same inventory lists process groups, signals, cgroups, resource limits, and process inspection components, but those entries do not establish that a complete process-management path runs end to end. The README likewise cautions that modules and unit tests do not prove a bootable, usable OS. ([SigmaOS Repository](https://github.com/AaryanSinghChauhan09/SigmaOS))

The plan below orders the work around that gap: first prove process creation, execution, waiting, and cleanup in a booted target; then build reliable service supervision and management tools. Linux, POSIX, and FreeBSD are references for semantics and operational practices—not claims that SigmaOS has their ABI compatibility. ([Linux Kernel Syscall Guide](https://docs.kernel.org/process/adding-syscalls.html))

## Design and safety rules

- **Separate kernel lifecycle from service policy.** The kernel owns process/thread creation, scheduling, exit, wait, signal delivery, and resource accounting. PID 1 or a service manager owns service definitions, dependency policy, restart decisions, and shutdown orchestration.
- **Model lifecycle states explicitly.** Define valid transitions for created, starting, running, blocked, stopping, exited, reaped, and failed processes/services. Reject invalid transitions and test races such as exit during stop or restart.
- **Reap every child.** Preserve child exit status until it is collected, notify the parent, and ensure PID 1 adopts/reaps orphaned children according to documented policy. POSIX `wait`/`waitpid` return child status and consume it; test normal exit, signal termination, stop/continue, and interruption. ([POSIX wait Specification](https://pubs.opengroup.org/onlinepubs/9799919799/functions/wait.html))
- **Avoid PID-reuse races in management.** Use a stable kernel process reference or descriptor for long-lived supervisor handles; if management operates on numeric PIDs, validate identity at the time of action. Linux pidfds are one reference design for stable references to processes. ([Linux pidfd_open(2)](https://www.man7.org/linux/man-pages/man2/pidfd_open.2.html))
- **Bound service recovery.** Restart policies need delays and start-rate limits; otherwise a crashing service can consume resources or block boot in a restart loop. Linux service managers document restart policies, start timeouts, stop timeouts, and rate limits as separate controls. ([systemd.service Reference](https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html))
- **Keep service dependencies declarative and cycle-checked.** A service must not start until required dependencies satisfy their readiness condition; dependency cycles should fail validation with a clear diagnostic. FreeBSD `rcorder` and service dependencies provide a useful comparison. ([FreeBSD rc.d Documentation](https://docs-archive.freebsd.org/doc/11.0-RELEASE/usr/local/share/doc/freebsd/en/books/handbook/configtuning-rcd.html))
- **Make resource controls enforceable, not decorative.** The inventory’s cgroup-like controllers should not be advertised as effective limits until process creation, scheduler, and memory paths enforce them. Linux cgroup v2 organizes processes hierarchically and distributes resources through controllers. ([SigmaOS Feature Inventory](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FEATURE_STATUS.toml))
- **Use least privilege.** Services should run under distinct identities/permissions when those mechanisms are real; expose only required files, IPC, devices, and network access.
- **Make stop and shutdown graceful but bounded.** Attempt the configured graceful termination, wait for a deadline, then use a documented forced-stop policy; always reap and record the outcome.
- **Test at multiple levels.** Unit tests validate state machines; integration tests exercise actual user processes; QEMU boot tests validate PID 1 and service behavior. Linux separates in-kernel unit tests from userspace selftests that exercise live interfaces. ([Linux Testing Overview](https://docs.kernel.org/dev-tools/testing-overview.html))

## Proposed PR sequence

| PR | Proposed title | Scope | Acceptance criteria |
|---|---|---|---|
| 1 | `docs(process): audit process lifecycle and manager status` | Audit scheduler, context switching, Ring 3, ELF loading, syscall dispatch, signals, `wait`, init, process groups, cgroups, procfs, and libc wrappers. Update the feature matrix with implementation and runtime evidence. | Every capability is marked Proposed, Prototype, Integrated, or Supported; process-model tests are distinguished from real user-mode execution. |
| 2 | `docs(process): define process and service lifecycle contracts` | Document process/thread identity, ownership, state transitions, exit status, wait semantics, parent/orphan policy, service readiness, dependencies, restart, and shutdown. | State-transition diagram and API contract are reviewed; unresolved semantics are tracked before implementation; Linux/BSD-inspired behavior is labeled rather than assumed. |
| 3 | `test(process): add deterministic lifecycle and race harness` | Add test fixtures for create/exit/wait, parent exit, simultaneous child exits, PID reuse, signals during waits, process-group transitions, and resource cleanup. | Tests are deterministic and bounded; each fixed race becomes a regression test; model tests are clearly identified as non-runtime evidence. |
| 4 | `feat(process): complete user-mode transition and minimal process launch` | Connect architecture user-mode transition, address-space setup, ELF loading, initial stack, argv/envp, and a minimal user program. Start with the primary QEMU architecture. | Fresh QEMU boot launches a user process, executes a known program, and records exit status; invalid executable and setup failure return cleanly without corrupting the kernel. |
| 5 | `feat(process): implement spawn and exec interface` | Implement a documented process creation/exec path with structured arguments, environment, inherited descriptors, working directory, permissions, and error reporting. | Real user program can launch another program; arguments and environment are preserved exactly; setup errors are reported to the parent; no shell-string execution is used internally. |
| 6 | `feat(process): implement wait and child status collection` | Implement `wait`/`waitpid` subset, status encoding, nonblocking wait, and supported stopped/continued status. | Tests cover normal exit, signal exit, no child, invalid options, `EINTR`, `WNOHANG`, and one-time status consumption; behavior matches the documented subset of POSIX. ([POSIX wait Specification](https://pubs.opengroup.org/onlinepubs/9799919799/functions/wait.html)) |
| 7 | `feat(process): add orphan adoption and PID 1 child reaping` | Define reparenting/adoption when a parent exits and implement PID 1’s wait loop. | Parent-exit tests prove descendants are adopted and eventually reaped; PID 1 remains responsive while reaping and does not lose exit status for supervised children. |
| 8 | `feat(process): add stable process references for supervision` | Add an internal kernel process handle or FD-backed process reference for service-manager operations; define identity lifetime and permission semantics. | A stale reference cannot target a newly reused PID; exited-process state is observable until collection; access checks are tested. Linux pidfds demonstrate the stable-reference design goal. ([Linux pidfd_open(2)](https://www.man7.org/linux/man-pages/man2/pidfd_open.2.html)) |
| 9 | `feat(init): spawn and supervise real service processes` | Replace service-state-only transitions with real spawn, readiness, exit collection, logs, and service identity. | QEMU starts a minimal dependency-ordered service set; service state reflects actual process state; failure to execute a binary is not reported as successful startup. |
| 10 | `feat(init): validate service dependencies and readiness` | Add declarative service definitions, dependency graph validation, required/optional dependencies, startup ordering, and readiness contract. | Cycles and missing required dependencies fail before launch; dependents wait for readiness, not merely process creation; shutdown order reverses dependency order. FreeBSD’s service system similarly models dependency ordering. ([FreeBSD rc.d Documentation](https://docs-archive.freebsd.org/doc/11.0-RELEASE/usr/local/share/doc/freebsd/en/books/handbook/configtuning-rcd.html)) |
| 11 | `feat(init): add bounded restart and failure policies` | Add no-restart, on-failure, and bounded restart policies with backoff, start limits, failure history, and manual reset. | Crash loops are rate-limited; intentional stop does not trigger restart; backoff and failure state are visible and tested. Linux service units provide restart and rate-limit patterns to adapt. ([systemd.service Reference](https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html)) |
| 12 | `feat(init): implement graceful service stop and system shutdown` | Add stop deadlines, signal delivery, escalation policy, dependency-ordered shutdown, child reaping, and final sync/teardown hooks. | Services receive graceful stop; deadline escalation is tested; shutdown cannot wait forever on an unresponsive service; all children are accounted for. |
| 13 | `feat(process): integrate process groups and terminal job control` | Connect process groups, sessions, controlling terminal, foreground/background jobs, and signal routing once the shell and TTY are integrated. | PTY/QEMU tests cover foreground group signals, stop/continue, shell exit, and terminal restoration; unsupported semantics are documented. |
| 14 | `feat(process): enforce per-service resource limits` | Connect CPU/memory/process limits and accounting to scheduler, allocator, and process creation. Define service-level defaults and delegation. | Limits are enforced in runtime—not only stored in config; fork/spawn fails predictably at limits; parent limits remain authoritative. Linux cgroup v2’s hierarchy and controller model is a reference, not a requirement to clone its filesystem ABI. ([Linux cgroup v2 Guide](https://docs.kernel.org/admin-guide/cgroup-v2.html)) |
| 15 | `feat(process): add process inspection and service-control interfaces` | Provide read-only process/service status APIs, events, logs, and typed start/stop/restart requests for CLI/GUI clients. | Access control limits visibility and actions; events cannot be lost silently; user tools report authoritative live state and support machine-readable output. |
| 16 | `security(process): enforce service isolation and privilege boundaries` | Integrate per-service identity, capability restrictions, filesystem/network access policy, inherited-FD policy, and syscall restrictions where enforcement exists. | Negative tests prove service A cannot access service B’s restricted resources; policy denial occurs in kernel/service boundary; unsupported controls are not shown as active. |
| 17 | `test(process): add booted multi-service and fault-injection suite` | Test missing executable, malformed ELF, dependency failure, crash, blocked stop, repeated restart, parent death, memory/process limit, and shutdown under QEMU. | Tests use actual user processes; failure artifacts identify the stage; resource counts return to expected values; no test depends on arbitrary sleeps for synchronization. |
| 18 | `docs(process): publish process-manager reference and support matrix` | Document process APIs, service definition format, states, permissions, recovery, logs, architecture support, and known limitations. | Every supported claim links to runtime tests; process-manager docs distinguish process supervision from service configuration and resource control. |

### Recommended first milestone

Start with PRs **1–7** and prove one end-to-end path: PID 1 launches a simple user program, the program exits, PID 1 collects its status, and resources are released. Only after this works should the project add restart policies, advanced service dependencies, process-group management, or cgroup-like controls. This sequencing addresses the repo’s explicit init milestone—real process spawning and dependency enforcement—rather than expanding the service-state model alone. ([SigmaOS Feature Inventory](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FEATURE_STATUS.toml))

## Periodic maintenance PR cadence

| Cadence | Proposed recurring PR/task | Required evidence |
|---|---|---|
| Every PR | `ci(process): run lifecycle checks for touched components` | Build, unit/state tests, syscall ABI checks, and relevant QEMU user-process tests; skipped tests explained. |
| Weekly | `maint(process): triage process and service regressions` | Reproducer, affected lifecycle stage, runtime target, owner, and fix or documented deferral. |
| Weekly | `test(process): review crashes, hangs, and resource leaks` | New failures minimized; leak/hang status recorded; resolved issues become regression tests. |
| Monthly | `audit(process): reconcile lifecycle code and capability matrix` | Scheduler, syscall, ELF, signals, wait, init, process groups, procfs, and resource-control claims checked against runtime tests. |
| Monthly | `test(init): run service start-stop and dependency matrix` | Missing dependency, ordering, readiness, crash, intentional stop, restart, and shutdown tests on the supported QEMU target. |
| Monthly | `security(process): review privilege and identity operations` | Review process targeting, stable handles/PID reuse, credentials, executable loading, inherited descriptors, and service permissions. |
| Quarterly | `maint(process): review resource limits and accounting` | CPU/memory/process limits validated under load; failures and exhaustion behavior documented; inactive controllers are clearly labeled. |
| Quarterly | `docs(process): review POSIX/Linux/BSD compatibility claims` | Revalidate wait, process-group, service dependency, and management semantics; document explicit differences. |
| Each release candidate | `release(process): certify process-manager claims` | Exact architecture/QEMU configuration, boot-to-user-process test, service matrix, shutdown/recovery evidence, and known gaps. |
| After each incident | `fix(process): prevent recurrence of <failure>` | Root cause, regression test, affected targets, security/data-loss impact, and backport decision. |

These are recurring reviews, not automatic code changes. A task should produce a useful fix, test, documentation correction, or explicit “no change needed” record.

## Component-specific rules

- **Scheduler and context switching:** Keep runnable, blocked, stopped, and exiting state transitions synchronized with process management. Test wakeups, preemption, and exit races on the actual architecture path.
- **ELF loader and exec:** Validate headers, segment bounds, permissions, stack layout, and argument limits before replacing the current image. A failed exec must not leave a half-installed address space.
- **Wait and exit:** Preserve status until consumed once; define `ECHILD`, `EINTR`, and nonblocking behavior; handle multiple child exits without relying on one signal per child. POSIX defines child status collection and consumption semantics. ([POSIX wait Specification](https://pubs.opengroup.org/onlinepubs/9799919799/functions/wait.html))
- **Signals:** Keep process-directed and thread-directed delivery distinct; define stop, continue, terminate, and service-stop behavior. Do not treat a signal sent as proof the service has exited—wait for actual termination.
- **Init/PID 1:** Never block indefinitely on one service; continue reaping children; distinguish “process launched,” “service ready,” and “service healthy”; keep a recovery console path.
- **Process identity:** Do not rely on PID alone for asynchronous management if a stable object reference can be provided. Linux pidfds are specifically useful to avoid accidentally acting on a reused PID. ([Linux pidfd_open(2)](https://www.man7.org/linux/man-pages/man2/pidfd_open.2.html))
- **Service graph:** Validate dependencies before boot; reject cycles; distinguish ordering from requirement; make startup and shutdown ordering deterministic. FreeBSD’s rc system uses explicit dependency metadata rather than relying on script listing order. ([FreeBSD rc.d Documentation](https://docs-archive.freebsd.org/doc/11.0-RELEASE/usr/local/share/doc/freebsd/en/books/handbook/configtuning-rcd.html))
- **Restart and watchdog:** Restart only according to declared policy; add delay and rate limiting; expose the reason for failure; avoid restarting after an intentional manager stop. systemd documents restart modes, timeouts, watchdog behavior, and start-rate limiting as separate service controls. ([systemd.service Reference](https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html))
- **Resource controls:** Apply limits at process creation or an authoritative kernel control point; ensure descendants inherit policy; prevent a delegated service from escaping parent limits. Linux cgroup v2 describes hierarchical distribution and delegation boundaries. ([Linux cgroup v2 Guide](https://docs.kernel.org/admin-guide/cgroup-v2.html))
- **Observability:** Record service state transitions, exit codes, signal causes, restart counts, and start/stop durations. Bound logs and redact secrets.
- **Compatibility:** Keep native SigmaOS process APIs distinct from Linux emulation and POSIX libc behavior. Don’t claim Linux/BSD compatibility from similarly named modules.

## PR template and merge gates

```text
## Summary
What process-management or service-supervision gap does this PR address?

## Scope
Kernel lifecycle | syscall/libc | scheduler | init/service manager |
process inspection | resource control

## Semantics and compatibility
States, ownership, wait/exit behavior, service readiness, dependency and restart
policy, supported POSIX/Linux/BSD subset, and intentional deviations.

## Security and failure behavior
Authorization, PID identity/race behavior, executable validation, inherited
resources, limits, cleanup, and recovery.

## Runtime evidence
Unit/model | hosted | QEMU booted SigmaOS | physical target
List the exact architecture and test configuration.

## Tests
Lifecycle, child reaping, signals, failure injection, service dependencies,
restart loops, resource limits, and shutdown.

## Status and documentation
Proposed | Prototype | Integrated | Supported
Link updated feature inventory, service/process docs, and test evidence.

## Checklist
- [ ] Focused, reviewable change with acceptance criteria
- [ ] Real user-process behavior tested where relevant
- [ ] Child status and resources are collected/released correctly
- [ ] Service failure cannot create an unbounded restart loop
- [ ] Permission and resource-limit behavior tested
- [ ] Required CI passes; review conversations resolved
```

Require one maintainer approval for routine tests/docs and two qualified reviewers for process creation/exec, PID 1, signal/wait semantics, privilege boundaries, and resource enforcement. Protect the default branch with required PR reviews, status checks, and resolved conversations. GitHub supports these controls on protected branches. ([GitHub Protected Branches Guide](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches))

## Completion gates

Mark process management **Integrated** only when a booted SigmaOS target launches and executes a real user process, collects its exit status, and reaps it correctly. Mark it **Supported** only after repeatable tests cover failure to execute, child exit and adoption, signals, service dependencies/readiness, bounded restart, graceful shutdown, and the resource controls actually advertised. Linux’s testing guidance emphasizes that unit tests and userspace end-to-end tests serve different purposes; SigmaOS should report those evidence levels separately. ([SigmaOS Feature Inventory](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FEATURE_STATUS.toml))
