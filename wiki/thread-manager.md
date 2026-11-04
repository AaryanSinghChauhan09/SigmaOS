# SigmaOS Thread Manager Master Architecture, Support Matrix, and Program Plan

## Repository Context and Scope

The thread manager is a formal kernel subsystem responsible for thread lifecycle,
CPU context switching, stack management, priority and CPU affinity, wait-channel
blocking and wakeups, signal delivery masks, and join/detach synchronization.
It operates in close coordination with the CPU scheduler, process manager, and
interrupt handler while maintaining clear subsystem boundaries.

## Contract and Invariants

### Thread Lifecycle States

The canonical thread states are:

- `New`: The thread object has been allocated and initialized with stacks and
  registers, but has not yet been enqueued into a scheduler run queue.
- `Ready`: The thread is runnable and eligible for CPU execution in a scheduler
  run queue.
- `Running`: The thread is actively executing instructions on a CPU core.
- `Blocked`: The thread is descheduled waiting on a specific wait channel,
  timer, lock, or I/O event.
- `Suspended`: Execution is paused by an administrative control action or signal
  (e.g., job control or debugger attachment).
- `Zombie`: The thread has completed execution and exited, retaining its exit
  status code until reaped by its owning process or joiner.
- `Terminated`: All kernel stacks and execution resources have been reclaimed.

### Ownership Model

- **Kernel threads**: System threads running purely in kernel mode with no user
  address space.
- **User threads**: Application threads executing in user space with a dedicated
  user stack and kernel trap frame.
- **Process threads**: Threads tied to a specific parent process address space
  and file-descriptor table.
- **Detached threads**: Threads whose resources are reclaimed immediately upon
  exit without requiring an explicit join operation.

### Guarantees and Operational Invariants

- **Preemption model**: Preemptive time-slicing driven by timer ticks and
  voluntary yields via `sched_yield`.
- **Priority ranges**: Standard static priorities 0–139 (0–99 real-time,
  100–139 normal/fair-share), with dynamic priority adjustment and priority
  inheritance hooks to prevent priority inversion.
- **Affinity semantics**: Explicit 64-bit CPU affinity masks (`CpuAffinity`)
  controlling core placement.
- **Stack limits**: Guard pages placed at stack boundaries to detect and abort on
  stack overflow before memory corruption occurs.
- **Cancellation behavior**: Cancellation points evaluated at safe blocking
  boundaries; deferred cancellation executes cleanup handlers.
- **Wait-channel safety**: Lost-wakeup prevention using synchronized atomic state
  transitions and wait-channel registration before descheduling.
- **Non-goals**: No claim of complete POSIX compliance until the full syscall
  ABI, pthread compatibility suite, and libc-facing behavior are verified.

## Engineering Cadence

| Period | Thread manager / scheduler work | OS-wide maintenance | Required PR output |
|---|---|---|---|
| Daily | Triage scheduler bugs, flaky tests, lock-order warnings, and regression reports | Triage CI failures, build breakage, security advisories, and documentation drift | Bug reports or small fix PRs |
| Weekly | Implement one scheduler behavior: run queue, preemption, priority, affinity, or wait queue | Dependency/toolchain updates, lint cleanup, test-suite repair | One scoped PR per change |
| Biweekly | Integration test: threads with signals, FDs, IPC, timers, and process exit | Component status audit: update `FEATURE_STATUS.toml` and docs with evidence | Status + test-evidence PR |
| Monthly | Scheduler benchmark, contention test, starvation audit, deadlock/fuzz review | Release-candidate build, ISO smoke test, installer/update recovery test | Release-readiness PR |
| Quarterly | Scheduler architecture review against Linux/BSD patterns; remove dead code and duplicated plans | Roadmap consolidation, security review, API stability review, contributor survey | Architecture RFC + cleanup PRs |

## Development Phases

### Phase 1: Define the Contract
Define formal lifecycle states (`New`, `Ready`, `Running`, `Blocked`, `Suspended`,
`Zombie`, `Terminated`), ownership categories, stack limits, and machine-readable
status tracking in `FEATURE_STATUS.toml`.

