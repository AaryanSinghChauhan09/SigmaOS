# SigmaOS — Sovereign Rust Operating System

> **⚠️ DISCLAIMER & PROJECT WARNING**:
> SigmaOS is an experimental, sovereign Rust operating system workspace containing kernel, userspace, desktop, packaging, and tooling subsystems.
> **Unit-tested subsystems or passing component test suites DO NOT establish or prove a bootable, installable, or hardware-ready operating system.**
> Values in feature matrices track module-level logic test evidence; boot-to-shell, installer recovery, hardware validation, and full runtime path integration remain active priorities under development.

---

## Program Vision & Engineering Cadence

To convert isolated component logic and ambitious roadmap designs into auditable, verified, and reviewable pull requests, SigmaOS mandates a **cadence-based engineering program**.

| Period | Thread Manager / Scheduler Work | OS-Wide Maintenance Task | Required PR Output |
|---|---|---|---|
| **Daily** | Triage scheduler bugs, flaky tests, lock-order warnings, and regression reports. | Triage CI failures, build breakages, security advisories, and documentation drift. | Small bug-fix or triage PRs |
| **Weekly** | Implement one focused scheduler behavior: run queue, preemption, priority, affinity, or wait queue. | Dependency/toolchain updates, lint cleanup, test-suite repair and maintenance. | One scoped PR per change |
| **Biweekly** | Cross-subsystem integration test: threads with signals, FDs, IPC, timers, and process exit coordination. | Component status audit: update `FEATURE_STATUS.toml` and documentation with verified test evidence. | Status update + integration test evidence PR |
| **Monthly** | Scheduler benchmark, lock contention test, starvation audit, deadlock and fuzzing review. | Release-candidate build, ISO smoke test, installer & update recovery test. | Release-readiness PR |
| **Quarterly** | Scheduler architecture review against Linux/BSD paradigms; remove dead code and duplicated plans. | Roadmap consolidation, security review, API stability audit, contributor survey. | Architecture RFC + cleanup PRs |

---

## Thread Manager Subsystem Plan

The thread manager is defined as a formal subsystem with four explicit phase deliverables:

### Phase 1: Subsystem Contract Definition
- **Documentation**: See `docs/thread-manager.md` for the formal specification contract.
- **States**: `New`, `Ready`, `Running`, `Blocked`, `Suspended`, `Zombie`, `Terminated`.
- **Ownership**: `Kernel`, `User`, `Process`, `Detached`.
- **Guarantees**: Legal transition enforcement, saved `CpuContext`, `StackMeta` guard zones, priority ranges (0..=139), CPU affinity, wait-channel wakeup safety, and lock-order tracking.

### Phase 2: Core Thread Object Consolidation
- Single canonical `Thread` struct in `src/kernel/thread.rs`.
- `ThreadManager` wait-channel registration, wakeup, signal delivery, timeout, and reaping mechanics.

### Phase 3: Scheduler Integration Roadmap
1. **Run Queue**: Enqueue, dequeue, priority ordering, per-CPU queues, load balancing hooks.
2. **Context Switching**: Save/restore register state, stack switching, safe `Running`/`Ready` transitions.
3. **Preemption**: Timer-driven preemption, voluntary yield, priority inheritance, bounded latency.
4. **Blocking and Wakeup**: Sleep/wake channels, timeouts, lost-wakeup prevention.
5. **Exit and Reaping**: Thread exit, join/detach, zombie cleanup, stack reclamation, process exit coordination.
6. **Observability**: Thread statistics: state, CPU time, context switches, wait time, priority, and affinity.

### Phase 4: POSIX-Facing Behavior & Syscalls
- Thread creation, exit, join, detach syscalls.
- Mutex, condition variable, semaphore, reader-writer lock synchronization primitives.
- Thread-aware signal delivery, cancellation handlers, thread-specific data, and robust process termination.

---

## Prioritized Missing Components Matrix

| Component | Target Work / Missing Areas | PR Deliverable |
|---|---|---|
| **Boot Path** | End-to-end firmware-to-kernel-to-shell validation | `ci/boot-smoke` workflow plus boot log artifacts |
| **Thread Manager** | Canonical thread lifecycle, preemption, join/detach, wait channels | `feat(thread): canonical thread manager` |
| **Scheduler** | Deterministic run queues, preemption, fairness, starvation tests | `feat(sched): preemptive scheduler core` |
| **Process Lifecycle** | Fork/exec/wait/exit semantics and orphan/reparent behavior | `feat(process): POSIX-style lifecycle tests` |
| **Signals** | Per-thread masks, queued signals, signal-safe delivery | `feat(signal): thread-aware signal delivery` |
| **IPC** | Pipes, shared memory, message queues, eventfd consistency | `feat(ipc): unified IPC test matrix` |
| **VFS** | Mount namespace, path resolution, permission, crash consistency | `test(vfs): filesystem integration suite` |
| **Drivers** | Hardware test evidence and device support matrix | `docs(drivers): tested hardware matrix` |
| **Installer** | Failure recovery, rollback, disk validation, upgrade path | `feat(installer): recoverable install flow` |
| **Security** | Capability enforcement, sandbox policy, seccomp/Landlock restrictions | `feat(security): policy enforcement tests` |
| **Packaging** | Reproducible builds, SBOMs, signed updates, rollback | `feat(pkg): reproducible package pipeline` |
| **Observability** | Kernel logs, tracing, panic reports, crash dumps | `feat(observability): structured kernel tracing` |

---

## Suggested First 10 PR Sequence

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

---

## Verification & Standard Testing Commands

Before submitting any PR or marking a roadmap item complete, execute:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

For standalone subsystem unit verification (e.g. Thread Manager):

```bash
rustc --test src/kernel/thread.rs --edition=2021 -o /tmp/test_thread && /tmp/test_thread
```