### Phase 2: Core Thread Object
Consolidate a single canonical `Thread` abstraction containing:
- Thread ID (`ThreadId`) and owning Process ID (`Pid`).
- Saved CPU context (`CpuContext`).
- Kernel and user stack metadata (`StackMeta`) with guard zones.
- Priority, nice value, scheduling class, and CPU affinity mask (`CpuAffinity`).
- Wait channel (`WaitChannelId`), signal mask, and pending signals.
- File-descriptor table reference and resource limits.
- Exit status and join/detach tracking.
- Locks-held tracker for runtime lock-order verification.

### Phase 3: Scheduler Integration
Implement scheduler interactions in independently testable layers:
1. **Run queue**: Enqueue, dequeue, priority ordering, per-CPU queues.
2. **Context switching**: Save/restore CPU registers, stack switching, state transitions.
3. **Preemption**: Timer-driven preemption, voluntary yield, priority inheritance.
4. **Blocking and wakeup**: Sleep/wake channels, timeouts, lost-wakeup prevention.
5. **Exit and reaping**: Thread exit, join/detach synchronization, zombie cleanup.
6. **Observability**: Thread statistics: state, CPU time, context switches, wait time.

### Phase 4: POSIX-Facing Behavior
Expose thread behavior through stable syscalls:
- Thread creation (`clone`/`pthread_create`), exit, join, detach.
- Yield, sleep, nanosleep, and high-resolution timers.
- Synchronization primitives: mutex, condvar, semaphore, rwlock.
- Thread-directed signal routing and cancellation cleanup.
- CPU affinity and priority scheduling parameters.

## Missing Components Deliverable Matrix

| Component | Missing work | PR deliverable |
|---|---|---|
| Boot path | End-to-end firmware-to-kernel-to-shell validation | `ci/boot-smoke` workflow plus boot log artifacts |
| Thread manager | Canonical thread lifecycle, preemption, join/detach, and wait channels | `feat(thread): canonical thread manager` |
| Scheduler | Deterministic run queues, preemption, fairness, starvation tests | `feat(sched): preemptive scheduler core` |
| Process lifecycle | Fork/exec/wait/exit semantics and orphan/reparent behavior | `feat(process): POSIX-style lifecycle tests` |
| Signals | Per-thread masks, queued signals, signal-safe delivery | `feat(signal): thread-aware signal delivery` |
| IPC | Pipes, shared memory, message queues, eventfd consistency | `feat(ipc): unified IPC test matrix` |
| VFS | Mount namespace, path resolution, permission, and crash-consistency tests | `test(vfs): filesystem integration suite` |
| Drivers | Hardware test evidence and device support matrix | `docs(drivers): tested hardware matrix` |
| Installer | Failure recovery, rollback, disk validation, and upgrade path | `feat(installer): recoverable install flow` |
| Security | Capability enforcement, sandbox policy, seccomp/Landlock-style restrictions | `feat(security): policy enforcement tests` |
| Packaging | Reproducible builds, SBOMs, signed updates, rollback | `feat(pkg): reproducible package pipeline` |
| Observability | Kernel logs, tracing, panic reports, crash dumps | `feat(observability): structured kernel tracing` |

## First 10 Suggested PR Sequence

1. `docs(thread): define thread-manager contract and acceptance criteria`
2. `feat(thread): add canonical Thread object and lifecycle states`
3. `test(thread): add lifecycle, join, detach, and exit tests`
4. `feat(sched): add deterministic ready queue and scheduling-class hooks`
5. `feat(sched): add preemptive scheduling and context-switch test harness`
6. `fix(thread): add wait-channel wakeup and timeout race tests`
7. `feat(signal): route signals to the correct target thread`
8. `feat(proc): add process exit, orphan reparenting, and thread cleanup tests`
9. `ci: enforce fmt, clippy, unit tests, and standalone SigmaOS suites`
10. `docs(status): connect FEATURE_STATUS.toml to verified test evidence`

## Governance and Merge Gates

- Every PR must focus on a single subsystem boundary.
- All state transitions, lock acquisitions, and wait channels must have unit tests.
- Code changes require green CI: `cargo check --lib`, `cargo test --lib`, and `./run_sigma_tests.sh`.
- Status promotions in `FEATURE_STATUS.toml` must cite concrete test or boot evidence.
